#[derive(Debug, Clone)]
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

    pub fn to_string(&self) -> String {
        self.id.clone() + " - " + &self.description + " - " + &self.quantity.to_string()
    }
}

pub fn part_numbers_match(p1: &Part, p2: &Part) -> bool {
    string_partial_match(&p1.id, &p2.id)
}

pub fn string_partial_match(s1: &String, s2: &String) -> bool {
    s1 == s2 || s1.contains(s2) || s2.contains(s1)
}

pub fn get_part_by_num(parts: &Vec<Part>, id: String) -> Option<Part> {
    let parts_owned = parts.clone();
    let matches: Vec<Part> = parts_owned
        .into_iter()
        .filter(|p| string_partial_match(&p.id, &id))
        .collect();
    match matches.len() {
        0 => None,
        1 => Some(matches[0].clone()),
        _ => {
            println!("More than one match found, returning first one");
            return Some(matches[0].clone());
        }
    }
}

pub fn bom_contains_part(bom: &Vec<Part>, part: &Part) -> bool {
    for test_part in bom {
        if part_numbers_match(test_part, part) {
            return true;
        }
    }
    return false;
}

pub fn filter_dropped_parts(old: &Vec<Part>, new: &Vec<Part>) -> Vec<Vec<Part>> {
    let mut dropped = Vec::new();
    let mut old_reduced = old.clone();
    for old_part in old {
        if !bom_contains_part(new, old_part) {
            dropped.push(old_part.clone());
            old_reduced = remove_part_by_id_match(old_reduced.clone(), &old_part);
        }
    }
    vec![old_reduced, new.clone(), dropped]
}

pub fn filter_new_parts(old: &Vec<Part>, new: &Vec<Part>) -> Vec<Vec<Part>> {
    let mut added = Vec::new();
    let mut new_reduced = new.clone();
    for new_part in new {
        if !bom_contains_part(old, new_part) {
            added.push(new_part.clone());
            new_reduced = remove_part_by_id_match(new_reduced, &new_part);
        }
    }
    vec![old.clone(), new_reduced.clone(), added]
}

pub fn remove_part_by_id_match(parts: Vec<Part>, part: &Part) -> Vec<Part> {
    parts.into_iter().filter(|p| p.id != part.id).collect()
}

pub fn filter_unchanged_parts(old: &Vec<Part>, new: &Vec<Part>) -> Vec<Vec<Part>> {
    let mut unchanged = Vec::new();
    let mut old_reduced = old.clone();
    let mut new_reduced = new.clone();
    for old_part in old {
        if bom_contains_part(new, old_part) {
            let new_part = get_part_by_num(new, old_part.id.clone());
            match new_part {
                Some(part) => {
                    if old_part.quantity == part.quantity {
                        unchanged.push(part.clone());
                        old_reduced = remove_part_by_id_match(old_reduced, &part);
                        new_reduced = remove_part_by_id_match(new_reduced, &part);
                    }
                }
                None => (),
            }
        }
    }
    vec![old_reduced, new_reduced, unchanged]
}
