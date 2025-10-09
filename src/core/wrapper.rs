use async_trait::async_trait;
use napi::{bindgen_prelude::FnArgs, threadsafe_function::ThreadsafeFunctionCallMode};
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::timeout;

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
    // Create a channel to wait for handler completion
    let (tx, rx) = oneshot::channel();

    // Clone response to check later
    let res_clone = res.clone();

    // Call the handler
    let args = (req, res).into();
    let _ = self
      .tsfn
      .call(args, ThreadsafeFunctionCallMode::NonBlocking);

    // Spawn a task to wait for response data
    tokio::spawn(async move {
      // Poll for data with exponential backoff
      let mut delay = 10; // microseconds
      for _ in 0..50 {
        if res_clone.get_data().is_some() {
          let _ = tx.send(());
          return;
        }
        tokio::time::sleep(Duration::from_micros(delay)).await;
        delay = (delay * 2).min(1000); // max 1ms
      }
      // Timeout - send anyway
      let _ = tx.send(());
    });

    // Wait for handler to complete (with timeout)
    let _ = timeout(Duration::from_millis(100), rx).await;
  }
}
