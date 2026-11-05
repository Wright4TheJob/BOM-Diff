use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    pub id: String,
    pub description: String,
    pub quantity: i32,
}

impl Part {
    pub fn export_as_row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.description.clone(),
            self.quantity.to_string(),
        ]
    }
}
impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.id, self.description, self.quantity)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bom {
    pub parts: Vec<Part>,
}
impl Bom {
    pub fn consolidate(&mut self) {
        let mut parts = self.parts.clone();
        let mut consolidated_parts = Vec::new();
        for part in self.parts.clone() {
            let mut this_part = part.clone();
            if let Some(i) = matches_id(
                part,
                &Bom {
                    parts: parts.clone(),
                },
            ) {
                this_part.quantity += parts.get(i).unwrap().quantity;
                parts.remove(i);
            }
            if matches_id(
                this_part.clone(),
                &Bom {
                    parts: consolidated_parts.clone(),
                },
            )
            .is_none()
            {
                consolidated_parts.push(this_part);
            }
        }
        self.parts = consolidated_parts;
    }
    pub fn contains_part(&self, part: &Part) -> bool {
        for test_part in self.parts.clone() {
            if part_numbers_match(&test_part, part) {
                return true;
            }
        }
        false
    }
}
pub fn read_bom(file_path: &str, id_col: usize, desc_col: usize, quantity_col: usize) -> Bom {
    let parts = if file_path.ends_with(".csv") {
        read_csv_bom(file_path, id_col, desc_col, quantity_col, Some(b';'))
    } else if file_path.ends_with(".ods") {
        todo!("Other file formats");
    } else {
        todo!("Catch other formats")
    };
    Bom { parts }
}
#[test]
fn test_consolidate_bom() {
    let p1 = Part {
        id: "1".to_string(),
        description: "".to_string(),
        quantity: 2,
    };
    let p2 = Part {
        id: "2".to_string(),
        description: "".to_string(),
        quantity: 3,
    };
    let p3 = Part {
        id: "1".to_string(),
        description: "".to_string(),
        quantity: 2,
    };
    let mut parts = Bom {
        parts: vec![p1, p2, p3],
    };
    parts.consolidate();
    assert_eq!(4, parts.parts.get(0).unwrap().quantity);
    assert_eq!(2, parts.parts.len())
}
#[test]
fn test_match_id() {
    let p1 = Part {
        id: "1".to_string(),
        description: "".to_string(),
        quantity: 2,
    };
    let p2 = Part {
        id: "2".to_string(),
        description: "".to_string(),
        quantity: 3,
    };
    let p3 = Part {
        id: "1".to_string(),
        description: "".to_string(),
        quantity: 2,
    };
    assert_eq!(
        Some(1),
        matches_id(
            p1,
            &Bom {
                parts: vec![p2, p3]
            }
        )
    )
}
#[test]
fn test_not_match_id() {
    let p1 = Part {
        id: "1".to_string(),
        description: "".to_string(),
        quantity: 2,
    };
    let p2 = Part {
        id: "2".to_string(),
        description: "".to_string(),
        quantity: 3,
    };
    let p3 = Part {
        id: "3".to_string(),
        description: "".to_string(),
        quantity: 2,
    };
    assert_eq!(
        None,
        matches_id(
            p1,
            &Bom {
                parts: vec![p2, p3]
            }
        )
    )
}
pub fn matches_id(part: Part, parts: &Bom) -> Option<usize> {
    for (i, p) in parts.parts.iter().enumerate() {
        if p.id == part.id {
            return Some(i);
        }
    }
    None
}
fn read_csv_bom(
    file_path: &str,
    id_col: usize,
    desc_col: usize,
    quantity_col: usize,
    delimiter: Option<u8>,
) -> Vec<Part> {
    let file = File::open(file_path).unwrap();
    let mut rdr = match delimiter {
        Some(d) => csv::ReaderBuilder::new().delimiter(d).from_reader(file),
        None => csv::ReaderBuilder::new().delimiter(b',').from_reader(file),
    };
    rdr.records()
        .flatten()
        .map(|r| {
            let id = r.get(id_col).unwrap();
            let desc = r.get(desc_col).unwrap();
            let quantity = r.get(quantity_col).unwrap();
            Part {
                id: id.to_string(),
                description: desc.to_string(),
                quantity: quantity.parse().unwrap_or(0),
            }
        })
        .collect()
}
pub fn part_numbers_match(p1: &Part, p2: &Part) -> bool {
    string_partial_match(&p1.id, &p2.id)
}

