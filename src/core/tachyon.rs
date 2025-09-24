use bytes::{Buf, Bytes};
use dashmap::DashMap;
use http_body_util::{combinators::BoxBody, BodyExt};
use hyper::{header, server::conn::http1, service::service_fn, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use napi::{
  bindgen_prelude::{FnArgs, Function},
  Result,
};
use napi_derive::napi;
use std::{
  net::{IpAddr, Ipv4Addr, SocketAddr},
  sync::Arc,
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

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";

#[napi]
pub struct Tachyon {
  routes: Arc<DashMap<String, TachyonRouter>>,
}

impl Default for Tachyon {
  fn default() -> Self {
    Self {
      routes: Arc::new(DashMap::new()),
    }
  }
}

#[napi]
impl Tachyon {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self::default()
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
    ts_args_type = r#"route: string, callback: ((req: TachyonRequest, res: TachyonResponse) => void) | ((req: TachyonRequest, res: TachyonResponse) => Promise<void>)"#,
    js_name = "get"
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
    let mut result = Vec::new();
    for r in self.routes.iter() {
      let key = r.key().clone();
      let router = r.value();
      if let Some(colon_pos) = key.find(':') {
        let method = &key[..colon_pos];
        let path = &key[colon_pos + 1..];
        result.push(format!("{} {}", path, Method::from(method)));
      } else {
        result.push(format!("{} {}", key, Method::new(router.method())));
      }
    }
    result
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

  pub fn get_routes(&self) -> Arc<DashMap<String, TachyonRouter>> {
    Arc::clone(&self.routes)
  }

  async fn echo(
    routes: Arc<DashMap<String, TachyonRouter>>,
    req: Request<hyper::body::Incoming>,
  ) -> std::result::Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let path = req.uri().path();
    let method = Method::from(req.method());
    let is_json = req
      .headers()
      .get("content-type")
      .and_then(|ct| ct.to_str().ok())
      .map(|ct| ct.to_ascii_lowercase().starts_with("application/json"))
      .unwrap_or(false);
    let response_builder = Response::builder();

    // Ultra-fast dynamic route lookup with parameter matching
    let route_key = format!("{}:{}", method.id(), path);

    // Fast read lock for route lookup - optimized for nanosecond performance
    let (found_route, matched_key) = {
      if routes.contains_key(&route_key) {
        (true, route_key.clone())
      } else {
        // Try parameter matching
        let mut matched = false;
        let mut match_key = String::new();

        for key in routes.iter().map(|f| f.key().clone()) {
          if utils::route_matches(&key, &route_key) {
            matched = true;
            match_key = key.clone();
            break;
          }
        }
        (matched, match_key)
      }
    };

    if found_route {
      // Pre-allocated response and request objects
      let whole_body = req.collect().await?.aggregate();
      let mut data = serde_json::Value::Null;

      if is_json {
        data = match serde_json::from_reader(whole_body.reader()) {
          Ok(json) => json,
          Err(_) => serde_json::Value::Null,
        };
      }
      if data.is_null()
        && is_json
        && (method == Method::Post || method == Method::Put || method == Method::Patch)
      {
        return Ok(
          response_builder
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(full(INTERNAL_SERVER_ERROR))
            .unwrap(),
        );
      }
      let response = TachyonResponse::new();
      let request = TachyonRequest::new(data);

      // Get handler and call it
      let maybe_handler = {
        if let Some(route_ref) = routes.get(&matched_key) {
          Some(route_ref.handler())
        } else {
          None
        }
      };

      if let Some(handler) = maybe_handler {
        handler.call(request, response.clone()).await;
      }

      let status_code = match StatusCode::from_u16(response.get_status()) {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Ok(status_code) => status_code,
      };
      let mut response_builder = response_builder.status(status_code);
      let response_data = match response.take_data() {
        Some(data) => {
          if data.trim_start().starts_with('{') || data.trim_start().starts_with('[') {
            response_builder = response_builder.header(header::CONTENT_TYPE, "application/json");
          }
          full(data)
        }
        None => empty(),
      };
      let response_builder = response_builder.body(response_data);
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
