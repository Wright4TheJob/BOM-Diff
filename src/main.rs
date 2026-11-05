mod bom;
use iced::widget::{Column, button, checkbox, column, row, text, text_input};
use rfd::FileDialog;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Default)]
struct State {
    bom1_path: String,
    bom2_path: String,
    output_path: String,
    output_filename: String,
    has_headers: bool,
}
#[derive(Debug, Clone)]
pub enum Message {
    PickBOM1,
    PickBOM2,
    PickOutputPath,
    ChangedFilename(String),
    Compare,
    HasHeaders(bool),
}

impl State {
    pub fn view(&self) -> Column<'_, Message> {
        let gap = 5;
        let space = 5;
        column![
            row![
                button("Pick Old BOM").on_press(Message::PickBOM1),
                text(format!("Old BOM: {}", self.bom1_path))
            ]
            .padding(gap)
            .spacing(space),
            row![
                button("Pick New BOM").on_press(Message::PickBOM2),
                text(format!("New BOM: {}", self.bom2_path))
            ]
            .padding(gap)
            .spacing(space),
            row![button("Pick Output Path").on_press(Message::PickOutputPath)]
                .padding(5)
                .spacing(space),
            row![
                text_input("Output_BOM.csv", &self.output_filename)
                    .on_input(Message::ChangedFilename)
            ]
            .padding(gap)
            .spacing(space),
            row![
                checkbox(self.has_headers)
                    .label("Data has headers")
                    .on_toggle(Message::HasHeaders)
            ]
            .padding(gap)
            .spacing(space),
            row![button("Compare BOMs").on_press(Message::Compare)]
                .padding(gap)
                .spacing(space),
        ]
        .padding(gap)
        .spacing(space)
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::PickBOM1 => {
                if let Some(path) = self.get_file_path() {
                    self.bom1_path = path;
                }
            }
            Message::PickBOM2 => {
                if let Some(path) = self.get_file_path() {
                    self.bom2_path = path;
                }
            }
            Message::PickOutputPath => {
                if let Some(path) = self.get_dir_path() {
                    self.output_path = path;
                }
            }
            Message::ChangedFilename(filename) => {
                self.output_filename = filename;
            }
            Message::Compare => {
                let old = bom::read_bom(&self.bom1_path);
                let new = bom::read_bom(&self.bom2_path);
                let (unchanged, changed, added, removed) = bom::compare_boms(&old, &new);
                println!("Unchanged:");
                unchanged.iter().for_each(|p| println!("{p}"));
                println!("Changed Quantity:");
                changed.iter().for_each(|p| println!("{p}"));
                println!("Added:");
                added.iter().for_each(|p| println!("{p}"));
                println!("Removed:");
                removed.iter().for_each(|p| println!("{p}"));
            }
            Message::HasHeaders(headers) => self.has_headers = headers,
        }
    }
    pub fn get_file_path(&self) -> Option<String> {
        let file = FileDialog::new()
            .add_filter("csv", &["csv"])
            // .add_filter("spreadsheet", &["xls", "xlsx", "ods"])
            .set_directory("~")
            .pick_file();
        file.map(|path| path.to_string_lossy().into_owned())
        // match file {
        // Some(path) => Some(path.to_string_lossy().into_owned()),
        // None => None,
        // }
    }
    pub fn get_dir_path(&self) -> Option<String> {
        let file = FileDialog::new().set_directory("~").pick_folder();
        file.map(|path| path.to_string_lossy().into_owned())
    }
}

pub fn read_csv<P: AsRef<Path>>(filename: P) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
        let mut line = Vec::new();
        for field in &record {
            line.push(field.to_string());
            println!("{:?}", field);
        }

        data.push(line);
    }
    Ok(data)
}

fn main() -> iced::Result {
    iced::run(State::update, State::view)
}
