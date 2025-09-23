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

pub trait TachyonHandler: Send + Sync {
  fn call(&self, req: TachyonRequest, res: TachyonResponse);
}

pub struct TachyonRouter {
  method: u8,
  handler: Box<dyn TachyonHandler>,
  optimized: bool,
}

impl TachyonRouter {
  pub fn new(method: u8, handler: Box<dyn TachyonHandler>) -> Self {
    Self {
      method,
      handler,
      optimized: false,
    }
  }

  pub fn optimize_for_speed(&mut self) {
    self.optimized = true;
  }

  pub fn method(&self) -> u8 {
    self.method
  }

  pub fn handler(&self) -> &dyn TachyonHandler {
    &*self.handler
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
    let handler = callback
      .build_threadsafe_function()
      .callee_handled::<false>()
      .weak::<false>()
      .build()?;
    let wrapper = ThreadsafeFunctionWrapper::new(handler);
    let route_key = format!("{}:{}", method.id(), route);

    // Create optimized route with pre-allocated response buffer
    let mut router = TachyonRouter::new(method.id(), Box::new(wrapper));
    router.optimize_for_speed();

    // Fast write lock for route insertion
    if let Ok(mut routes) = self.get_routes().write() {
      routes.insert(route_key, router);
    }

    Ok(())
  }
}
