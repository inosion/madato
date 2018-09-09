// #![feature(slice_patterns)]


extern crate calamine;
extern crate madato;

use calamine::{open_workbook_auto, DataType, Reader};

use madato::types::{ErroredTable, NamedTable, RenderOptions, TableRow};

///
/// Given a path to a Calamine supported Spreadsheet,
/// return a String or Error of that Spreadsheet
///
pub fn spreadsheet_to_md(
    filename: String,
    render_options: &Option<RenderOptions>,
) -> Result<String, String> {
    let results =
        read_excel_to_named_tables(filename, render_options.clone().and_then(|r| r.sheet_name));
    if results.len() <= 1 {
        Ok(madato::named_table_to_md(results[0].clone(), false, render_options))
    } else {
        Ok(results
            .iter()
            .map(|table_result| {
                madato::named_table_to_md(table_result.clone(), true, &render_options.clone())
            })
            .collect::<Vec<String>>()
            .join("\n\n"))
    }
}

pub fn read_excel_to_named_tables(
    filename: String,
    sheet_name: Option<String>,
) -> Vec<Result<NamedTable<String, String>, ErroredTable>> {
    // opens a new workbook
    let mut workbook = open_workbook_auto(filename).expect("Cannot open file");

    let sheet_names = if let Some(sheet_name) = sheet_name {
        workbook
            .sheet_names()
            .to_owned()
            .into_iter()
            .filter(|n| *n == sheet_name)
            .to_owned()
            .collect::<Vec<_>>()
    } else {
        workbook.sheet_names().to_owned()
    };

    let sheets: Vec<Result<NamedTable<String, String>, ErroredTable>> = sheet_names
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

///
/// Return a Vec<String> of Sheet Names
///
pub fn list_sheet_names(filename: String) -> Result<Vec<String>, String> {
    let workbook = open_workbook_auto(filename).expect("Could not open the file");
    Ok(workbook.sheet_names().to_owned())
}

pub fn md_santise(data: &DataType) -> String {
    data.to_string()
        .replace("|", "\\|")
        .replace("\r\n", "<br/>")
        .replace("\n", "<br/>")
        .replace("\r", "<br/>")
}

///
/// Use calamine to print sheet names
///
pub fn get_sheet_names(filename: String) {
    for s in list_sheet_names(filename).unwrap() {
        println!("{}", s);
    }
}
