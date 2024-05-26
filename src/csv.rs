use super::mk_table;
use crate::types::*;

use linked_hash_map::LinkedHashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn load_csv(csv: &str) -> Result<Table<String, String>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let headers = rdr.headers()?.clone();
    let mut table: Table<String, String> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let mut row: TableRow<String, String> = LinkedHashMap::new();

        for (header, field) in headers.iter().zip(record.iter()) {
            row.insert(header.into(), field.into());
        }

        table.push(row);
    }

    Ok(table)
}

pub fn mk_md_table_from_csv(csv: &str, render_options: &Option<RenderOptions>) -> String {
    mk_table(
        &load_csv(csv).expect("Failed to load the CSV"),
        render_options,
    )
}

/// Given results of tables, throw them back out as csv
pub fn mk_csv_from_table_result(
    tables: Vec<Result<NamedTable<String, String>, MadatoError>>,
) -> Result<String, MadatoError> {
    let table_map: LinkedHashMap<String, Table<String, String>> =
        tables.into_iter().filter_map(Result::ok).collect();

    // if we only have one table, strip off the key (get just the value)
    if table_map.len() == 1 {
        let (_, table) = table_map.into_iter().next().unwrap();
        let mut wtr = csv::Writer::from_writer(vec![]);
        if let Some(first_row) = table.iter().next() {
            wtr.write_record(first_row.keys())?;
            // .expect("Failed to serialize headers");
        }
        for row in table {
            wtr.write_record(row.values())?;
            // .expect("Failed to serialize row");
        }
        let r = wtr
            .into_inner()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        String::from_utf8(r).map_err(|e| e.into())
    } else {
        let mut csvs = Vec::new();
        for (_, table) in table_map {
            let mut wtr = csv::Writer::from_writer(vec![]);
            if let Some(first_row) = table.iter().next() {
                wtr.write_record(first_row.keys())?;
                // .expect("Failed to serialize headers");
            }
            for row in table {
                wtr.write_record(row.values())?;
                // .expect("Failed to serialize row");
            }
            csvs.push(
                String::from_utf8(wtr.into_inner().unwrap())
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?, //.expect("Failed to write CSV"))
                                                                                      //.expect("Failed to convert CSV to string"),
            );
        }
        Ok(csvs.join("\n"))
    }
}

pub fn csv_file_to_md(
    filename: String,
    render_options: &Option<RenderOptions>,
) -> Result<String, MadatoError> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(mk_md_table_from_csv(&contents, render_options))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_csv() {
        let csv = "header1,header2\nvalue1,value2";
        let table = load_csv(csv).unwrap();
        assert_eq!(table.len(), 1);
        let row = &table[0];
        assert_eq!(row.get("header1"), Some(&"value1".to_string()));
        assert_eq!(row.get("header2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_mk_md_table_from_csv() {
        let csv = "header1,header2\nvalue1,value2";
        let md_table = mk_md_table_from_csv(csv, &None);
        assert!(md_table.contains("|header1|header2|"));
        assert!(md_table.contains("|value1 |value2 |"));
    }

    #[test]
    fn test_mk_csv_from_table_result() {
        let table: NamedTable<String, String> = (
            "table1".to_string(),
            vec![[
                ("header1".to_string(), "value1".to_string()),
                ("header2".to_string(), "value2".to_string()),
            ]
            .iter()
            .cloned()
            .collect()],
        );
        let tables = vec![Ok(table)];
        let csv = mk_csv_from_table_result(tables).unwrap();
        assert!(csv.contains("header1,header2"));
        assert!(csv.contains("value1,value2"));
    }

    #[test]
    fn test_csv_file_to_md() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let csv = "header1,header2\nvalue1,value2";
        let mut tmp_file = NamedTempFile::new().unwrap();
        write!(tmp_file, "{}", csv).unwrap();

        let md_table =
            csv_file_to_md(tmp_file.path().to_str().unwrap().to_string(), &None).unwrap();
        assert!(md_table.contains("|header1|header2|"));
        assert!(md_table.contains("|value1 |value2 |"));
    }
}
