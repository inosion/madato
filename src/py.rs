use pyo3::prelude::*;
// use pythonize::{depythonize_bound, pythonize};

use crate::cal::spreadsheet_to_md as internal_spreadsheet_to_md;
use crate::cal::spreadsheet_to_named_table as internal_spreadsheet_to_named_table;
use crate::csv::csv_file_to_md as internal_csv_file_to_md;
use crate::csv::mk_csv_from_table_result as internal_mk_csv_from_table_result;
use crate::csv::mk_md_table_from_csv as internal_mk_md_table_from_csv;
use crate::types::MadatoError;
use crate::types::RenderOptions as InternalRenderOptions;
use crate::yaml::mk_json_from_table_result as internal_mk_json_from_table_result;
use crate::yaml::mk_md_table_from_yaml as internal_mk_md_table_from_yaml;
// use crate::mk_table as internal_mk_table;
// use crate::types::TableRow;

use crate::yaml::mk_yaml_from_table_result as internal_mk_yaml_from_table_result;
use crate::yaml::yaml_file_to_md as internal_yaml_file_to_md;

fn from_madato_error(e: MadatoError) -> PyErr {
    match e {
        MadatoError::IOError(e) => PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()),
        MadatoError::YamlError(e) => PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()),
        MadatoError::CsvError(e) => PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()),
        MadatoError::JsonError(e) => PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()),
        MadatoError::DataError(_) => PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()),
        MadatoError::CalError(_) => PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()),
    }
}

#[pyclass]
#[derive(Clone)]
pub struct RenderOptions {
    /// Filters to apply to the data
    pub filters: Option<Vec<crate::types::KVFilter>>,

    /// Column headings to use
    pub headings: Option<crate::types::Headers>,

    /// When XLSX, ODS, the sheet name to use
    pub sheet_name: Option<String>,
}

fn from_python_render_options(ro: &Option<RenderOptions>) -> Option<InternalRenderOptions> {
    match ro {
        Some(ro) => Some(InternalRenderOptions {
            filters: ro.filters.clone(),
            headings: ro.headings.clone(),
            sheet_name: ro.sheet_name.clone(),
        }),
        None => None,
    }
}

#[pyfunction]
pub fn yaml_file_to_md(
    filename: String,
    render_options: Option<RenderOptions>,
) -> PyResult<String> {
    internal_yaml_file_to_md(filename, &from_python_render_options(&render_options))
        .map_err(from_madato_error)
}

#[pyfunction]
pub fn yaml_str_to_md(yaml_str: String, render_options: Option<RenderOptions>) -> PyResult<String> {
    Ok(internal_mk_md_table_from_yaml(
        &yaml_str,
        &from_python_render_options(&render_options),
    ))
}

#[pyfunction]
pub fn json_file_to_md(
    filename: String,
    render_options: Option<RenderOptions>,
) -> PyResult<String> {
    internal_yaml_file_to_md(filename, &from_python_render_options(&render_options))
        .map_err(from_madato_error)
}

#[pyfunction]
pub fn json_str_to_md(json: String, render_options: Option<RenderOptions>) -> PyResult<String> {
    Ok(internal_mk_md_table_from_yaml(
        &json,
        &from_python_render_options(&render_options),
    ))
}

#[pyfunction]
pub fn csv_file_to_md(filename: String, render_options: Option<RenderOptions>) -> PyResult<String> {
    internal_csv_file_to_md(filename, &from_python_render_options(&render_options))
        .map_err(from_madato_error)
}

#[pyfunction]
pub fn csv_to_md(csv: String, render_options: Option<RenderOptions>) -> PyResult<String> {
    Ok(internal_mk_md_table_from_csv(
        &csv,
        &from_python_render_options(&render_options),
    ))
}

#[pyfunction]
pub fn spreadsheet_to_md(
    filename: String,
    render_options: Option<RenderOptions>,
) -> PyResult<String> {
    internal_spreadsheet_to_md(filename, &from_python_render_options(&render_options))
        .map_err(|e| from_madato_error(e.into()))
}

#[pyfunction]
pub fn spreadsheet_to_json(filename: String, sheet_name: Option<String>) -> PyResult<String> {
    let tables = internal_spreadsheet_to_named_table(filename, sheet_name);
    internal_mk_json_from_table_result(tables).map_err(from_madato_error)
}

#[pyfunction]
pub fn spreadsheet_to_yaml(filename: String, sheet_name: Option<String>) -> PyResult<String> {
    let tables = internal_spreadsheet_to_named_table(filename, sheet_name);
    internal_mk_yaml_from_table_result(tables).map_err(from_madato_error)
}

#[pyfunction]
pub fn spreadsheet_to_csv(filename: String, sheet_name: Option<String>) -> PyResult<String> {
    let tables = internal_spreadsheet_to_named_table(filename, sheet_name);
    internal_mk_csv_from_table_result(tables).map_err(from_madato_error)
}

#[pymodule]
#[pyo3(name = "madato")]
fn madato(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(csv_file_to_md, m)?)?;
    m.add_function(wrap_pyfunction!(csv_to_md, m)?)?;
    m.add_function(wrap_pyfunction!(spreadsheet_to_csv, m)?)?;

    m.add_function(wrap_pyfunction!(yaml_file_to_md, m)?)?;
    m.add_function(wrap_pyfunction!(yaml_str_to_md, m)?)?;
    m.add_function(wrap_pyfunction!(spreadsheet_to_yaml, m)?)?;

    m.add_function(wrap_pyfunction!(json_file_to_md, m)?)?;
    m.add_function(wrap_pyfunction!(json_str_to_md, m)?)?;
    m.add_function(wrap_pyfunction!(spreadsheet_to_json, m)?)?;

    m.add_function(wrap_pyfunction!(spreadsheet_to_md, m)?)?;
    Ok(())
}
