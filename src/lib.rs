#![feature(
    use_extern_macros, wasm_custom_section, wasm_import_module, iterator_flatten, slice_patterns
)]

extern crate calamine;
extern crate indexmap;
extern crate serde_yaml;
extern crate wasm_bindgen;
extern crate linked_hash_map;

pub mod excel;
pub mod types;

#[macro_use]
mod utils;

use indexmap::IndexSet;
use std::cmp;
use types::*;
use wasm_bindgen::prelude::*;
use std::collections::BTreeMap;

#[allow(unused_imports)]
use utils::StripMargin;

#[test]
fn can_extract_headers() {
    let hdrs = vec![
        linkedhashmap![s!("foo") => s!("ggg"), s!("bar") => s!("fred"), s!("nop") => s!("no")], // foo bar nop
        linkedhashmap![s!("foo") => s!("seventy"), s!("bar") => s!("barry"), s!("nop") => s!("no"), s!("aaa") => s!("ddd")], //
        linkedhashmap![s!("bar") => s!("col has no foo"), s!("fff") => s!("ffsd")],
    ];

    let expected = indexset![s!("bar"), s!("foo"), s!("nop"), s!("aaa"), s!("fff")];
    let result = collect_headers(&hdrs);
    assert!(expected == result);
}

pub fn collect_headers(data: &[TableRow<String, String>]) -> IndexSet<String> {
    data.iter().flat_map(|hm| hm.keys().cloned()).collect()
}

#[test]
fn can_mk_header() {
    let hdr = mk_header(&vec![(s!("bard"), 5), (s!("other"), 8)]);

    // the | below is the margin
    let expected = "
   ||bard | other  |
   ||-----|--------|"
        .strip_margin();
    assert!(hdr == expected);
}

/// Returns a String of the heading and 2nd line of a markdown table.
///
/// # Arguments
///
/// `headings` - vector of headings (column titles over the table)
///
pub fn mk_header(heading_data: &[(String, usize)]) -> String {
    let heading: String = heading_data.iter().fold(String::from("|"), |res, h| {
        format!("{}{: ^width$}|", res, h.0, width = h.1)
    });
    let dashed: String = heading_data.iter().fold(String::from("|"), |res, h| {
        format!("{}{:-^width$}|", res, "-", width = h.1)
    });

    format!("{}\n{}", heading, dashed)
}

#[test]
fn can_mk_data() {
    let tbl_md = mk_data(
        &vec![(s!("foo"), 5), (s!("bar"), 8)],
        &vec![
            linkedhashmap![s!("foo") => s!("ggg"), s!("bar") => s!("fred"), s!("nop") => s!("no")],
            linkedhashmap![s!("foo") => s!("seventy"), s!("bar") => s!("barry"), s!("nop") => s!("no")],
            linkedhashmap![s!("bar") => s!("col has no foo")],
        ],
    );

    // the | below is the margin
    let expected = "
   || ggg |  fred  |
   ||seventy| barry  |
   ||     |col has no foo|"
        .strip_margin();

    println!("{}\n{}", tbl_md, expected);

    assert!(tbl_md == expected);
}

/// Takes an ordered list of tuples; (key, column_width) and a Vector of TableRows, the cell values
/// The TableRow could carry more data than the keys provided. That is, only hm.get(key) will appear in the output.
///
/// returns a string of rows; `\n` separated in the form
///
/// ```text
/// | val1 | val3 | val4 | val5 |
/// ...
/// | val1 | val3 | val4 | val5 |
/// ```
///
/// # Arguments
///
/// `keys` - for the linkedhashmaps. keys determine cell order in a row
/// `data` - Vector of TableRows
///
pub fn mk_data(heading_data: &[(String, usize)], data: &[TableRow<String, String>]) -> String {
    let ret: Vec<String> = data
        .iter()
        .map(|hm| {
            heading_data.iter().fold(String::from("|"), |res, k| {
                let s = match hm.get(&k.0) {
                    Some(x) => x.to_string(),
                    None => "".into(),
                };

                format!("{}{: ^width$}|", res, s, width = k.1)
            })
        })
        .collect::<Vec<String>>();

    // make a new String of all the concatenated fields
    ret.join("\n")
}