pub fn string_partial_match(s1: &String, s2: &String) -> bool {
    s1 == s2 || s1.contains(s2) || s2.contains(s1)
}

pub fn get_part_by_num(parts: &Bom, id: String) -> Option<Part> {
    let parts_owned = parts.to_owned();
    let matches: Vec<Part> = parts_owned
        .parts
        .into_iter()
        .filter(|p| string_partial_match(&p.id, &id))
        .collect();
    match matches.len() {
        0 => None,
        1 => Some(matches[0].clone()),
        _ => {
            println!("More than one match found, returning first one");
            Some(matches[0].clone())
        }
    }
}

pub fn filter_removed_parts(old: &Bom, new: &Bom) -> Bom {
    let mut dropped = Vec::new();
    let mut old_reduced = old.clone();
    for old_part in old.parts.clone() {
        if !new.contains_part(&old_part) {
            dropped.push(old_part.clone());
            old_reduced = remove_part_by_id_match(old_reduced.clone(), &old_part);
        }
    }
    Bom { parts: dropped }
}

pub fn filter_added_parts(old: &Bom, new: &Bom) -> Bom {
    let mut added = Vec::new();
    let mut new_reduced = new.clone();
    for new_part in new.parts.clone() {
        if !old.contains_part(&new_part) {
            added.push(new_part.clone());
            new_reduced = remove_part_by_id_match(new_reduced, &new_part);
        }
    }
    Bom { parts: added }
}

pub fn remove_part_by_id_match(bom: Bom, part: &Part) -> Bom {
    Bom {
        parts: bom.parts.into_iter().filter(|p| p.id != part.id).collect(),
    }
}

pub fn filter_unchanged_parts(old: &Bom, new: &Bom) -> Bom {
    let mut unchanged = Vec::new();
    let mut old_reduced = old.clone();
    let mut new_reduced = new.clone();
    for old_part in old.parts.clone() {
        if let Some(new_part) = get_part_by_num(new, old_part.id.clone())
            && old_part.quantity == new_part.quantity
        {
            unchanged.push(new_part.clone());
            old_reduced = remove_part_by_id_match(old_reduced, &new_part);
            new_reduced = remove_part_by_id_match(new_reduced, &new_part);
        }
    }
    Bom { parts: unchanged }
}

pub fn filter_quantity_changed_parts(old: &Bom, new: &Bom) -> Bom {
    let mut changed = Vec::new();
    let mut old_reduced = old.clone();
    let mut new_reduced = new.clone();
    for old_part in old.parts.clone() {
        if let Some(part) = get_part_by_num(new, old_part.id.clone())
            && old_part.quantity != part.quantity
        {
            let mut changed_part = part.clone();
            changed_part.quantity = part.quantity - old_part.quantity;
            changed.push(changed_part);
            old_reduced = remove_part_by_id_match(old_reduced, &part);
            new_reduced = remove_part_by_id_match(new_reduced, &part);
        }
    }
    Bom { parts: changed }
}

pub fn compare_boms(old: &Bom, new: &Bom) -> (Bom, Bom, Bom, Bom) {
    let unchanged = filter_unchanged_parts(old, new);
    let changed = filter_quantity_changed_parts(old, new);
    let added = filter_added_parts(old, new);
    let removed = filter_removed_parts(old, new);
    (unchanged, changed, added, removed)
}
