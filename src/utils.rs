use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
  Full::new(chunk.into())
    .map_err(|never| match never {})
    .boxed()
}

pub fn empty() -> BoxBody<Bytes, hyper::Error> {
  Empty::<Bytes>::new()
    .map_err(|never| match never {})
    .boxed()
}

pub fn route_matches(route_pattern: &str, actual_route: &str) -> bool {
  let pattern_colon = route_pattern.find(':');
  let actual_colon = actual_route.find(':');

  if pattern_colon.is_none() || actual_colon.is_none() {
    return false;
  }

  let pattern_method = &route_pattern[..pattern_colon.unwrap()];
  let actual_method = &actual_route[..actual_colon.unwrap()];

  if pattern_method != actual_method {
    return false;
  }

  let pattern_path = &route_pattern[pattern_colon.unwrap() + 1..];
  let actual_path = &actual_route[actual_colon.unwrap() + 1..];

  let pattern_segments: Vec<&str> = pattern_path.split('/').collect();
  let actual_segments: Vec<&str> = actual_path.split('/').collect();

  if pattern_segments.len() != actual_segments.len() {
    return false;
  }

  // Match each segment
  for (pattern_seg, actual_seg) in pattern_segments.iter().zip(actual_segments.iter()) {
    if pattern_seg.starts_with(':') {
      if actual_seg.is_empty() {
        return false;
      }
      continue;
    } else if pattern_seg != actual_seg {
      return false;
    }
  }

  true
}
