#![feature(
    extern_prelude, use_extern_macros, wasm_custom_section, wasm_import_module
)]

extern crate wasm_bindgen;
extern crate madato;

use wasm_bindgen::prelude::*;

/*
//#[wasm_bindgen]
pub fn md_table_yaml_and_headings(headings: &str, yaml: &str) -> String {
    madato::yaml::md_table_yaml_and_headings(headings, yaml)
}
// #[wasm_bindgen]
pub fn mk_md_table_from_yaml(yaml: &str, render_options: &Option<RenderOptions>) -> String {
    madato::yaml::mk_md_table_from_yaml(yaml, render_options)
}
*/
#[wasm_bindgen]
pub fn test_str(tt: String) -> String {
    tt
}
