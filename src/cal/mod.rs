// #![feature(slice_patterns)]

pub mod error;

use calamine::{open_workbook_auto, Data, Reader};

use crate::named_table_to_md;
use crate::types::{MadatoError, NamedTable, RenderOptions, TableRow};

///
/// Given a path to a Calamine supported Spreadsheet,
/// return a String or Error of that Spreadsheet
///
pub fn spreadsheet_to_md(
    filename: String,
    render_options: &Option<RenderOptions>,
) -> Result<String, error::MadatoCalError> {
    let results = spreadsheet_to_named_table_internal(
        filename,
        render_options.clone().and_then(|r| r.sheet_name),
        render_options,
    );
    if results.len() <= 1 {
        Ok(named_table_to_md(
            &results[0].clone().map_err(MadatoError::from),
            false,
            render_options,
        ))
    } else {
        Ok(results
            .iter()
            .map(|table_result| {
                named_table_to_md(
                    &table_result.clone().map_err(MadatoError::from),
                    true,
                    &render_options.clone(),
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n"))
    }
}

pub fn spreadsheet_to_named_table(
    filename: String,
    render_options: &Option<RenderOptions>,
) -> Vec<Result<NamedTable<String, String>, MadatoError>> {
    let sheet_name = render_options.clone().and_then(|r| r.sheet_name);
    let tables = spreadsheet_to_named_table_internal(filename, sheet_name, render_options);
    tables
        .into_iter()
        .map(|res| res.map_err(MadatoError::from))
        .collect()
}

fn spreadsheet_to_named_table_internal(
    filename: String,
    sheet_name: Option<String>,
    render_options: &Option<RenderOptions>,
) -> Vec<Result<NamedTable<String, String>, error::MadatoCalError>> {
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

    let sheets: Vec<Result<NamedTable<String, String>, error::MadatoCalError>> = sheet_names
        .iter()
        .map(|name| {
            let sheet = workbook
                .worksheet_range(name)
                .map_err(error::MadatoCalError::from)?;

            // Extract formulas if requested
            let formulas = if render_options.as_ref().map_or(false, |opts| {
                opts.extract_formulas || opts.show_formula_with_value
            }) {
                workbook.worksheet_formula(name).ok()
            } else {
                None
            };

            Ok((name.clone(), {
                let headers = extract_header_row(&sheet)?;

                sheet
                    .rows()
                    .skip(1)
                    .enumerate()
                    .map(|(row_idx, row)| {
                        headers
                            .iter()
                            .map(|(i, col)| {
                                let value = md_santise(&row[*i]);
                                let cell_value = if let Some(ref formula_range) = formulas {
                                    let formula = formula_range
                                        .get((row_idx + 1, *i))
                                        .unwrap_or(&String::new())
                                        .to_string();

                                    if !formula.is_empty() {
                                        if render_options
                                            .as_ref()
                                            .map_or(false, |opts| opts.show_formula_with_value)
                                        {
                                            format!("{}<br/>fx: {}", value, formula)
                                        } else if render_options
                                            .as_ref()
                                            .map_or(false, |opts| opts.extract_formulas)
                                        {
                                            formula
                                        } else {
                                            value
                                        }
                                    } else {
                                        value
                                    }
                                } else {
                                    value
                                };

                                ((**col).to_string(), cell_value)
                            })
                            .collect::<TableRow<String, String>>()
                    })
                    .collect::<Vec<_>>()
            }))
        })
        .collect::<Vec<_>>();

    sheets
}

///
/// Internal fn, extract the header row from a sheet
/// If a cell in the row is empty, it will be replaced with NULL0, NULL1, etc.
///
fn extract_header_row(
    sheet: &calamine::Range<Data>,
) -> Result<Vec<(usize, String)>, error::MadatoCalError> {
    let first_row: Vec<(usize, String)> = sheet
        .rows()
        .next()
        .ok_or(error::MadatoCalError::MissingDataInSheet())?
        .iter()
        .enumerate()
        .map(|(i, c)| match c {
            Data::Empty => (i, format!("NULL{}", i)),
            _ => (i, c.to_string()),
        })
        .collect();
    Ok(first_row)
}

///
/// Return a Vec<String> of Sheet Names
///
pub fn list_sheet_names(filename: String) -> Result<Vec<String>, error::MadatoCalError> {
    let workbook = open_workbook_auto(filename)?;
    Ok(workbook.sheet_names().to_owned())
}

pub fn md_santise(data: &Data) -> String {
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
