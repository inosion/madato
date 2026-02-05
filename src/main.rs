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
use std::fs;

const USAGE: &str = "
madato utility - Tabular Data Helper

SpreadSheet <--> YAML <--> JSON <--> Markdown

Usage:
  madato table [-t <type>] [-s <sheetname>] [-o <outputtype>] [-f <filters>...] [-c <column>...] [--formulas] [--formula-with-value] <filename>
  madato sheetlist <filename>
  madato (-h | --help)
  madato --version

Options:
  table                         Generate Makrdown or YAML tables from a Source (YAML, ODS, XLSX, CSV)
  sheetlist                     Read an Excel/ODS file and list out the names in the sheet.

  <filename>                    Input Filename.

  -t --type <type>              Input Type (auto-detected from extension if not specified).
                                XLSX(xls, xlsx, xlsm, xlsb, ods), YAML(yaml, yml), JSON or CSV
  -s --sheetname <sheetname>    When a Spreadsheet, restrict to just one of the sheets.
  -o --outputtype <outputtype>  JSON, MD (Markdown), CSV or YAML. [default: MD]
  -f --filters <filters>        Filter data in the results based on a simple, key=value
  -c --columns <column>         List of Columns to output 'only'
  --formulas                    Extract formulas instead of cell values (spreadsheets only)
  --formula-with-value          Show both value and formula in format: VALUE<br/>fx: FORMULA
  -h --help                     Show this screen.
  --version                     Show version.

Quick examples

  madato table -o JSON workbook3.xlsx
  madato table --sheetname Sheet2 someSheet_workbook.ods
  madato table -o YAML workbook3.xlsx
  madato table my_structured_data.yaml
  madato table -o YAML --filters 'Col1=Year.* Col[4-9]=.*' workbook3.xlsx
  madato table -o CSV test/sample_multi_sheet.xlsx
  madato table test/potatoes.csv
  madato table --formulas workbook3.xlsx
  madato table --formula-with-value workbook3.xlsx
  madato table -t YAML -o MD my_data.txt    # Override auto-detection

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
    flag_formulas: bool,
    flag_formula_with_value: bool,
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

