use ::reversejp::{Properties, ReverseJp};
use pyo3::prelude::*;
use std::sync::OnceLock;

// Global instance of ReverseJp that's created once on first access
static GLOBAL_INSTANCE: OnceLock<ReverseJp> = OnceLock::new();

fn get_global_instance() -> &'static ReverseJp {
    GLOBAL_INSTANCE.get_or_init(|| match ReverseJp::with_embedded_data() {
        Ok(instance) => instance,
        Err(_) => panic!("Failed to initialize ReverseJp with embedded data"),
    })
}

// #[derive(IntoPyObject, IntoPyObjectRef)]

#[pyclass]
struct Property {
    #[pyo3(get)]
    code: String,
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    en_name: String,
}

impl From<Properties> for Property {
    fn from(props: Properties) -> Self {
        Self {
            code: props.code,
            name: props.name,
            en_name: props.en_name,
        }
    }
}

#[pyfunction]
fn find_properties(_py: Python, longitude: f64, latitude: f64) -> PyResult<Vec<Property>> {
    let reverse_jp = get_global_instance();
    let rust_properties = reverse_jp.find_properties(longitude, latitude);

    let mut result = Vec::new();
    for props in rust_properties {
        let py_property = Property::from(props);
        result.push(py_property);
    }

    Ok(result)
}

/// A Python module for reverse geocoding in Japan
#[pymodule]
fn reversejp(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Initialize the global instance upfront
    let _ = get_global_instance();

    m.add_class::<Property>()?;
    m.add_function(wrap_pyfunction!(find_properties, m)?)?;

    Ok(())
}
