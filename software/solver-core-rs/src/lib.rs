use pyo3::prelude::*;
use pyo3::exceptions::{PyValueError, PyRuntimeError, PyException};
use once_cell::sync::OnceCell;

pyo3::create_exception!(_core, CubieCreationError, PyException);

// generate the table only once
static DATA_TABLE: OnceCell<kewb::DataTable> = OnceCell::new();

#[pyfunction]
fn init_table() {
    DATA_TABLE.get_or_init(kewb::DataTable::default);
}

#[pyfunction]
#[pyo3(signature = (faces, start_max_moves, stop_max_moves, timeout = None))]
// max moves is inclusive: [start_max_moves; stop_max_moves]
fn solve_from_faces(faces: &str, start_max_moves: u8, stop_max_moves: u8, timeout: Option<f32>) -> PyResult<String> {
	let data_table_get = DATA_TABLE.get();
	let data_table = match data_table_get {
		Some(v) => v,
		None => return Err(PyRuntimeError::new_err("DataTable not initialized")),
	};
	
	let face_cube_result = kewb::FaceCube::try_from(faces);
	let face_cube = match face_cube_result {
		Ok(v) => v,
		Err(_) => return Err(PyValueError::new_err("Invalid faces data")),	
	};

	let starting_state_result = kewb::CubieCube::try_from(&face_cube);
	let starting_state = match starting_state_result {
		Ok(v) => v,
		Err(_) => return Err(CubieCreationError::new_err("Could not create CubieCube from FaceCube"))
	};
	
	for max_solver_moves in start_max_moves .. stop_max_moves + 1 {
		let mut solver = kewb::Solver::new(data_table, max_solver_moves, timeout);
		println!("Made solver with {} moves", max_solver_moves);
		match solver.solve(starting_state) {
			Some(solution) => {
				println!("{}",solution);
			},
			None => println!("no solution :(, at least this time")
		};
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

	m.add("CubieCreationError", m.py().get_type::<CubieCreationError>())?;

    Ok(())
}
