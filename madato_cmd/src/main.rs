#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate madato;
extern crate madato_cal;

use docopt::Docopt;
use madato::types::*;
use madato::yaml::*;
use madato_cal::*;

const USAGE: &str = "
madato utility - Tabular Data Helper

SpreadSheet <--> YAML <--> JSON <--> Markdown 

Usage:
  madato table -t <type> [-s <sheetname>] [-o <outputtype>] [-f <filters>...] [-c <column>...] <filename>
  madato sheetlist <filename>
  madato (-h | --help)
  madato --version

Options:
  table                         Generate Makrdown or YAML tables from a Source.
  sheetlist                     Read an Excel/ODS file and list out the names in the sheet.

  <filename>                    Input Filename.

  -t --type <type>              Input Type.
  -s --sheetname <sheetname>    When a Spreadsheet, restrict to just one of the sheets.
  -o --outputtype <outputtype>  MD (Markdown) or YAML. [default: MD]
  -f --filters <filters>        Filter data in the results based on a simple, key=value
  -c --columns <column>         List of Columns to output 'only'
  -h --help                     Show this screen.
  --version                     Show version.

Filtering Example:

  Basic Filtering support occurs on a row by row basis where the key=value pair need to match.
  Both support a regular expression over the key and or the value.

  col[0-9]=val.*
  columnname=A[0-9]
  .*=[0-9] id=.*

  - Filtering will always occur, before the column limiters run.
  - Any = (equals) required in the filter, will need to be prefiltered with a \\ backslash.

Column Limit:
  Limit the Columns that are printed. (note, filtering occurs to ALL columns, before the output limit)

  - two colums '-c id -c amount'
  - multiple columns '-c col1 -c col2 -c col3'  
  - a column name can appear more than once eg: '-c col2 -c col2 -c col3'
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_table: bool,
    cmd_sheetlist: bool,
    arg_filename: String,

    flag_type: Option<FileType>,
    flag_sheetname: Option<String>,
    flag_outputtype: OutputType,
    flag_filters: Vec<String>,
    flag_columns: Vec<String>,
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

pub fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}

fn main() -> Result<(), String> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(version())).deserialize())
        .unwrap_or_else(|e| e.exit());

    // println!("args = {:?}", args);

    if args.cmd_sheetlist {
        get_sheet_names(args.arg_filename);
        return Ok(());
    };

    let headings = if args.flag_columns.len() > 0 {
        Some(args.flag_columns)
    } else {
        None
    };

    let filters: Vec<KVFilter> = args
        .flag_filters
        .iter()
        .map(|s| {
            static SR: &'static str = "!!_STR_REPLACE_!!";
            let kv = s
                .replace("\\=", SR)
                .split("=")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            KVFilter::new(kv[0].replace(SR, "\\="), kv[1].replace(SR, "\\="))
        })
        .collect();

    let render_options = Some(RenderOptions {
        headings: headings,
        sheet_name: args.flag_sheetname.clone(),
        filters: Some(filters),
    });

    let output_string = match args.flag_outputtype {
        OutputType::MD => match args.flag_type {
            Some(FileType::YAML) => yaml_file_to_md(args.arg_filename, &render_options),
            Some(FileType::XLSX) => spreadsheet_to_md(args.arg_filename, &render_options),
            _ => Err(String::from("not implemented")),
        },
        OutputType::YAML => {
            let tables = read_excel_to_named_tables(args.arg_filename, args.flag_sheetname);
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
