use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pythonize::pythonize;
use estat::EstatClient;

#[pyclass]
struct PyEstatClient {
    inner: EstatClient,
}

#[pymethods]
impl PyEstatClient {
    #[new]
    #[pyo3(signature = (api_key, base_url = None))]
    fn new(api_key: String, base_url: Option<String>) -> Self {
        let inner = match base_url {
            Some(url) => EstatClient::with_base_url(api_key, &url),
            None => EstatClient::new(api_key),
        };
        PyEstatClient { inner }
    }

    fn get_stats_list(&self, py: Python, stats_code: &str, limit: usize) -> PyResult<PyObject> {
        let res = self
            .inner
            .get_stats_list(stats_code, limit)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))?;
        
        pythonize(py, &res)
            .map(|bound| bound.unbind())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymodule]
fn estat_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEstatClient>()?;
    Ok(())
}