#[test]
fn can_make_table() {
    let tbl_md = mk_table(
        &vec![s!("foo"), s!("bar")],
        &vec![
            linkedhashmap![s!("foo") => s!("ggg"), s!("bar") => s!("fred"), s!("nop") => s!("no")],
            linkedhashmap![s!("foo") => s!("seventy"), s!("bar") => s!("barry"), s!("nop") => s!("no")],
            linkedhashmap![s!("bar") => s!("col has no foo")],
        ],
    );

    // the | below is the margin
    let expected = "
    ||  foo  |     bar      |
    ||-------|--------------|
    ||  ggg  |     fred     |
    ||seventy|    barry     |
    ||       |col has no foo|"
        .strip_margin();

    assert!(tbl_md == expected);
}

/// Takes an ordered list of headings and a Vector of TableRows, the cell values
/// and produces a formatted Markdown Table.
///
/// # Arguments
///
/// `headings` - Which values, in that order, to use as the table output
/// `data`     - Vector of TableRows
///
pub fn mk_table(headings: &[String], data: &[TableRow<String, String>]) -> String {
    // for each heading, find the "widest" heading, or value

    let heading_data: Vec<(String, usize)> = headings
        .iter()
        .map(|h| {
            (
                h.clone(),
                data.iter().fold(h.len(), | max, hm |  // how to return a 
                                   cmp::max(max,
                                     match hm.get(h)  {
                                       Some(v) => v.to_string().len(),
                                       None    => 0
                                     }
                                    )),
            )
        })
        .collect::<Vec<(String, usize)>>();

    format!(
        "{}\n{}",
        mk_header(&heading_data),
        mk_data(&heading_data, data)
    )
}

#[test]
fn can_make_table_all_cols() {
    let tbl_md = mk_table_all_cols(&vec![
        linkedhashmap![s!("foo") => s!("ggg"), s!("bar") => s!("fred"), s!("nop") => s!("no")],
        linkedhashmap![s!("foo") => s!("seventy"), s!("bar") => s!("barry"), s!("nop") => s!("no")],
        linkedhashmap![s!("bar") => s!("col has no foo")],
    ]);

    // the | below is the margin
    let expected = "
    ||  foo  |     bar      |nop|
    ||-------|--------------|---|
    ||  ggg  |     fred     |no |
    ||seventy|    barry     |no |
    ||       |col has no foo|   |"
        .strip_margin();

    println!("{}\n{}", tbl_md, expected);

    assert!(tbl_md == expected);
}

/// Takes a Vector of TableRows, and prints a Markdown Table
/// and produces a formatted Markdown Table.
///
/// # Arguments
///
/// `data`     - Vector of TableRows
///
pub fn mk_table_all_cols(data: &[TableRow<String, String>]) -> String {
    let keys: Vec<String> = collect_headers(data).into_iter().collect();

    mk_table(&keys, data)
}

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

    let tbl_md = mk_md_table_from_yaml(yml_data);
    assert!(tbl_md == expected);
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
    mk_table_all_cols( &load_yaml(yaml))
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

    let tbl_md = mk_md_table_from_yaml_with_headings(&headings, yml_data);
    assert!(tbl_md == expected);
}

#[wasm_bindgen]
pub fn mk_md_table_from_yaml_with_headings_list(headings: &str, yaml: &str) -> String {
    mk_md_table_from_yaml_with_headings(
        &headings.split(',').map(String::from).collect::<Vec<_>>(),
        yaml,
    )
}

pub fn mk_md_table_from_yaml_with_headings(headings: &[String], yaml: &str) -> String {
    mk_table(&headings, &load_yaml(yaml))
}

fn load_yaml(yaml: &str) -> Table<String,String> {
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
