use napi::{bindgen_prelude::FnArgs, threadsafe_function::ThreadsafeFunctionCallMode};

use crate::{core::router::TachyonHandler, TachyonRequest, TachyonResponse};

type TachyonThreadsafeFunction = napi::threadsafe_function::ThreadsafeFunction<
  FnArgs<(TachyonRequest, TachyonResponse)>,
  (),
  FnArgs<(TachyonRequest, TachyonResponse)>,
  napi::Status,
  false,
>;

pub struct ThreadsafeFunctionWrapper {
  tsfn: TachyonThreadsafeFunction,
}

impl ThreadsafeFunctionWrapper {
  pub fn new(tsfn: TachyonThreadsafeFunction) -> Self {
    Self { tsfn }
  }
}

impl TachyonHandler for ThreadsafeFunctionWrapper {
  fn call(&self, req: TachyonRequest, res: TachyonResponse) {
    self
      .tsfn
      .call((req, res).into(), ThreadsafeFunctionCallMode::NonBlocking);
  }
}
