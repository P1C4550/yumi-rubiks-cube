use pyo3::prelude::*;
use once_cell::sync::OnceCell;

static DATA_TABLE: OnceCell<kewb::DataTable> = OnceCell::new();

#[pyfunction]
fn init_table() {
    DATA_TABLE.get_or_init(kewb::DataTable::default);
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(init_table, m)?)?;

    Ok(())
}
