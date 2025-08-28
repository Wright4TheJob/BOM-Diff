mod bom;
use iced::widget::{Column, button, column, text_input};
use rfd::FileDialog;
use std::fs::File;
use std::path::Path;
use std::{error::Error, io, process};

#[derive(Default)]
struct State {
    bom1_path: String,
    bom2_path: String,
    output_path: String,
    output_filename: String,
}
#[derive(Debug, Clone)]
pub enum Message {
    PickBOM1,
    PickBOM2,
    PickOutputPath,
    ChangedFilename(String),
    Compare,
}

impl State {
    pub fn view(&self) -> Column<Message> {
        column![
            button("Pick Old BOM").on_press(Message::PickBOM1),
            button("Pick New BOM").on_press(Message::PickBOM2),
            button("Pick Output Path").on_press(Message::PickOutputPath),
            text_input("Type something here...", &self.output_filename)
                .on_input(Message::ChangedFilename),
            button("Compare BOMs").on_press(Message::Compare),
        ]
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::PickBOM1 => match self.get_file_path() {
                Some(path) => {
                    self.bom1_path = path;
                }
                None => (),
            },
            Message::PickBOM2 => match self.get_file_path() {
                Some(path) => {
                    self.bom2_path = path;
                }
                None => (),
            },
            Message::PickOutputPath => match self.get_file_path() {
                Some(path) => {
                    self.output_path = path;
                }
                None => (),
            },
            Message::ChangedFilename(filename) => {
                self.output_filename = filename;
            }
            Message::Compare => {
                let old = bom::read_bom(&self.bom1_path);
                let new = bom::read_bom(&self.bom2_path);
                let (unchanged, changed, added, removed) = bom::compare_boms(&old, &new);
            }
        }
    }
    pub fn get_file_path(&self) -> Option<String> {
        let file = FileDialog::new()
            .add_filter("csv", &["csv"])
            // .add_filter("spreadsheet", &["xls", "xlsx", "ods"])
            .set_directory("/")
            .pick_file();
        match file {
            Some(path) => Some(path.to_string_lossy().into_owned()),
            None => None,
        }
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
    iced::run("BOM Diff", State::update, State::view)
}
