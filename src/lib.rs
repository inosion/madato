#![feature(use_extern_macros, wasm_custom_section, wasm_import_module, iterator_flatten)]

extern crate indexmap;
extern crate serde_yaml;
extern crate wasm_bindgen;

use indexmap::IndexSet;
use std::cmp;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[allow(unused_macros)]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[allow(unused_macros)]
macro_rules! treemap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::BTreeMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[allow(unused_macros)]
macro_rules! indexset {
    ($( $key: expr ),*) => {{
         let mut map = ::indexmap::IndexSet::new();
         $( map.insert($key); )*
         map
    }}
}

/*
 * https://gist.github.com/kardeiz/26c303957fc298212c3623c01a26f38c
 */
pub trait StripMargin {
    fn strip_margin(self) -> String;
}

impl StripMargin for &'static str {
    fn strip_margin(self) -> String {
        let mut out = Vec::new();
        for l in self.lines().filter(|x| !x.is_empty()) {
            for s in l.splitn(2, '|').nth(1) {
                out.push(s);
            }
        }
        out.join("\n")
    }
}

#[test]
fn can_extract_headers() {
    let hdrs = vec![
        treemap!["foo" => "ggg", "bar" => "fred", "nop" => "no"], // foo bar nop
        treemap!["foo" => "seventy", "bar" => "barry", "nop" => "no", "aaa" => "ddd"], //
        treemap!["bar" => "col has no foo", "fff" => "ffsd"],
    ];

    let expected = indexset!["bar", "foo", "nop", "aaa", "fff"];
    let result = collect_headers(&hdrs);
    assert!(expected == result);
}

pub fn collect_headers<'a>(data: &[BTreeMap<&'a str, &str>]) -> IndexSet<&'a str> {
    data.iter().flat_map(|hm| hm.keys().cloned()).collect()
}

