use super::mk_table;
use linked_hash_map::LinkedHashMap;

use crate::types::*;
use std::fs::File;
use std::io::prelude::*;

#[allow(unused_imports)]
use crate::utils::StripMargin;

#[test]
fn can_yaml_to_md() {
    let yml_data = "
    |- data1: somevalue
    |  data2: someother value here
    |  col3: 100
    |  col4: gar gar
    |- data1: that
    |  data2: nice
    |  col3: 190x
    |- data1: this
    |  data2: someother value here
    |  col3: 100
    |  col4: ta da
    |"
    .strip_margin();

    // the | below is the margin
    let expected = "
    ||  data1  |       data2        |col3| col4  |
    ||---------|--------------------|----|-------|
    ||somevalue|someother value here|100 |gar gar|
    ||  that   |        nice        |190x|       |
    ||  this   |someother value here|100 | ta da |"
        .strip_margin();

    let tbl_md = mk_md_table_from_yaml(&yml_data, &None);
    assert!(tbl_md == expected);
}

#[test]
fn can_yaml_to_md_with_headings() {
    let yml_data = "
    |- data1: somevalue
    |  data2: someother value here
    |  col3: 100
    |  col4: gar gar
    |- data1: that
    |  data2: nice
    |  col3: 190x
    |- data1: this
    |  data2: someother value here
    |  col3: 100
    |  col4: ta da
    |"
    .strip_margin();
    let headings = vec![s!("data1"), s!("data2"), s!("col4")];

    // the | below is the margin
    let expected = "
    ||  data1  |       data2        | col4  |
    ||---------|--------------------|-------|
    ||somevalue|someother value here|gar gar|
    ||  that   |        nice        |       |
    ||  this   |someother value here| ta da |"
        .strip_margin();

    let render_options = RenderOptions {
        headings: Some(headings),
        ..Default::default()
    };

    let tbl_md = mk_md_table_from_yaml(&yml_data, &Some(render_options));

    println!("::expected\n{}\n\nreceived:{}", expected, tbl_md);
    assert!(tbl_md == expected);
}

fn load_yaml(yaml: &str) -> Table<String, String> {
    let deserialized_map: Table<String, String> = serde_yaml::from_str(&yaml).unwrap();
    deserialized_map
}

fn _load_json(json: &str) -> Table<String, String> {
    let deserialized_map: Table<String, String> = serde_json::from_str(&json).unwrap();
    deserialized_map
}

pub fn md_table_yaml_and_headings(headings: &str, yaml: &str) -> String {
    // we don't use an indexSet here for headings because the user may want repeats of the columns
    let render_options = RenderOptions {
        headings: Some(headings.split(',').map(String::from).collect::<Vec<_>>()),
        ..Default::default()
    };
    mk_table(&load_yaml(yaml), &Some(render_options))
}

/// Takes a String of YAML. An Array of Maps, 1 Level deep, and returns a Markdown Table
///
/// ```text
/// - data1: somevalue
///   data2: someother value here
///   col3: 100
///   col4: gar gar
/// - data1: that
///   data2: nice
///   col3: 190x
/// - data1: this
///   data2: someother value here
///   col3: 100
///   col4: ta da
/// ```
///
/// gives
///
/// ```text
/// |col3| col4  |  data1  |       data2        |
/// |----|-------|---------|--------------------|
/// |100 |gar gar|somevalue|someother value here|
/// |190x|       |  that   |        nice        |
/// |100 | ta da |  this   |someother value here|
/// ```
///
pub fn mk_md_table_from_yaml(yaml: &str, render_options: &Option<RenderOptions>) -> String {
    mk_table(&load_yaml(yaml), render_options)
}

/// Given results of tables, throw them back out as YAML
pub fn mk_yaml_from_table_result(
    tables: Vec<Result<NamedTable<String, String>, MadatoError>>,
) -> Result<String, MadatoError> {
    let table_map: LinkedHashMap<String, Table<String, String>> =
        tables.into_iter().filter_map(Result::ok).collect();

    // if we only have one table, strip off the key (get just the value)
    if table_map.len() == 1 {
        serde_yaml::to_string(&table_map.values().next()).map_err(|e| e.into())
    } else {
        serde_yaml::to_string(&table_map).map_err(|e| e.into())
    }
}

/// Given results of tables, throw them back out as JSON
pub fn mk_json_from_table_result(
    tables: Vec<Result<NamedTable<String, String>, MadatoError>>,
) -> Result<String, MadatoError> {
    let table_map: LinkedHashMap<String, Table<String, String>> =
        tables.into_iter().filter_map(Result::ok).collect();

    // if we only have one table, strip off the key (get just the value)
    if table_map.len() == 1 {
        serde_json::to_string_pretty(&table_map.values().next()).map_err(|e| e.into())
    } else {
        serde_json::to_string_pretty(&table_map).map_err(|e| e.into())
    }
}

pub fn yaml_file_to_md(
    filename: String,
    render_options: &Option<RenderOptions>,
) -> Result<String, MadatoError> {
    let mut file = File::open(filename)?; //.expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //.expect("Unable to read the file");

    Ok(mk_md_table_from_yaml(&contents, render_options))
}
