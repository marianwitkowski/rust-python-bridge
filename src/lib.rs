use rand::Rng; 
use pyo3::prelude::*;

#[pyfunction]
fn approximate_pi(iterations: u64) -> PyResult<f64> {
    let mut pi = 0.0;
    for k in 0..iterations {
        // Używamy jawnego typu dla licznika pętli i wyniku
        let term = (-1.0_f64).powi(k as i32) / (2 * k as i32 + 1) as f64;
        pi += term;
    }
    Ok(pi * 4.0)
}

#[pyfunction]
fn approximate_pi_monte_carlo(iterations: u64) -> PyResult<f64> {
    let mut rng = rand::thread_rng();
    let mut points_inside_circle = 0;

    for _ in 0..iterations {
        let x: f64 = rng.gen(); // Generuje liczbę zmiennoprzecinkową od 0.0 do 1.0
        let y: f64 = rng.gen(); // Jak wyżej
        if x*x + y*y <= 1.0 {
            points_inside_circle += 1;
        }
    }

    Ok(4.0 * (points_inside_circle as f64) / (iterations as f64))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_python_bridge(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(approximate_pi_monte_carlo, m)?)?;
    m.add_function(wrap_pyfunction!(approximate_pi, m)?)?;
    Ok(())
}
