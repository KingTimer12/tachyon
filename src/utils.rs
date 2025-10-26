use std::{sync::Arc, time::Duration};

use bytes::Bytes;
use dashmap::DashMap;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};

use crate::{core::router::TachyonRouter, TachyonRequest, TachyonResponse};

#[inline(always)]
pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
  Full::new(chunk.into())
    .map_err(|never| match never {})
    .boxed()
}

#[inline(always)]
pub fn empty() -> BoxBody<Bytes, hyper::Error> {
  Empty::<Bytes>::new()
    .map_err(|never| match never {})
    .boxed()
}

// função pequena de warmup que chama cada handler uma vez
pub fn warmup_routes(routes: &Arc<DashMap<String, TachyonRouter>>) {
  // Cria um request/response mínimo (vazio)
  let req = TachyonRequest::new(serde_json::Value::Null);
  let res = TachyonResponse::new();

  // itera rotas e dispara chamadas blocking em spawn_blocking para inicializar V8/TSFN
  for entry in routes.iter() {
    let handler = entry.value().handler();
    // se o handler for o wrapper que contém tsfn, chamamos de forma blocking
    // aqui assumimos que handler.call() é async; chamamos via spawn_blocking com timeout
    let req_clone = req.clone();
    let res_clone = res.clone();

    // spawn_blocking para não bloquear o runtime async
    let h = tokio::task::spawn_blocking(move || {
      // Dependendo da sua implementação, handler.call pode ser um método sync ou async em objeto;
      // aqui usamos tsfn.call(Blocking) diretamente se você tiver acesso ao TSFN wrapper.
      // Se não, apenas vamos invocar handler.call synchronously as you do in normal flow.
      // Exemplo genérico (pseudocódigo):
      let _ = handler.call(req_clone, res_clone);
    });
  }
}

/// Ultra-fast route matching with zero allocations
/// Optimized for nanosecond-level performance
#[inline]
pub fn route_matches(route_pattern: &str, actual_route: &str) -> bool {
  // Fast path: if lengths are very different, can't match
  if route_pattern.len().abs_diff(actual_route.len()) > 50 {
    return false;
  }

  // Find method separator ':'
  let Some(pattern_colon) = route_pattern.as_bytes().iter().position(|&b| b == b':') else {
    return false;
  };

  let Some(actual_colon) = actual_route.as_bytes().iter().position(|&b| b == b':') else {
    return false;
  };

  // Fast method comparison (usually just 1-6 bytes: GET, POST, PUT, DELETE, PATCH)
  if pattern_colon != actual_colon {
    return false;
  }

  // SAFETY: We know the position is valid and contains ':'
  let pattern_method = unsafe { route_pattern.get_unchecked(..pattern_colon) };
  let actual_method = unsafe { actual_route.get_unchecked(..actual_colon) };

  if pattern_method != actual_method {
    return false;
  }

  // Get paths after method
  let pattern_path = unsafe { route_pattern.get_unchecked(pattern_colon + 1..) };
  let actual_path = unsafe { actual_route.get_unchecked(actual_colon + 1..) };

  // Fast path: exact match (no parameters)
  if pattern_path == actual_path {
    return true;
  }

  // Only do parameter matching if pattern contains ':'
  if !pattern_path.contains(':') {
    return false;
  }

  // Zero-allocation segment matching using iterators
  let mut pattern_segments = pattern_path.split('/');
  let mut actual_segments = actual_path.split('/');

  loop {
    match (pattern_segments.next(), actual_segments.next()) {
      (Some(pattern_seg), Some(actual_seg)) => {
        // Parameter segment (starts with ':')
        if pattern_seg.starts_with(':') {
          // Parameters can't be empty
          if actual_seg.is_empty() {
            return false;
          }
          continue;
        }

        // Exact match required
        if pattern_seg != actual_seg {
          return false;
        }
      }
      (None, None) => return true, // Both exhausted = match
      _ => return false,           // Length mismatch
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_exact_match() {
    assert!(route_matches("0:/users", "0:/users"));
    assert!(route_matches("1:/api/posts", "1:/api/posts"));
  }

  #[test]
  fn test_parameter_match() {
    assert!(route_matches("0:/users/:id", "0:/users/123"));
    assert!(route_matches(
      "0:/users/:id/posts/:postId",
      "0:/users/123/posts/456"
    ));
  }

  #[test]
  fn test_method_mismatch() {
    assert!(!route_matches("0:/users", "1:/users"));
  }

  #[test]
  fn test_path_mismatch() {
    assert!(!route_matches("0:/users", "0:/posts"));
    assert!(!route_matches("0:/users/:id", "0:/posts/123"));
  }

  #[test]
  fn test_length_mismatch() {
    assert!(!route_matches("0:/users/:id", "0:/users/123/extra"));
    assert!(!route_matches("0:/users/:id/posts", "0:/users/123"));
  }

  #[test]
  fn test_empty_parameter() {
    assert!(!route_matches("0:/users/:id", "0:/users/"));
  }
}
