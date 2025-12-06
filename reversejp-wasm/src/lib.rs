use js_sys::Error;
use once_cell::sync::OnceCell;
use reversejp::ReverseJp;
use reversejp::get_landslide_data;
use wasm_bindgen::prelude::*;

static GLOBAL_REVERSE_JP: OnceCell<ReverseJp> = OnceCell::new();

fn get_instance() -> Result<&'static ReverseJp, JsValue> {
    GLOBAL_REVERSE_JP.get_or_try_init(|| {
        ReverseJp::with_embedded_data().map_err(|err| JsValue::from(Error::new(&err.to_string())))
    })
}

/// Initialize the WebAssembly module and preload the embedded geospatial data.
#[wasm_bindgen]
pub fn initialize() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let _ = get_instance()?;
    Ok(())
}

/// Perform a reverse geocoding lookup for the given longitude and latitude.
#[wasm_bindgen]
pub fn find_properties(longitude: f64, latitude: f64) -> Result<JsValue, JsValue> {
    let reverse_jp = get_instance()?;
    let properties = reverse_jp.find_properties(longitude, latitude);

    serde_wasm_bindgen::to_value(&properties)
        .map_err(|err| JsValue::from(Error::new(&err.to_string())))
}

#[wasm_bindgen]
pub fn get_landslide_data_wasm(idx: usize) -> Result<JsValue, JsValue> {
    let result = get_landslide_data(idx);
    match result {
        Ok(data) => Ok(JsValue::from_str(&data)),
        Err(err) => Err(JsValue::from(Error::new(&err.to_string()))),
    }
}
