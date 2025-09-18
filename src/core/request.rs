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
