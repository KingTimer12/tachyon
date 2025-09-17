use bytes::Bytes;
use dashmap::DashMap;
use http_body_util::combinators::BoxBody;
use hyper::{server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use napi_derive::napi;
use tokio::{net::TcpListener, task};
use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, sync::Arc};

use crate::{core::router::{TachyonRouter, TachyonThreadsafeFunction}, utils::full};

#[napi]
pub struct Tachyon {
  routes: Arc<DashMap<String, TachyonRouter>>,
}
#[napi]
impl Tachyon {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      routes: Arc::new(DashMap::new()),
    }
  }

  #[napi(
    ts_args_type = "route: string, handler: (request: TachyonRequest, response: TachyonResponse) => void"
  )]
  pub fn get(&self, route: String, handler: TachyonThreadsafeFunction) {
    self.routes.insert(
      route.to_string(),
      TachyonRouter::new("GET", handler),
    );
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
    let listener = TcpListener::bind(addr)
      .await
      .map_err(|e| napi::Error::from(e))?;
    println!("Listening on http://{}", addr);

    loop {
      let (stream, _) = listener.accept().await.map_err(|e| napi::Error::from(e))?;
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
  ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let routes_clone = Arc::clone(&routes);
    for route in routes_clone.iter() {
      if route.key().eq(req.uri().path()) && route.value().method().eq(req.method().as_str()) {
        return Ok(Response::new(full(
          "Hello, world!",
        )));
      }
    }
    Ok(Response::new(full("Not Found")))
  }
}