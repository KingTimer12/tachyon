use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{server::conn::http1, service::service_fn, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use napi::{
  bindgen_prelude::{FnArgs, Function},
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
    methods::Method,
    request::TachyonRequest,
    response::TachyonResponse,
    router::{HTTPCall, TachyonRouter},
  },
  utils::{self, empty, full},
};

// static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";

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
    self.call(route, Method::new(0), callback)
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
    self.call(route, Method::new(1), callback)
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
    self.call(route, Method::new(2), callback)
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
    self.call(route, Method::new(3), callback)
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
    self.call(route, Method::new(4), callback)
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

  pub fn get_routes(&self) -> Arc<RwLock<HashMap<String, TachyonRouter>>> {
    Arc::clone(&self.routes)
  }

  async fn echo(
    routes: Arc<RwLock<HashMap<String, TachyonRouter>>>,
    req: Request<hyper::body::Incoming>,
  ) -> std::result::Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let path = req.uri().path();
    let method = Method::from(req.method());
    let response_builder = Response::builder();

    // Ultra-fast dynamic route lookup with parameter matching
    let route_key = format!("{}:{}", method.id(), path);

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
            if utils::route_matches(key, &route_key) {
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

      tokio::time::sleep(tokio::time::Duration::from_micros(0)).await; // To prevent error in send response

      let status_code = match StatusCode::from_u16(response.get_status()) {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Ok(status_code) => status_code,
      };
      let response_data = match response.take_data() {
        Some(data) => full(data),
        None => empty(),
      };

      let response_builder = response_builder.status(status_code).body(response_data);
      Ok(response_builder.unwrap())
    } else {
      Ok(
        response_builder
          .status(StatusCode::NOT_FOUND)
          .body(full(NOTFOUND))
          .unwrap(),
      )
    }
  }
}
