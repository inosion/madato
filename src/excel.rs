#![feature(slice_patterns)]

use calamine::{open_workbook_auto, DataType, Reader};

use mk_table_all_cols;
use types::{Table, TableRow};

fn table_formatter(
    table: Result<(String, Table<String, String>), (String, String)>,
    print_name: bool,
) -> String {
    match table {
        Err((name, error)) => format!("Sheet `{}` errored: {}", name, error),
        Ok((name, table_data)) => {
            if print_name {
                format!("**{}**\n{}", name, mk_table_all_cols(&table_data))
            } else {
                mk_table_all_cols(&table_data)
            }
        }
    }
}

pub fn spreadsheet_to_md(filename: String, sheet_name: Option<String>) -> Result<String, String> {
    let results = read_excel(filename, sheet_name);
    if results.len() <= 1 {
        Ok(table_formatter(results[0].clone(), false))
    } else {
        Ok(results
            .iter()
            .map(|table_result| table_formatter(table_result.clone(), true))
            .collect::<Vec<String>>()
            .join("\n\n"))
    }
}

pub fn read_excel(
    filename: String,
    sheet_name: Option<String>,
) -> Vec<Result<(String, Table<String, String>), (String, String)>> {
    // opens a new workbook
    let mut workbook = open_workbook_auto(filename).expect("Cannot open file");

    let sheet_names = if let Some(sheet_name) = sheet_name {
        workbook.sheet_names().to_owned().into_iter().filter(|n| *n == sheet_name).to_owned().collect::<Vec<_>>()
    } else {
        workbook.sheet_names().to_owned()
    };

    let sheets: Vec<Result<(String, Table<String, String>), (String, String)>> = sheet_names
        .iter()
        .map(|name| {
            let maybe_sheet = workbook.worksheet_range(name);
            match maybe_sheet {
                None => Err((name.clone(), format!("sheet {} is empty", name))),
                Some(Err(err)) => Err((name.clone(), format!("{}", err))),
                Some(Ok(sheet)) => Ok((name.clone(), {
                    let first_row: Vec<(usize, String)> = sheet
                        .rows()
                        .next()
                        .expect("Missing data in the sheet")
                        .iter()
                        .enumerate()
                        .map(|(i, c)| match c {
                            DataType::Empty => (i, format!("NULL{}", i)),
                            _ => (i, c.to_string()),
                        })
                        .collect();

                    sheet
                        .rows()
                        .skip(1)
                        .map(|row| {
                            first_row
                                .iter()
                                .map(|(i, col)| ((**col).to_string(), md_santise(&row[*i])))
                                .collect::<TableRow<String, String>>()
                        })
                        .collect::<Vec<_>>()
                })),
            }
        })
        .collect::<Vec<_>>();

    sheets
}

fn md_santise(data: &DataType) -> String {
    data.to_string()
        .replace("|", "\\|")
        .replace("\r\n", "<br/>")
        .replace("\n", "<br/>")
        .replace("\r", "<br/>")
}
