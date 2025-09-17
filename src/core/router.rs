use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;

pub type TachyonThreadsafeFunction = ThreadsafeFunction<(TachyonRequest, TachyonResponse)>;

#[napi]
#[derive(Debug, Clone, Copy)]
pub struct TachyonRequest;

#[napi]
pub struct TachyonResponse;

pub struct TachyonRouter {
  method: String,
  #[allow(dead_code)]
  handler: TachyonThreadsafeFunction,
}

impl TachyonRouter {
  pub fn new(method: &str, handler: TachyonThreadsafeFunction) -> Self {
    Self { method: method.to_string(), handler }
  }

  pub fn method(&self) -> &str {
    &self.method
  }
}
