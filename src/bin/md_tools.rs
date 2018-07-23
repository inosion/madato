#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate markdown_tools;

use docopt::Docopt;
use markdown_tools::excel::*;
use markdown_tools::*;
use std::fs::File;
use std::io::prelude::*;

const USAGE: &'static str = "
Markdown Tools

Usage:
  md-tools table -t <type> <filename>
  md-tools (-h | --help)
  md-tools --version

Options:
  -h --help             Show this screen.
  --version             Show version.
  -t --type <type>      Input Type.
  <filename>            Input Filename.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_table: bool,
    flag_type: FileType,
    arg_filename: String,
}

#[derive(Debug, Deserialize)]
enum FileType {
    YAML,
    XLSX,
    CSV,
}

fn read_yaml_file(filename: String) -> Result<String, String> {
    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    Ok(mk_md_table_from_yaml(contents))
}

fn main() -> Result<(), String> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let result = match args.flag_type {
        FileType::YAML => read_yaml_file(args.arg_filename),
        FileType::XLSX => spreadsheet_to_md(args.arg_filename),
        _ => Err(String::from("not implemented")),
    };

    match result {
        Ok(markdown) => {
            println!("{}", markdown);
            Ok(())
        }
        Err(err) => Err(err),
    }
}
