use napi_derive::napi;

use crate::core::tachyon::Tachyon;

#[napi]
pub fn tachyon() -> napi::Result<Tachyon> {
  let server = Tachyon::new();
  Ok(server)
}
