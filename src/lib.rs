use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_json::Value as Json;
use toml::Value as Toml;

#[wasm_bindgen(catch)]
pub fn parse(input: String) -> Result<JsValue, JsValue> {
    match input.parse::<Toml>() {
        Ok(toml) => {
            let json = toml2json(toml);
            JsValue::from_serde(&json).map_err(|e| JsValue::from_str(&e.to_string()))
        }
        Err(error) => Err(JsValue::from_str(&format!("failed to parse TOML: {}", error))),
    }
}

#[wasm_bindgen]
pub fn stringify(input: JsValue) -> Result<JsValue, JsValue> {
    let json: Json = input.into_serde().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let toml = json2toml(json);
    match toml::to_string_pretty(&toml) {
        Ok(v) => Ok(JsValue::from_str(&v)),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

fn toml2json(toml: Toml) -> Json {
    match toml {
        Toml::String(s) => Json::String(s),
        Toml::Integer(i) => Json::Number(i.into()),
        Toml::Float(f) => serde_json::Number::from_f64(f).map(|n| Json::Number(n)).unwrap(),
        Toml::Boolean(b) => Json::Bool(b),
        Toml::Array(arr) => Json::Array(arr.into_iter().map(toml2json).collect()),
        Toml::Table(table) => {
            Json::Object(table.into_iter().map(|(k, v)| (k, toml2json(v))).collect())
        }
        Toml::Datetime(dt) => Json::String(dt.to_string()),
    }
}

fn json2toml(json: Json) -> Toml {
    match json {
        Json::String(s) => Toml::String(s),
        Json::Number(n) => {
             if n.is_i64() {
                 Toml::Integer(n.as_i64().unwrap())
             }  else {
                 Toml::Float(n.as_f64().unwrap())
             }
        },
        Json::Bool(b) => Toml::Boolean(b),
        Json::Null => Toml::String("null".to_owned()),
        Json::Array(arr) => Toml::Array(arr.into_iter().map(json2toml).collect()),
        Json::Object(table) => {
            Toml::Table(table.into_iter().map(|(k, v)| (k, json2toml(v))).collect())
        }
    }
}