use calamine::{open_workbook_auto, DataType, Range, Reader, Sheets, Xlsx};

use mk_table_all_cols;
use std::ops::Add;
use types::{Headers, MultiTables, Table, TableRow};

/*
pub fn read_excel<'a >(filename: String) -> Result<MultiTables<&'a str, &'a str>, String> {

    let tables = MultiTables::new())

}
*/
fn extract_headers_from_sheet(workbook: &mut Sheets, sheet_name: &str) -> Result<Headers, String> {
    //Ok(vec!["f","g","h"].into_iter().map(String::from).collect());

    match workbook.worksheet_range(sheet_name) {
        None => Err(format!("sheet {} is empty", sheet_name)),
        Some(Err(err)) => Err(String::from("oops 009")),
        Some(Ok(sheet)) => Ok(sheet
            .rows()
            .next()
            .expect("No Header Row found")
            .iter()
            .enumerate()
            .map(|(i, c)| match c {
                DataType::Empty => format!("NULL{}", i),
                _ => format!("{}", c).replace(",", "\\,"),
            })
            .collect()),
    }
}

pub fn spreadsheet_to_md(filename: String) -> Result<String, String> {
    Ok(read_excel(filename)
        .iter()
        .fold(String::from(""), |mut i, j| {
            match j {
                Err(err) => i.push_str("error"),
                Ok(table) => i.push_str(mk_table_all_cols(table).as_str()),
            }
            i
        }))
}

fn extract_headers(range: &Range<DataType>) -> Result<Headers, String> {
    Ok(range
        .rows()
        .next()
        .expect("No Header Row found")
        .iter()
        .enumerate()
        .map(|(i, c)| match c {
            DataType::Empty => format!("NULL{}", i),
            _ => format!("{}", c).replace(",", "\\,"),
        })
        .collect())
}

pub fn read_excel<'a>(filename: String) -> Vec<Result<Table<String, String>, String>> {
    // opens a new workbook
    let mut workbook = open_workbook_auto(filename).expect("Cannot open file");

    // Read whole worksheet data and provide some statistics

    let sheet_names = workbook.sheet_names().to_owned();

    let iter = sheet_names.iter();

    let sheets: Vec<Result<Table<String, String>, String>> = iter
        .map(|name| {
            let maybe_sheet = workbook.worksheet_range(name);
            match maybe_sheet {
                None => Err(format!("sheet {} is empty", name)),
                Some(Err(err)) => Err(format!("{}", err)),
                Some(Ok(sheet)) => Ok({
                    let first_row: Vec<(usize, &DataType)> = sheet
                        .rows()
                        .next()
                        .expect("Missing data in the sheet")
                        .iter()
                        .enumerate()
                        .collect();

                    sheet
                        .rows()
                        .skip(1)
                        .map(|row| {
                            first_row
                                .iter()
                                .map(|(i, col)| {
                                    //(col, format!("{}", col).as_str().to_owned(), format!("{}", row[*i]).as_str().to_owned())
                                    ((**col).to_string(), row[*i].to_string())
                                })
                                .collect::<TableRow<String, String>>()
                        })
                        .collect::<Vec<_>>()
                }),
            }
        })
        .collect::<Vec<_>>();

    sheets
}

pub fn read_excel2<'a>(filename: String) -> Result<MultiTables<&'a str, &'a str>, String> {
    // opens a new workbook
    let mut workbook = open_workbook_auto(filename).expect("Cannot open file");

    // Read whole worksheet data and provide some statistics

    let sheet_names = workbook.sheet_names().to_owned();

    for sheet_name in sheet_names {
        println!("::{}", sheet_name);
    }

    println!(
        "first sheet, first row {:#?}",
        extract_headers_from_sheet(&mut workbook, "Sheet1")
    );

    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.used_cells().count();
        println!(
            "Found {} cells in 'Sheet1', including {} non empty cells",
            total_cells, non_empty_cells
        );
        // alternatively, we can manually filter rows
        assert_eq!(
            non_empty_cells,
            range
                .rows()
                .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
                .count()
        );
    }

    // Check if the workbook has a vba project
    if let Some(Ok(mut vba)) = workbook.vba_project() {
        let vba = vba.to_mut();
        let module1 = vba.get_module("Module 1").unwrap();
        println!("Module 1 code:");
        println!("{}", module1);
        for r in vba.get_references() {
            if r.is_missing() {
                println!("Reference {} is broken or not accessible", r.name);
            }
        }
    }

    // You can also get defined names definition (string representation only)
    for name in workbook.defined_names() {
        println!("name: {}, formula: {}", name.0, name.1);
    }

    // Now get all formula!
    let sheets = workbook.sheet_names().to_owned();
    for s in sheets {
        println!(
            "found {} formula in '{}'",
            workbook
                .worksheet_formula(&s)
                .expect("sheet not found")
                .expect("error while getting formula")
                .rows()
                .flat_map(|r| r.iter().filter(|f| !f.is_empty()))
                .count(),
            s
        );
    }

    Ok(MultiTables::new())
}
