#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate markdown_tools;

use docopt::Docopt;
use markdown_tools::excel::*;
use markdown_tools::yaml::*;
use std::fs::File;
use std::io::prelude::*;

const USAGE: &str = "
Markdown Tools

Usage:
  md-tools table -t <type> [-s <sheetname>] [-o <outputtype>] <filename> 
  md-tools sheetlist <filename>
  md-tools (-h | --help)
  md-tools --version

Options:

  table                         Generate Makrdown or YAML tables from a Source.
  sheetlist                     Read an Excel/ODS file and list out the names in the sheet.
  -h --help                     Show this screen.
  --version                     Show version.
  -t --type <type>              Input Type.
  <filename>                    Input Filename.
  -s --sheetname <sheetname>    When a Spreadsheet, restrict to just one of the sheets.
  -o --outputtype <outputtype>  MD (Markdown) or YAML. [default: MD]
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_table: bool,
    cmd_sheetlist: bool,
    arg_filename: String,

    flag_type: Option<FileType>,
    flag_sheetname: Option<String>,
    flag_outputtype: OutputType,
}

#[derive(Debug, Deserialize)]
enum FileType {
    YAML,
    XLSX,
    CSV,
}

#[derive(Debug, Deserialize)]
enum OutputType {
    YAML,
    MD,
}

fn read_yaml_file(filename: String) -> Result<String, String> {
    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    Ok(mk_md_table_from_yaml(&contents))
}

fn get_sheet_names(filename: String) {
  for s in list_sheet_names(filename).unwrap() {
    println!("{}",s);
  }
}

fn main() -> Result<(), String> {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_sheetlist {
      get_sheet_names(args.arg_filename);
      return Ok(());
    } 

    let output_string = match args.flag_outputtype {
        OutputType::MD => match args.flag_type {
            Some(FileType::YAML) => read_yaml_file(args.arg_filename),
            Some(FileType::XLSX) => spreadsheet_to_md(args.arg_filename, args.flag_sheetname),
            _ => Err(String::from("not implemented")),
        },
        OutputType::YAML => {
            let tables = read_excel(args.arg_filename, args.flag_sheetname);
            Ok(mk_yaml_from_table_result(tables))
        }
    };

    match output_string {
        Ok(markdown) => {
            println!("{}", markdown);
            Ok(())
        }
        Err(err) => Err(err),
    }
}
