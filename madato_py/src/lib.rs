use madatointernal::types::MadatoError;
use pyo3::prelude::*;
extern crate madato as madatointernal;
// use pythonize::{depythonize_bound, pythonize};

use madato_cal::spreadsheet_to_md as internal_spreadsheet_to_md;
use madato_cal::spreadsheet_to_named_table as internal_spreadsheet_to_named_table;
use madatointernal::csv::csv_file_to_md as internal_csv_file_to_md;
use madatointernal::csv::mk_csv_from_table_result as internal_mk_csv_from_table_result;
use madatointernal::csv::mk_md_table_from_csv as internal_mk_md_table_from_csv;
use madatointernal::types::RenderOptions;
use madatointernal::yaml::mk_json_from_table_result as internal_mk_json_from_table_result;
use madatointernal::yaml::mk_md_table_from_yaml as internal_mk_md_table_from_yaml;
// use madatointernal::mk_table as internal_mk_table;
// use madatointernal::types::TableRow;

use madatointernal::yaml::mk_yaml_from_table_result as internal_mk_yaml_from_table_result;
use madatointernal::yaml::yaml_file_to_md as internal_yaml_file_to_md;

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
pub struct PyRenderOptions(RenderOptions);

#[pyfunction]
pub fn yaml_file_to_md(
    filename: String,
    render_options: Option<PyRenderOptions>,
) -> PyResult<String> {
    internal_yaml_file_to_md(filename, &render_options.map(|r| r.0)).map_err(from_madato_error)
}

#[pyfunction]
pub fn yaml_str_to_md(
    yaml_str: String,
    render_options: Option<PyRenderOptions>,
) -> PyResult<String> {
    Ok(internal_mk_md_table_from_yaml(
        &yaml_str,
        &render_options.map(|r| r.0),
    ))
}

// #[pyfunction]
// pub fn yaml_to_md(yaml: &PyAny, render_options: Option<PyRenderOptions>) -> PyResult<String> {
//     // Ok(internal_mk_md_table_from_yaml(&yaml, &render_options.map(|r| r.0)))
//     Python::with_gil(|py| {
//         let table: &[TableRow<String, String>] = depythonize_bound(yaml.into_bound(py)).unwrap();
//         Ok(internal_mk_table(&table, &render_options.map(|r| r.0)))

//     })

// }

#[pyfunction]
pub fn json_file_to_md(
    filename: String,
    render_options: Option<PyRenderOptions>,
) -> PyResult<String> {
    internal_yaml_file_to_md(filename, &render_options.map(|r| r.0)).map_err(from_madato_error)
}

#[pyfunction]
pub fn json_str_to_md(json: String, render_options: Option<PyRenderOptions>) -> PyResult<String> {
    Ok(internal_mk_md_table_from_yaml(
        &json,
        &render_options.map(|r| r.0),
    ))
}

#[pyfunction]
pub fn csv_file_to_md(
    filename: String,
    render_options: Option<PyRenderOptions>,
) -> PyResult<String> {
    internal_csv_file_to_md(filename, &render_options.map(|r| r.0)).map_err(from_madato_error)
}

#[pyfunction]
pub fn csv_to_md(csv: String, render_options: Option<PyRenderOptions>) -> PyResult<String> {
    Ok(internal_mk_md_table_from_csv(
        &csv,
        &render_options.map(|r| r.0),
    ))
}

#[pyfunction]
pub fn spreadsheet_to_md(
    filename: String,
    render_options: Option<PyRenderOptions>,
) -> PyResult<String> {
    internal_spreadsheet_to_md(filename, &render_options.map(|r| r.0))
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
