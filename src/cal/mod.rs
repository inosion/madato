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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RenderOptions;

    #[test]
    fn test_spreadsheet_to_md_without_formulas() {
        let render_options = Some(RenderOptions {
            sheet_name: Some("second_sheet".to_string()),
            extract_formulas: false,
            show_formula_with_value: false,
            ..Default::default()
        });

        let result = spreadsheet_to_md("test/sample_multi_sheet.xlsx".to_string(), &render_options);
        assert!(result.is_ok());
        let md = result.unwrap();
        
        // Should contain cell values, not formulas
        assert!(md.contains(">> Multiline"));
        assert!(!md.contains("fx: #REF!"));
        assert!(!md.contains("fx: B1"));
    }

    #[test]
    fn test_spreadsheet_to_md_with_formulas_only() {
        let render_options = Some(RenderOptions {
            sheet_name: Some("second_sheet".to_string()),
            extract_formulas: true,
            show_formula_with_value: false,
            ..Default::default()
        });

        let result = spreadsheet_to_md("test/sample_multi_sheet.xlsx".to_string(), &render_options);
        assert!(result.is_ok());
        let md = result.unwrap();
        
        // Should contain formulas where they exist
        assert!(md.contains("#REF!") || md.contains("B1") || md.contains("0/0"));
    }

    #[test]
    fn test_spreadsheet_to_md_with_formula_and_value() {
        let render_options = Some(RenderOptions {
            sheet_name: Some("second_sheet".to_string()),
            extract_formulas: false,
            show_formula_with_value: true,
            ..Default::default()
        });

        let result = spreadsheet_to_md("test/sample_multi_sheet.xlsx".to_string(), &render_options);
        assert!(result.is_ok());
        let md = result.unwrap();
        
        // Should contain both value and formula in format VALUE<br/>fx: FORMULA
        assert!(md.contains("<br/>fx:"));
    }

    #[test]
    fn test_spreadsheet_to_named_table_with_formulas() {
        let render_options = Some(RenderOptions {
            sheet_name: Some("second_sheet".to_string()),
            extract_formulas: true,
            show_formula_with_value: false,
            ..Default::default()
        });

        let result = spreadsheet_to_named_table("test/sample_multi_sheet.xlsx".to_string(), &render_options);
        assert_eq!(result.len(), 1);
        
        let table_result = &result[0];
        assert!(table_result.is_ok());
        
        if let Ok((name, table)) = table_result {
            assert_eq!(name, "second_sheet");
            assert!(!table.is_empty());
            
            // Check that at least one cell contains a formula
            let has_formula = table.iter().any(|row| {
                row.values().any(|val| val.contains("B1") || val.contains("#REF!") || val.contains("0/0"))
            });
            assert!(has_formula, "Expected to find formulas in the table");
        }
    }

    #[test]
    fn test_list_sheet_names() {
        let result = list_sheet_names("test/sample_multi_sheet.xlsx".to_string());
        assert!(result.is_ok());
        
        let sheets = result.unwrap();
        assert_eq!(sheets.len(), 3);
        assert!(sheets.contains(&"Sheet1".to_string()));
        assert!(sheets.contains(&"second_sheet".to_string()));
        assert!(sheets.contains(&"3rd Sheet".to_string()));
    }

    #[test]
    fn test_md_santise() {
        assert_eq!(md_santise(&Data::String("test|pipe".to_string())), "test\\|pipe");
        assert_eq!(md_santise(&Data::String("line1\nline2".to_string())), "line1<br/>line2");
        assert_eq!(md_santise(&Data::String("line1\r\nline2".to_string())), "line1<br/>line2");
        assert_eq!(md_santise(&Data::Int(42)), "42");
        assert_eq!(md_santise(&Data::Float(3.14)), "3.14");
        assert_eq!(md_santise(&Data::Empty), "");
    }
}