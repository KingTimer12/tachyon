use bytes::Bytes;
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
  utils::{self, empty, full, warmup_routes},
};

static NOTFOUND: &str = "Not Found";

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
  /// Supports both sync and async handlers
  ///
  /// Example usage:
  /// ```javascript
  /// app.get('/', (req, res) => {
  ///   res.send('Hello World!')
  /// })
  ///
  /// app.get('/async', async (req, res) => {
  ///   res.send('Async Hello!')
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
  /// Supports both sync and async handlers
  ///
  /// Example usage:
  /// ```javascript
  /// app.post('/users', (req, res) => {
  ///   res.send('User created!')
  /// })
  ///
  /// app.post('/users', async (req, res) => {
  ///   await db.save(req.body)
  ///   res.send('User created!')
  /// })
  /// ```
  #[napi(
    ts_args_type = r#"route: string, callback: ((req: TachyonRequest, res: TachyonResponse) => void) | ((req: TachyonRequest, res: TachyonResponse) => Promise<void>)"#
  )]
  pub fn post(
    &self,
    route: String,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()> {
    self.call(route, Method::new(1), callback)
  }

  /// Add a PUT route handler with Express-like syntax
  /// Supports both sync and async handlers
  ///
  /// Example usage:
  /// ```javascript
  /// app.put('/users/:id', (req, res) => {
  ///   res.send('User updated!')
  /// })
  /// ```
  #[napi(
    ts_args_type = r#"route: string, callback: ((req: TachyonRequest, res: TachyonResponse) => void) | ((req: TachyonRequest, res: TachyonResponse) => Promise<void>)"#
  )]
  pub fn put(
    &self,
    route: String,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()> {
    self.call(route, Method::new(2), callback)
  }

  /// Add a DELETE route handler with Express-like syntax
  /// Supports both sync and async handlers
  ///
  /// Example usage:
  /// ```javascript
  /// app.delete('/users/:id', (req, res) => {
  ///   res.send('User deleted!')
  /// })
  /// ```
  #[napi(
    ts_args_type = r#"route: string, callback: ((req: TachyonRequest, res: TachyonResponse) => void) | ((req: TachyonRequest, res: TachyonResponse) => Promise<void>)"#
  )]
  pub fn delete(
    &self,
    route: String,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()> {
    self.call(route, Method::new(3), callback)
  }

  /// Add a PATCH route handler with Express-like syntax
  /// Supports both sync and async handlers
  ///
  /// Example usage:
  /// ```javascript
  /// app.patch('/users/:id', (req, res) => {
  ///   res.send('User patched!')
  /// })
  /// ```
  #[napi(
    ts_args_type = r#"route: string, callback: ((req: TachyonRequest, res: TachyonResponse) => void) | ((req: TachyonRequest, res: TachyonResponse) => Promise<void>)"#
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
    let mut result = Vec::with_capacity(self.routes.len());
    for r in self.routes.iter() {
      let key = r.key();
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
    let listener = TcpListener::bind(addr).await.map_err(napi::Error::from)?;
    println!("Listening on http://{}", addr);

    warmup_routes(&self.routes);

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
          eprintln!("Error serving connection: {:?}", err);
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

    // Fast content-type check
    let is_json = req
      .headers()
      .get(header::CONTENT_TYPE)
      .and_then(|ct| ct.to_str().ok())
      .map(|ct| ct.starts_with("application/json"))
      .unwrap_or(false);

    // Pre-build route key for fastest lookup
    let route_key = format!("{}:{}", method.id(), path);

    // Ultra-fast route lookup - try exact match first
    let handler = if let Some(route_ref) = routes.get(&route_key) {
      Some(route_ref.handler())
    } else {
      // Fallback to parameter matching only if exact match fails
      routes
        .iter()
        .find(|entry| utils::route_matches(entry.key(), &route_key))
        .map(|entry| entry.value().handler())
    };

    // If no route found, return 404 immediately
    let Some(handler) = handler else {
      return Ok(
        Response::builder()
          .status(StatusCode::NOT_FOUND)
          .body(full(NOTFOUND))
          .unwrap(),
      );
    };

    // Collect body
    let whole_body = req.collect().await?.to_bytes();

    // Parse JSON only if content-type is JSON
    let data = if is_json {
      serde_json::from_slice(&whole_body).unwrap_or(serde_json::Value::Null)
    } else {
      serde_json::Value::Null
    };

    // Validate JSON for POST/PUT/PATCH
    if data.is_null()
      && is_json
      && (method == Method::Post || method == Method::Put || method == Method::Patch)
    {
      return Ok(
        Response::builder()
          .status(StatusCode::BAD_REQUEST)
          .body(full("Invalid JSON"))
          .unwrap(),
      );
    }

    // Create request and response objects
    let request = TachyonRequest::new(data);
    let response = TachyonResponse::new();

    // Call handler (supports both sync and async)
    handler.call(request, response.clone()).await;

    // Build response with minimal allocations
    let status_code =
      StatusCode::from_u16(response.get_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let mut response_builder = Response::builder().status(status_code);

    let response_data = if let Some(data) = response.take_data() {
      // Auto-detect JSON response
      let trimmed = data.trim_start();
      if trimmed.starts_with('{') || trimmed.starts_with('[') {
        response_builder = response_builder.header(header::CONTENT_TYPE, "application/json");
      }
      full(data)
    } else {
      empty()
    };

    Ok(response_builder.body(response_data).unwrap())
  }
}
