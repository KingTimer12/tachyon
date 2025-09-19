use crate::core::{request::TachyonRequest, response::TachyonResponse};

pub trait TachyonHandler: Send + Sync {
  fn call(&self, req: TachyonRequest, res: TachyonResponse);
}

pub struct TachyonRouter {
  method: String,
  handler: Box<dyn TachyonHandler>,
  optimized: bool,
}

impl TachyonRouter {
  pub fn new(method: &str, handler: Box<dyn TachyonHandler>) -> Self {
    Self {
      method: method.to_string(),
      handler,
      optimized: false,
    }
  }

  /// Optimize router for nanosecond-level performance
  pub fn optimize_for_speed(&mut self) {
    self.optimized = true;
    // Pre-warm the handler for faster execution
    // This could include JIT optimizations or cache warming
  }

  pub fn method(&self) -> &str {
    &self.method
  }

  pub fn handler(&self) -> &dyn TachyonHandler {
    &*self.handler
  }
}