fn detect_file_type(filename: &str) -> Option<FileType> {
    // First try magic number detection
    if let Ok(mut file) = fs::File::open(filename) {
        let mut buffer = [0u8; 8192];
        if let Ok(n) = std::io::Read::read(&mut file, &mut buffer) {
            if n > 0 {
                let kind = infer::get(&buffer[..n]);

                // Check for spreadsheet formats
                if let Some(file_type) = kind {
                    match file_type.mime_type() {
                        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => return Some(FileType::XLSX),
                        "application/vnd.ms-excel" => return Some(FileType::XLSX),
                        "application/vnd.oasis.opendocument.spreadsheet" => return Some(FileType::XLSX),
                        "application/json" => return Some(FileType::JSON),
                        _ => {}
                    }
                }

                // Try to detect text-based formats (CSV, YAML, JSON)
                if let Ok(text) = std::str::from_utf8(&buffer[..n]) {
                    let trimmed = text.trim_start();
                    // JSON starts with { or [ - check this first before YAML
                    if trimmed.starts_with('{') || trimmed.starts_with('[') {
                        return Some(FileType::JSON);
                    }
                    // YAML typically starts with --- or has key: value pairs
                    if trimmed.starts_with("---") || trimmed.contains(":\\n") || trimmed.contains(": ") {
                        return Some(FileType::YAML);
                    }
                    // CSV detection: comma-separated values
                    if trimmed.lines().next().map_or(false, |line| line.contains(',')) {
                        return Some(FileType::CSV);
                    }
                }
            }
        }
    }

    // Fallback to extension-based detection
    let lower = filename.to_lowercase();
    if lower.ends_with(".csv") {
        Some(FileType::CSV)
    } else if lower.ends_with(".yaml") || lower.ends_with(".yml") {
        Some(FileType::YAML)
    } else if lower.ends_with(".json") {
        Some(FileType::JSON)
    } else if lower.ends_with(".xlsx")
        || lower.ends_with(".xls")
        || lower.ends_with(".xlsm")
        || lower.ends_with(".xlsb")
        || lower.ends_with(".ods") {
        Some(FileType::XLSX)
    } else {
        None
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
        extract_formulas: args.flag_formulas,
        show_formula_with_value: args.flag_formula_with_value,
    });

    // Auto-detect file type if not specified
    let file_type = args.flag_type.or_else(|| detect_file_type(&args.arg_filename))
        .expect("Unable to detect file type. Please specify with -t option.");

    let output_string = match args.flag_outputtype {
        OutputType::MD => match file_type {
            FileType::YAML => yaml_file_to_md(args.arg_filename, &render_options),
            FileType::JSON => yaml_file_to_md(args.arg_filename, &render_options),
            FileType::XLSX => {
                spreadsheet_to_md(args.arg_filename, &render_options).map_err(|e| e.into())
            }
            FileType::CSV => csv_file_to_md(args.arg_filename, &render_options),
        },
        OutputType::YAML => mk_yaml_from_table_result(spreadsheet_to_named_table(
            args.arg_filename,
            &render_options,
        )),
        OutputType::JSON => mk_json_from_table_result(spreadsheet_to_named_table(
            args.arg_filename,
            &render_options,
        )),
        OutputType::CSV => mk_csv_from_table_result(spreadsheet_to_named_table(
            args.arg_filename,
            &render_options,
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
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_detect_xlsx_by_magic_number() {
        let result = detect_file_type("test/sample_multi_sheet.xlsx");
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), FileType::XLSX));
    }

    #[test]
    fn test_detect_csv_by_content() {
        let result = detect_file_type("test/potatoes.csv");
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), FileType::CSV));
    }

    #[test]
    fn test_detect_yaml_by_extension() {
        let result = detect_file_type("test/test.yml");
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), FileType::YAML));
    }

    #[test]
    fn test_detect_xlsx_renamed_file() {
        // Create a temporary file with wrong extension but XLSX content
        let mut temp_file = NamedTempFile::new().unwrap();
        let xlsx_content = std::fs::read("test/sample_multi_sheet.xlsx").unwrap();
        temp_file.write_all(&xlsx_content).unwrap();
        temp_file.flush().unwrap();

        let result = detect_file_type(temp_file.path().to_str().unwrap());
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), FileType::XLSX), "Should detect XLSX by magic number even without extension");
    }

    #[test]
    fn test_detect_csv_by_extension() {
        let mut temp_file = NamedTempFile::with_suffix(".csv").unwrap();
        temp_file.write_all(b"col1,col2,col3\n1,2,3\n4,5,6").unwrap();
        temp_file.flush().unwrap();

        let result = detect_file_type(temp_file.path().to_str().unwrap());
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), FileType::CSV));
    }

    #[test]
    fn test_detect_yaml_by_content() {
        let mut temp_file = NamedTempFile::with_suffix(".txt").unwrap();
        temp_file.write_all(b"---\n- key: value\n  other: data").unwrap();
        temp_file.flush().unwrap();

        let result = detect_file_type(temp_file.path().to_str().unwrap());
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), FileType::YAML), "Should detect YAML by content pattern");
    }

    #[test]
    fn test_detect_json_by_content() {
        let mut temp_file = NamedTempFile::with_suffix(".txt").unwrap();
        temp_file.write_all(b"{\"key\": \"value\"}").unwrap();
        temp_file.flush().unwrap();

        let result = detect_file_type(temp_file.path().to_str().unwrap());
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), FileType::JSON), "Should detect JSON by content pattern");
    }

    #[test]
    fn test_detect_unknown_file() {
        let mut temp_file = NamedTempFile::with_suffix(".unknown").unwrap();
        temp_file.write_all(b"random binary data \x00\x01\x02").unwrap();
        temp_file.flush().unwrap();

        let result = detect_file_type(temp_file.path().to_str().unwrap());
        assert!(result.is_none(), "Should return None for unrecognized file types");
    }

    #[test]
    fn test_version() {
        let ver = version();
        assert!(!ver.is_empty() || ver == "");
    }
}