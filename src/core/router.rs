use napi::{bindgen_prelude::FnArgs, threadsafe_function::ThreadsafeFunction};

use crate::core::{request::TachyonRequest, response::ResponseHandle};

pub type TachyonThreadsafeFunction = ThreadsafeFunction<FnArgs<(TachyonRequest, ResponseHandle)>>;

pub struct TachyonRouter {
  method: String,
  #[allow(dead_code)]
  handler: TachyonThreadsafeFunction,
}

impl TachyonRouter {
  pub fn new(method: &str, handler: TachyonThreadsafeFunction) -> Self {
    Self {
      method: method.to_string(),
      handler,
    }
  }

  pub fn method(&self) -> &str {
    &self.method
  }

  pub fn handler(&self) -> &TachyonThreadsafeFunction {
    &self.handler
  }
}