#[test]
fn can_mk_header() {
    let hdr = mk_header(&vec![("bard", 5), ("other", 8)]);

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
pub fn mk_header(heading_data: &[(&str, usize)]) -> String {
    let heading: String = heading_data.iter().fold(String::from("|"), |res, h| {
        format!("{}{: ^width$}|", res, h.0, width = h.1)
    });
    let dashed: String = heading_data.iter().fold(String::from("|"), |res, h| {
        format!("{}{:-^width$}|", res, "-", width = h.1)
    });

    return format!("{}\n{}", heading, dashed);
}

#[test]
fn can_mk_data() {
    let tbl_md = mk_data(
        &vec![("foo", 5), ("bar", 8)],
        &vec![
            treemap!["foo" => "ggg", "bar" => "fred", "nop" => "no"],
            treemap!["foo" => "seventy", "bar" => "barry", "nop" => "no"],
            treemap!["bar" => "col has no foo"],
        ],
    );

    // the | below is the margin
    let expected = "
   || ggg |  fred  |
   ||seventy| barry  |
   ||     |col has no foo|"
        .strip_margin();

    assert!(tbl_md == expected);
}

/// Takes an ordered list of tuples; (key, column_width) and a Vector of BTreeMaps, the cell values
/// The BTreeMap could carry more data than the keys provided. That is, only hm.get(key) will appear in the output.
///
/// returns a string of rows; `\n` separated in the form
///
/// ```
/// | val1 | val3 | val4 | val5 |
/// ...
/// | val1 | val3 | val4 | val5 |
/// ```
///
/// # Arguments
///
/// `keys` - for the treemaps. keys determine cell order in a row
/// `data` - Vector of BTreeMaps
///
pub fn mk_data<T: ToString + ?Sized>(
    heading_data: &[(&str, usize)],
    data: &[BTreeMap<&str, &T>],
) -> String {
    let ret: Vec<String> = data
        .iter()
        .map(|hm| {
            let h: &BTreeMap<&str, &T> = hm;
            let m = heading_data.iter().fold(String::from("|"), |res, k| {
                let s = match h.get(k.0) {
                    Some(x) => x.to_string(),
                    None => "".into(),
                };

                format!("{}{: ^width$}|", res, s, width = k.1)
            });
            return m;
        })
        .collect::<Vec<String>>();

    // make a new String of all the concatenated fields
    return ret.join("\n");
}

#[test]
fn can_make_table() {
    let tbl_md = mk_table(
        &vec!["foo", "bar"],
        &vec![
            treemap!["foo" => "ggg", "bar" => "fred", "nop" => "no"],
            treemap!["foo" => "seventy", "bar" => "barry", "nop" => "no"],
            treemap!["bar" => "col has no foo"],
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

/// Takes an ordered list of headings and a Vector of BTreeMaps, the cell values
/// and produces a formatted Markdown Table.
///
/// # Arguments
///
/// `headings` - Which values, in that order, to use as the table output
/// `data`     - Vector of BTreeMaps
///
pub fn mk_table<T: ToString + ?Sized>(headings: &[&str], data: &[BTreeMap<&str, &T>]) -> String {
    // for each heading, find the "widest" heading, or value

    let heading_data: Vec<(&str, usize)> = headings
        .iter()
        .map(|h| {
            (
                *h,
                data.iter().fold(h.len(), | max, hm |  // how to return a 
                                   cmp::max(max,
                                     match hm.get(h)  {
                                       Some(v) => v.to_string().len(),
                                       None    => 0
                                     }
                                    )),
            )
        })
        .collect::<Vec<(&str, usize)>>();

    format!(
        "{}\n{}",
        mk_header(&heading_data),
        mk_data(&heading_data, data)
    )
}

#[test]
fn can_make_table_all_cols() {
    let tbl_md = mk_table_all_cols(&vec![
        treemap!["foo" => "ggg", "bar" => "fred", "nop" => "no"],
        treemap!["foo" => "seventy", "bar" => "barry", "nop" => "no"],
        treemap!["bar" => "col has no foo"],
    ]);

    // the | below is the margin
    let expected = "
    ||     bar      |  foo  |nop|
    ||--------------|-------|---|
    ||     fred     |  ggg  |no |
    ||    barry     |seventy|no |
    ||col has no foo|       |   |"
        .strip_margin();

    assert!(tbl_md == expected);
}

/// Takes a Vector of BTreeMaps, and prints a Markdown Table
/// and produces a formatted Markdown Table.
///
/// # Arguments
///
/// `data`     - Vector of BTreeMaps
///
pub fn mk_table_all_cols(data: &[BTreeMap<&str, &str>]) -> String {
    let keys: Vec<&str> = collect_headers(data).into_iter().collect();

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

    let tbl_md = mk_md_table_from_yaml(&yml_data);
    assert!(tbl_md == expected);
}

/// Takes a String of YAML. An Array of Maps, 1 Level deep, and returns a Markdown Table
/// ```
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
/// ```
/// |col3| col4  |  data1  |       data2        |
/// |----|-------|---------|--------------------|
/// |100 |gar gar|somevalue|someother value here|
/// |190x|       |  that   |        nice        |
/// |100 | ta da |  this   |someother value here|
/// ```
///
#[wasm_bindgen]
pub fn mk_md_table_from_yaml(yaml: &str) -> String {
    let deserialized_map: Vec<BTreeMap<String, String>> = serde_yaml::from_str(&yaml).unwrap();
    let str_btm = deserialized_map
        .iter()
        .map(|btree| {
            btree
                .iter()
                .map(|(x, y)| (x.as_str(), y.as_str()))
                .collect::<BTreeMap<&str, &str>>()
        })
        .collect::<Vec<_>>();

    mk_table_all_cols(&str_btm)
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
    let headings = vec!["data1", "data2", "col4"];

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

#[wasm_bindgen]
pub fn mk_md_table_from_yaml_with_headings_list(headings: &str, yaml: &str) -> String {
    mk_md_table_from_yaml_with_headings(&headings.split(",").collect::<Vec<_>>(), &yaml)
}

pub fn mk_md_table_from_yaml_with_headings(headings: &[&str], yaml: &str) -> String {
    let deserialized_map: Vec<BTreeMap<String, String>> = serde_yaml::from_str(&yaml).unwrap();
    let str_btm = deserialized_map
        .iter()
        .map(|btree| {
            btree
                .iter()
                .map(|(x, y)| (x.as_str(), y.as_str()))
                .collect::<BTreeMap<&str, &str>>()
        })
        .collect::<Vec<_>>();

    mk_table(&headings, &str_btm)
}
