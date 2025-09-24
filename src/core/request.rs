use napi_derive::napi;
use serde_json::Value;

#[napi(object)]
#[derive(Debug, Clone)]
pub struct TachyonRequest {
  pub body: Value,
}

impl TachyonRequest {
  pub fn new(body: Value) -> Self {
    Self { body }
  }
}

impl Default for TachyonRequest {
  fn default() -> Self {
    Self::new(Value::Null)
  }
}
