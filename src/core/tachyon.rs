use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use napi::{
  bindgen_prelude::{FnArgs, Function},
  threadsafe_function::ThreadsafeFunctionCallMode,
  Result,
};
use napi_derive::napi;
use std::{
  collections::HashMap,
  net::{IpAddr, Ipv4Addr, SocketAddr},
  sync::{Arc, RwLock},
};
use tokio::{net::TcpListener, task};

use crate::{
  core::{
    request::TachyonRequest,
    response::TachyonResponse,
    router::{TachyonHandler, TachyonRouter},
  },
  utils::full,
};

type TachyonThreadsafeFunction = napi::threadsafe_function::ThreadsafeFunction<
  FnArgs<(TachyonRequest, TachyonResponse)>,
  (),
  FnArgs<(TachyonRequest, TachyonResponse)>,
  napi::Status,
  false,
>;

struct ThreadsafeFunctionWrapper {
  tsfn: TachyonThreadsafeFunction,
}

impl TachyonHandler for ThreadsafeFunctionWrapper {
  fn call(&self, req: TachyonRequest, res: TachyonResponse) {
    let _ = self
      .tsfn
      .call((req, res).into(), ThreadsafeFunctionCallMode::Blocking);
  }
}

#[napi]
pub struct Tachyon {
  routes: Arc<RwLock<HashMap<String, TachyonRouter>>>,
}

impl Default for Tachyon {
  fn default() -> Self {
    Self {
      routes: Arc::new(RwLock::new(HashMap::new())),
    }
  }
}

