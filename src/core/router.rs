use std::sync::Arc;

use async_trait::async_trait;
use napi::{
  bindgen_prelude::{FnArgs, Function},
  Result,
};

use crate::{
  core::{
    methods::Method, request::TachyonRequest, response::TachyonResponse,
    wrapper::ThreadsafeFunctionWrapper,
  },
  Tachyon,
};

#[async_trait]
pub trait TachyonHandler: Send + Sync {
  async fn call(&self, req: TachyonRequest, res: TachyonResponse);
}

pub struct TachyonRouter {
  method: u8,
  handler: Arc<dyn TachyonHandler>,
}

impl TachyonRouter {
  pub fn new(method: u8, handler: Arc<dyn TachyonHandler>) -> Self {
    Self { method, handler }
  }

  pub fn method(&self) -> u8 {
    self.method
  }

  pub fn handler(&self) -> Arc<dyn TachyonHandler> {
    Arc::clone(&self.handler)
  }
}

pub trait HTTPCall {
  fn call(
    &self,
    route: String,
    method: Method,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()>;
}

impl HTTPCall for Tachyon {
  fn call(
    &self,
    route: String,
    method: Method,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()> {
    // Build threadsafe function directly
    // This works for both sync and async JavaScript functions
    let handler = callback
      .build_threadsafe_function()
      .weak::<false>()
      .build()?;

    let wrapper = ThreadsafeFunctionWrapper::new(handler);
    let route_key = format!("{}:{}", method.id(), route);

    // Fast insertion into route table
    let router = TachyonRouter::new(method.id(), Arc::new(wrapper));
    self.get_routes().insert(route_key, router);

    Ok(())
  }
}
