use bytes::Bytes;
use dashmap::DashMap;
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
  net::{IpAddr, Ipv4Addr, SocketAddr},
  sync::{Arc, Mutex},
};
use tokio::{net::TcpListener, task};

use crate::{
  core::{
    request::TachyonRequest,
    response::{ResponseHandle, TachyonResponse, RESPONSES},
    router::TachyonRouter,
  },
  utils::full,
};

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
    Self {
      routes: Arc::new(DashMap::new()),
    }
  }

  #[napi]
  pub fn get(
    &self,
    route: String,
    callback: Function<'static, FnArgs<(TachyonRequest, ResponseHandle)>>,
  ) -> Result<()> {
    let handler = callback
      .build_threadsafe_function()
      .callee_handled::<true>()
      .weak::<false>()
      .build()?;
    self
      .routes
      .insert(route.to_string(), TachyonRouter::new("GET", handler));
    Ok(())
  }

  #[napi]
  pub fn routes(&self) -> Vec<String> {
    self
      .routes
      .iter()
      .map(|entry| format!("{} {}", entry.key(), entry.value().method()))
      .collect()
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
    routes: Arc<DashMap<String, TachyonRouter>>,
    req: Request<hyper::body::Incoming>,
  ) -> std::result::Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let routes_clone = Arc::clone(&routes);
    for route in routes_clone.iter() {
      if route.key().eq(req.uri().path()) && route.value().method().eq(req.method().as_str()) {
        let response = Arc::new(Mutex::new(TachyonResponse::new()));
        let request = TachyonRequest::new();

        let id: u32 = 1;

        RESPONSES.insert(id, Arc::clone(&response));
        let handle = ResponseHandle::new(id);

        let tsfn = Arc::new(route.value().handler());
        tsfn.call(
          Ok(napi::bindgen_prelude::FnArgs {
            data: (request, handle),
          }),
          ThreadsafeFunctionCallMode::Blocking,
        );
        let resp = response.lock().unwrap();
        if let Some(res) = resp.data() {
          return Ok(Response::new(full(res.to_string())));
        }
        return Ok(Response::new(full("")));
      }
    }
    Ok(Response::new(full("Not Found")))
  }
}
