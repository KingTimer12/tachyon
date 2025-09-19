use napi_derive::napi;
use serde_json::Value;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;

// Ultra-fast response using minimal synchronization
#[napi]
pub struct TachyonResponse {
  data: Arc<std::sync::Mutex<Option<String>>>,
  status_code: AtomicU16,
}

impl Default for TachyonResponse {
  fn default() -> Self {
    Self::new()
  }
}

#[napi]
impl TachyonResponse {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      data: Arc::new(std::sync::Mutex::new(None)),
      status_code: AtomicU16::new(200),
    }
  }

  #[napi]
  pub fn send(&self, msg: String) -> String {
    if let Ok(mut data) = self.data.lock() {
      *data = Some(msg.clone());
    }
    msg
  }

  #[napi]
  pub fn status(&self, code: u16) -> TachyonResponse {
    self.status_code.store(code, Ordering::Relaxed);
    TachyonResponse {
      data: Arc::clone(&self.data),
      status_code: AtomicU16::new(code),
    }
  }

  #[napi]
  pub fn json(&self, data: Value) -> String {
    let json_string = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());
    if let Ok(mut data_guard) = self.data.lock() {
      *data_guard = Some(json_string.clone());
    }
    json_string
  }

  pub fn take_data(&self) -> Option<String> {
    if let Ok(mut data) = self.data.lock() {
      data.take()
    } else {
      None
    }
  }

  pub fn get_data(&self) -> Option<String> {
    if let Ok(data) = self.data.lock() {
      data.clone()
    } else {
      None
    }
  }

  pub fn get_status(&self) -> u16 {
    self.status_code.load(Ordering::Relaxed)
  }

  pub fn inner_ptr(&self) -> *const () {
    Arc::as_ptr(&self.data) as *const ()
  }
}

impl Clone for TachyonResponse {
  fn clone(&self) -> Self {
    Self {
      data: Arc::clone(&self.data),
      status_code: AtomicU16::new(self.status_code.load(Ordering::Relaxed)),
    }
  }
}
