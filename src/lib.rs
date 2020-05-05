use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_json::Value as Json;
use toml::Value as Toml;

#[wasm_bindgen(catch)]
pub fn parse(input: String) -> Result<JsValue, JsValue> {
    match input.parse() {
        Ok(toml) => {
            let json = convert(toml).map_err(|e| JsValue::from_str(&e))?;
            JsValue::from_serde(&json).map_err(|e| JsValue::from_str(&e.to_string()))
        }
        Err(error) => Err(JsValue::from_str(&format!("failed to parse TOML: {}", error))),
    }
}

fn convert(toml: Toml) -> Result<Json, String> {
    match toml {
        Toml::String(s) => Ok(Json::String(s)),
        Toml::Integer(i) => Ok(Json::Number(i.into())),
        Toml::Float(f) => {
            serde_json::Number::from_f64(f).map(|n | Json::Number(n)).ok_or("float infinite and nan not allowed".to_owned())
        }
        Toml::Boolean(b) => Ok(Json::Bool(b)),
        Toml::Array(arr) => arr.into_iter().map(convert).collect::<Result<Vec<Json>, String>>().map(|v| Json::Array(v)),
        Toml::Table(table) => table.into_iter()
            .map(|(k, v)| convert(v).map(|v2| (k, v2)))
            .collect::<Result<serde_json::Map<String, Json>, String>>().map(|v| Json::Object(v)),
        Toml::Datetime(dt) => Ok(Json::String(dt.to_string())),
    }
}