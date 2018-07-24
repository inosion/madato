use std::collections::BTreeMap;
use types::*;
use wasm_bindgen::prelude::*;

use super::{mk_table, mk_table_all_cols};

#[allow(unused_imports)]
use utils::StripMargin;

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
    ||col3| col4  |  data1  |       data2        |
    ||----|-------|---------|--------------------|
    ||100 |gar gar|somevalue|someother value here|
    ||190x|       |  that   |        nice        |
    ||100 | ta da |  this   |someother value here|"
        .strip_margin();

    let tbl_md = mk_md_table_from_yaml(&yml_data);
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

    let tbl_md = mk_md_table_from_yaml_with_headings(&headings, &yml_data);
    assert!(tbl_md == expected);
}

pub fn mk_md_table_from_yaml_with_headings(headings: &[String], yaml: &str) -> String {
    mk_table(&headings, &load_yaml(yaml))
}

fn load_yaml(yaml: &str) -> Table<String, String> {
    let deserialized_map: Vec<BTreeMap<String, String>> = serde_yaml::from_str(&yaml).unwrap();

    deserialized_map
        .iter()
        .map(|btree| {
            btree
                .iter()
                .map(|(x, y)| (x.clone(), y.clone()))
                .collect::<TableRow<String, String>>()
        })
        .collect::<Vec<_>>()
}

#[wasm_bindgen]
pub fn mk_md_table_from_yaml_with_headings_list(headings: &str, yaml: &str) -> String {
    mk_md_table_from_yaml_with_headings(
        &headings.split(',').map(String::from).collect::<Vec<_>>(),
        yaml,
    )
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
#[wasm_bindgen]
pub fn mk_md_table_from_yaml(yaml: &str) -> String {
    mk_table_all_cols(&load_yaml(yaml))
}
