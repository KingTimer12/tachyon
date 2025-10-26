use napi_derive::napi;
use serde_json::Value;
use std::sync::atomic::{AtomicPtr, AtomicU16, Ordering};
use std::sync::Arc;

// Ultra-fast lock-free response implementation
#[napi]
pub struct TachyonResponse {
  data: Arc<AtomicPtr<String>>,
  status_code: Arc<AtomicU16>,
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
      data: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
      status_code: Arc::new(AtomicU16::new(200)),
    }
  }

  #[napi]
  pub fn send(&self, msg: Option<Value>) {
    let json_string = serde_json::to_string(&msg).unwrap_or_else(|_| "{}".to_string());

    // Allocate new string on heap and store pointer atomically
    let boxed = Box::new(json_string);
    let new_ptr = Box::into_raw(boxed);

    // Swap old pointer with new one
    let old_ptr = self.data.swap(new_ptr, Ordering::Release);

    // Free old pointer if it exists
    if !old_ptr.is_null() {
      unsafe {
        let _ = Box::from_raw(old_ptr);
      }
    }
  }

  #[napi]
  pub fn status(&self, code: u16) -> TachyonResponse {
    self.status_code.store(code, Ordering::SeqCst);
    Self {
      data: Arc::clone(&self.data),
      status_code: Arc::clone(&self.status_code),
    }
  }

  pub fn take_data(&self) -> Option<String> {
    // Atomically take the data pointer
    let ptr = self.data.swap(std::ptr::null_mut(), Ordering::SeqCst);

    if ptr.is_null() {
      None
    } else {
      unsafe {
        let boxed = Box::from_raw(ptr);
        Some(*boxed)
      }
    }
  }

  pub fn get_data(&self) -> Option<String> {
    // Read without taking ownership
    let ptr = self.data.load(Ordering::SeqCst);

    if ptr.is_null() {
      None
    } else {
      unsafe {
        let data = &*ptr;
        Some(data.clone())
      }
    }
  }

  pub fn get_status(&self) -> u16 {
    self.status_code.load(Ordering::SeqCst)
  }

  pub fn inner_ptr(&self) -> *const () {
    Arc::as_ptr(&self.data) as *const ()
  }
}

impl Clone for TachyonResponse {
  fn clone(&self) -> Self {
    Self {
      data: Arc::clone(&self.data),
      status_code: Arc::clone(&self.status_code),
    }
  }
}

impl Drop for TachyonResponse {
  fn drop(&mut self) {
    // Only free if this is the last reference
    if Arc::strong_count(&self.data) == 1 {
      let ptr = self.data.load(Ordering::Acquire);
      if !ptr.is_null() {
        unsafe {
          let _ = Box::from_raw(ptr);
        }
      }
    }
  }
}