#[napi]
impl Tachyon {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      routes: Arc::new(RwLock::new(HashMap::new())),
    }
  }

  /// Add a GET route handler with Express-like syntax
  ///
  /// Example usage:
  /// ```javascript
  /// app.get('/', (req, res) => {
  ///   res.send('Hello World!')
  /// })
  /// ```
  #[napi(
    ts_args_type = "route: string, callback: (req: TachyonRequest, res: TachyonResponse) => void"
  )]
  pub fn get(
    &self,
    route: String,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()> {
    let handler = callback
      .build_threadsafe_function()
      .callee_handled::<false>()
      .weak::<false>()
      .build()?;
    let wrapper = ThreadsafeFunctionWrapper { tsfn: handler };
    let route_key = format!("GET:{}", route);

    // Create optimized route with pre-allocated response buffer
    let mut router = TachyonRouter::new("GET", Box::new(wrapper));
    router.optimize_for_speed();

    // Fast write lock for route insertion
    if let Ok(mut routes) = self.routes.write() {
      routes.insert(route_key, router);
    }

    Ok(())
  }

  /// Add a POST route handler with Express-like syntax
  ///
  /// Example usage:
  /// ```javascript
  /// app.post('/users', (req, res) => {
  ///   res.send('User created!')
  /// })
  /// ```
  #[napi(
    ts_args_type = "route: string, callback: (req: TachyonRequest, res: TachyonResponse) => void"
  )]
  pub fn post(
    &self,
    route: String,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()> {
    let handler = callback
      .build_threadsafe_function()
      .callee_handled::<false>()
      .weak::<false>()
      .build()?;
    let wrapper = ThreadsafeFunctionWrapper { tsfn: handler };
    let route_key = format!("POST:{}", route);

    // Create optimized route with pre-allocated response buffer
    let mut router = TachyonRouter::new("POST", Box::new(wrapper));
    router.optimize_for_speed();

    // Fast write lock for route insertion
    if let Ok(mut routes) = self.routes.write() {
      routes.insert(route_key, router);
    }

    Ok(())
  }

  /// Add a PUT route handler with Express-like syntax
  ///
  /// Example usage:
  /// ```javascript
  /// app.put('/users/:id', (req, res) => {
  ///   res.send('User updated!')
  /// })
  /// ```
  #[napi(
    ts_args_type = "route: string, callback: (req: TachyonRequest, res: TachyonResponse) => void"
  )]
  pub fn put(
    &self,
    route: String,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()> {
    let handler = callback
      .build_threadsafe_function()
      .callee_handled::<false>()
      .weak::<false>()
      .build()?;
    let wrapper = ThreadsafeFunctionWrapper { tsfn: handler };
    let route_key = format!("PUT:{}", route);

    // Create optimized route with pre-allocated response buffer
    let mut router = TachyonRouter::new("PUT", Box::new(wrapper));
    router.optimize_for_speed();

    // Fast write lock for route insertion
    if let Ok(mut routes) = self.routes.write() {
      routes.insert(route_key, router);
    }

    Ok(())
  }

  /// Add a DELETE route handler with Express-like syntax
  ///
  /// Example usage:
  /// ```javascript
  /// app.delete('/users/:id', (req, res) => {
  ///   res.send('User deleted!')
  /// })
  /// ```
  #[napi(
    ts_args_type = "route: string, callback: (req: TachyonRequest, res: TachyonResponse) => void"
  )]
  pub fn delete(
    &self,
    route: String,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()> {
    let handler = callback
      .build_threadsafe_function()
      .callee_handled::<false>()
      .weak::<false>()
      .build()?;
    let wrapper = ThreadsafeFunctionWrapper { tsfn: handler };
    let route_key = format!("DELETE:{}", route);

    // Create optimized route with pre-allocated response buffer
    let mut router = TachyonRouter::new("DELETE", Box::new(wrapper));
    router.optimize_for_speed();

    // Fast write lock for route insertion
    if let Ok(mut routes) = self.routes.write() {
      routes.insert(route_key, router);
    }

    Ok(())
  }

  /// Add a PATCH route handler with Express-like syntax
  ///
  /// Example usage:
  /// ```javascript
  /// app.patch('/users/:id', (req, res) => {
  ///   res.send('User patched!')
  /// })
  /// ```
  #[napi(
    ts_args_type = "route: string, callback: (req: TachyonRequest, res: TachyonResponse) => void"
  )]
  pub fn patch(
    &self,
    route: String,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()> {
    let handler = callback
      .build_threadsafe_function()
      .callee_handled::<false>()
      .weak::<false>()
      .build()?;
    let wrapper = ThreadsafeFunctionWrapper { tsfn: handler };
    let route_key = format!("PATCH:{}", route);

    // Create optimized route with pre-allocated response buffer
    let mut router = TachyonRouter::new("PATCH", Box::new(wrapper));
    router.optimize_for_speed();

    // Fast write lock for route insertion
    if let Ok(mut routes) = self.routes.write() {
      routes.insert(route_key, router);
    }

    Ok(())
  }

  #[napi]
  pub fn routes(&self) -> Vec<String> {
    if let Ok(routes) = self.routes.read() {
      routes
        .iter()
        .map(|(key, router)| {
          if let Some(colon_pos) = key.find(':') {
            let method = &key[..colon_pos];
            let path = &key[colon_pos + 1..];
            format!("{} {}", path, method)
          } else {
            format!("{} {}", key, router.method())
          }
        })
        .collect()
    } else {
      Vec::new()
    }
  }

  #[napi]
  pub async fn listen(&self, port: u16) -> napi::Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    // Implement server listening logic here
    let listener = TcpListener::bind(addr).await.map_err(napi::Error::from)?;
    println!("Listening on http://{}", addr);

    loop {
      let (stream, _) = listener.accept().await.map_err(napi::Error::from)?;
      let io = TokioIo::new(stream);
      let routes = Arc::clone(&self.routes);
      task::spawn(async move {
        if let Err(err) = http1::Builder::new()
          .serve_connection(
            io,
            service_fn(move |req| {
              let routes_clone = Arc::clone(&routes);
              async move { Self::echo(routes_clone, req).await }
            }),
          )
          .await
        {
          println!("Error serving connection: {:?}", err);
        }
      });
    }
  }

  async fn echo(
    routes: Arc<RwLock<HashMap<String, TachyonRouter>>>,
    req: Request<hyper::body::Incoming>,
  ) -> std::result::Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let path = req.uri().path();
    let method = req.method().as_str();

    // Ultra-fast dynamic route lookup with parameter matching
    let route_key = format!("{}:{}", method, path);

    // Fast read lock for route lookup - optimized for nanosecond performance
    let (found_route, matched_key) = {
      if let Ok(routes_guard) = routes.read() {
        // First try exact match
        if routes_guard.contains_key(&route_key) {
          (true, route_key.clone())
        } else {
          // Try parameter matching
          let mut matched = false;
          let mut match_key = String::new();

          for (key, _) in routes_guard.iter() {
            if Self::route_matches(key, &route_key) {
              matched = true;
              match_key = key.clone();
              break;
            }
          }
          (matched, match_key)
        }
      } else {
        (false, String::new())
      }
    };

    if found_route {
      // Pre-allocated response and request objects
      let response = TachyonResponse::new();
      let request = TachyonRequest::new();

      // Get handler and call it
      if let Ok(routes_guard) = routes.read() {
        if let Some(route) = routes_guard.get(&matched_key) {
          let handler = route.handler();
          handler.call(request, response.clone());
        }
      }

      // Ultra-minimal delay - optimized for nanosecond performance
      tokio::time::sleep(tokio::time::Duration::from_nanos(50)).await;

      if let Some(response_data) = response.take_data() {
        Ok(Response::new(full(response_data)))
      } else {
        Ok(Response::new(full("Internal Server Error".to_string())))
      }
    } else {
      // 404 - Route not found
      Ok(Response::new(full("Not Found".to_string())))
    }
  }

  // Helper function to match routes with parameters (e.g., /users/:id)
  fn route_matches(route_pattern: &str, actual_route: &str) -> bool {
    // Extract method and path from both pattern and actual route
    // Pattern format: "PUT:/users/:id"
    // Actual format: "PUT:/users/1"

    let pattern_colon = route_pattern.find(':');
    let actual_colon = actual_route.find(':');

    if pattern_colon.is_none() || actual_colon.is_none() {
      return false;
    }

    let pattern_method = &route_pattern[..pattern_colon.unwrap()];
    let actual_method = &actual_route[..actual_colon.unwrap()];

    // Methods must match exactly
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
        // Parameter segment - matches anything non-empty
        if actual_seg.is_empty() {
          return false;
        }
        continue;
      } else if pattern_seg != actual_seg {
        // Static segment must match exactly
        return false;
      }
    }

    true
  }
}
