use madato::cal::get_sheet_names;
use madato::cal::spreadsheet_to_md;
use madato::cal::spreadsheet_to_named_table;
use madato::csv::csv_file_to_md;
use madato::csv::mk_csv_from_table_result;
use madato::types::KVFilter;
use madato::types::MadatoError;
use madato::types::RenderOptions;
use madato::yaml::mk_json_from_table_result;
use madato::yaml::mk_yaml_from_table_result;
use madato::yaml::yaml_file_to_md;

use docopt::Docopt;
use serde::Deserialize;

const USAGE: &str = "
madato utility - Tabular Data Helper

SpreadSheet <--> YAML <--> JSON <--> Markdown

Usage:
  madato table -t <type> [-s <sheetname>] [-o <outputtype>] [-f <filters>...] [-c <column>...] <filename>
  madato sheetlist <filename>
  madato (-h | --help)
  madato --version

Options:
  table                         Generate Makrdown or YAML tables from a Source (YAML, ODS, XLSX, CSV)
  sheetlist                     Read an Excel/ODS file and list out the names in the sheet.

  <filename>                    Input Filename.

  -t --type <type>              Input Type. XLSX(xls, xlsx, xlsm, xlsb, ods), YAML(table/row structure) or CSV
  -s --sheetname <sheetname>    When a Spreadsheet, restrict to just one of the sheets.
  -o --outputtype <outputtype>  JSON, MD (Markdown), CSV or YAML. [default: MD]
  -f --filters <filters>        Filter data in the results based on a simple, key=value
  -c --columns <column>         List of Columns to output 'only'
  -h --help                     Show this screen.
  --version                     Show version.

Quick examples

  madato table -t XLSX -o JSON workbook3.xlsx
  madato table -t XLSX -o MD   --sheetname Sheet2 someSheet_workbook.ods
  madato table -t XLSX -o YAML workbook3.xlsx
  madato table -t YAML -o MD   my_structured_data.yaml
  madato table -t XLSX -o YAML --filters 'Col1=Year.* Col[4-9]=.*' workbook3.xlsx
  madato table -t XLSX -o CSV test/sample_multi_sheet.xlsx
  madato table -t CSV -o MD test/potatoes.csv

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
    JSON,
    XLSX,
    CSV,
}

#[derive(Debug, Deserialize)]
enum OutputType {
    YAML,
    JSON,
    CSV,
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

#[cfg(feature = "cli")]
fn main() -> Result<(), MadatoError> {
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
            Some(FileType::JSON) => yaml_file_to_md(args.arg_filename, &render_options),
            Some(FileType::XLSX) => {
                spreadsheet_to_md(args.arg_filename, &render_options).map_err(|e| e.into())
            }
            Some(FileType::CSV) => csv_file_to_md(args.arg_filename, &render_options),
            None => panic!("No FileType specified"),
        },
        OutputType::YAML => mk_yaml_from_table_result(spreadsheet_to_named_table(
            args.arg_filename,
            args.flag_sheetname,
        )),
        OutputType::JSON => mk_json_from_table_result(spreadsheet_to_named_table(
            args.arg_filename,
            args.flag_sheetname,
        )),
        OutputType::CSV => mk_csv_from_table_result(spreadsheet_to_named_table(
            args.arg_filename,
            args.flag_sheetname,
        )),
    };

    // with output_string, print it out, or return the error
    match output_string {
        Ok(makrdown) => {
            println!("{}", makrdown);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
