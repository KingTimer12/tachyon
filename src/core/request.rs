use napi_derive::napi;

#[napi]
#[derive(Debug, Clone, Copy)]
pub struct TachyonRequest;

#[napi]
impl TachyonRequest {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self
  }
}

impl Default for TachyonRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl TachyonRequest {
  pub fn method(&self) -> &str {
    "GET"
  }
}
