#![feature(
    use_extern_macros, wasm_custom_section, wasm_import_module, iterator_flatten, slice_patterns,
    extern_prelude, serde_impl
)]

extern crate calamine;
extern crate indexmap;

extern crate linked_hash_map;
extern crate regex;
extern crate serde;
extern crate serde_derive;
extern crate serde_yaml;
extern crate wasm_bindgen;

#[macro_use]
pub mod utils;
pub mod excel;
pub mod types;
pub mod yaml;

use indexmap::IndexSet;
use regex::Regex;
use std::cmp;
use types::*;

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
        &None,
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
pub fn mk_data(
    heading_data: &[(String, usize)],
    data: &[TableRow<String, String>],
    render_options: &Option<RenderOptions>,
) -> String {

    let filters: Option<Vec<KVFilter>> = render_options.clone().and_then(|ro| ro.filters);

    let iter: Box<Iterator<Item=&TableRow<String,String>>> = match filters { 
        None => Box::new(data.iter()),
        Some(vfilts) => Box::new(data.iter().filter(|row| filter_tablerows(row, &vfilts))),
    };

    let ret: Vec<String> = iter
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


fn filter_tablerows(row: &TableRow<String, String>, vfilters: &Vec<KVFilter>) -> bool{
    vfilters.iter().any(|f| tablerow_filter(row, f))
}

///
/// Per row filter. Takes a regex and the row.
/// If the "regex" for a key and a value returns one or more
/// matches (a key - to a cell), then this row is "kept". (returns true)
///
/// If the regex pair in KVFilter returns no matches across all cells the this
/// row is filtered out (return false)
fn tablerow_filter(row: &TableRow<String, String>, filt: &KVFilter) -> bool {
        let key_re = Regex::new(filt.key.as_str()).unwrap();
        let val_re = Regex::new(filt.value.as_str()).unwrap();

        row.keys()
            .filter(|k| {
                key_re.is_match(k) && match row.get(k.clone()) {
                    Some(v) => val_re.is_match(v),
                    None => false,
                }
            })
            .collect::<Vec<_>>()
            .len() > 0
}

#[test]
fn can_make_table() {
    let tbl_md = mk_table(
        &vec![
            linkedhashmap![s!("foo") => s!("ggg"), s!("bar") => s!("fred"), s!("nop") => s!("no")],
            linkedhashmap![s!("foo") => s!("seventy"), s!("bar") => s!("barry"), s!("nop") => s!("no")],
            linkedhashmap![s!("bar") => s!("col has no foo")],
        ],
        &Some(RenderOptions {
            headings: Some(vec![s!("foo"), s!("bar")]),
            ..Default::default()
        }),
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

#[test]
fn can_make_table_all_cols() {
    let tbl_md = mk_table(
        &vec![
            linkedhashmap![s!("foo") => s!("ggg"), s!("bar") => s!("fred"), s!("nop") => s!("no")],
            linkedhashmap![s!("foo") => s!("seventy"), s!("bar") => s!("barry"), s!("nop") => s!("no")],
            linkedhashmap![s!("bar") => s!("col has no foo")],
        ],
        &None,
    );

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

/// Takes an ordered list of headings and a Vector of TableRows, the cell values
/// and produces a formatted Markdown Table.
///
/// # Arguments
///
/// `headings`       - Which values, in that order, to use as the table output
/// `data`           - Vector of TableRows
/// `render_options` - Set of "config" that drives filtering, ordering, output.
///
pub fn mk_table(
    data: &[TableRow<String, String>],
    render_options: &Option<RenderOptions>,
) -> String {
    // for each heading, find the "widest" heading, or value

    let headings = match render_options {
        Some(RenderOptions {
            headings: Some(h), ..
        }) => h.clone(),
        _ => collect_headers(data).into_iter().collect(),
    };

    let heading_data: Vec<(String, usize)> = headings
        .iter()
        .map(|h| {
            (
                h.clone(),
                data.iter().fold(h.len(), |max, hm| {
                    cmp::max(
                        max,
                        match hm.get(h) {
                            Some(v) => v.to_string().len(),
                            None => 0,
                        },
                    )
                }),
            )
        })
        .collect::<Vec<(String, usize)>>();

    format!(
        "{}\n{}",
        mk_header(&heading_data),
        mk_data(&heading_data, data, render_options)
    )
}
