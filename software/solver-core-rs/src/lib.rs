use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;
use once_cell::sync::OnceCell;

// generate the table only once
static DATA_TABLE: OnceCell<kewb::DataTable> = OnceCell::new();

#[pyfunction]
fn init_table() {
    DATA_TABLE.get_or_init(kewb::DataTable::default);
}

#[pyfunction]
#[pyo3(signature = (faces, start_max_moves, stop_max_moves, timeout = None))]
fn solve_from_faces(faces: &str, start_max_moves: u8, stop_max_moves: u8, timeout: Option<f32>) -> PyResult<String> {
	if timeout.is_none() {println!("non :(")}

	let data_table_get = DATA_TABLE.get();
	let data_table = match data_table_get {
		Some(v) => v,
		None => return Err(PyTypeError::new_err("DataTable NOT initialized")),
	};
	
	let face_cube_result = kewb::FaceCube::try_from(faces);
	let face_cube = match face_cube_result {
		Ok(v) => v,
		Err(_) => return Err(PyTypeError::new_err("Invalid faces data")),	
	};

	for max_solver_moves in start_max_moves..stop_max_moves {
		let solver = kewb::Solver::new(&data_table, max_solver_moves, timeout);
		println!("Made solver with {} moves", max_solver_moves);
	}
	
	Ok("".to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(init_table, m)?)?;
	m.add_function(wrap_pyfunction!(solve_from_faces, m)?)?;

    Ok(())
}
