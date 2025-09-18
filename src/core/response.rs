use std::sync::{Arc, Mutex};

use dashmap::DashMap;
use napi_derive::napi;
use once_cell::sync::Lazy;
use serde_json::Value;

pub static RESPONSES: Lazy<DashMap<u32, Arc<Mutex<TachyonResponse>>>> = Lazy::new(DashMap::new);

#[napi]
pub struct ResponseHandle {
  pub id: u32,
}

#[napi]
impl ResponseHandle {
  #[napi(constructor)]
  pub fn new(id: u32) -> Self {
    Self { id }
  }

  #[napi]
  pub fn send(&self, data: Value) {
    if let Some(resp) = RESPONSES.get(&self.id) {
      let mut resp = resp.lock().unwrap();
      resp.send(data);
    }
  }
}

#[napi]
pub struct TachyonResponse {
  data: Option<serde_json::Value>,
}

#[napi]
impl TachyonResponse {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self { data: None }
  }

  #[napi]
  pub fn send(&mut self, value: Value) {
    self.data = Some(value);
  }

  pub fn data(&self) -> Option<Value> {
    self.data.clone()
  }
}
