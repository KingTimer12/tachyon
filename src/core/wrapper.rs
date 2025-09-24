use async_trait::async_trait;
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

#[async_trait]
impl TachyonHandler for ThreadsafeFunctionWrapper {
  async fn call(&self, req: TachyonRequest, res: TachyonResponse) {
    let func_async = (req.clone(), res.clone()).into();
    match self.tsfn.call_async(func_async).await {
      Ok(ret) => {
        eprintln!("tsfn.call_async sucesso. {:?}", ret);
      }
      Err(err) => {
        println!("tsfn.call_async erro: {:?}", err);

        // fallback
        let func_sync = (req, res).into();
        self
          .tsfn
          .call(func_sync, ThreadsafeFunctionCallMode::NonBlocking);
      }
    }
  }
}
