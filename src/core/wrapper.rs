use async_trait::async_trait;
use napi::{bindgen_prelude::FnArgs, threadsafe_function::ThreadsafeFunctionCallMode};
use std::{sync::Arc, time::Duration};
use tokio::time::{timeout, Instant};

use crate::{core::router::TachyonHandler, TachyonRequest, TachyonResponse};

type TachyonThreadsafeFunction = napi::threadsafe_function::ThreadsafeFunction<
  FnArgs<(TachyonRequest, TachyonResponse)>,
  (),
  FnArgs<(TachyonRequest, TachyonResponse)>,
  napi::Status,
  false,
>;

pub struct ThreadsafeFunctionWrapper {
  tsfn: Arc<TachyonThreadsafeFunction>,
}

impl ThreadsafeFunctionWrapper {
  pub fn new(tsfn: TachyonThreadsafeFunction) -> Self {
    Self {
      tsfn: Arc::new(tsfn),
    }
  }
}

#[async_trait]
impl TachyonHandler for ThreadsafeFunctionWrapper {
  async fn call(&self, req: TachyonRequest, res: TachyonResponse) {
    // clone response para manter a original visível ao echo
    let res_for_js = res.clone(); // this will be passed to JS in the args
    let res_for_poll = res_for_js.clone(); // this will be used for polling after call

    // build args using the clone intended for JS
    let args = (req, res_for_js).into();
    let tsfn = Arc::clone(&self.tsfn);

    // timeout global (adjust if needed)
    let global_timeout = Duration::from_millis(100);
    let start = Instant::now();

    // spawn_blocking: call JS (Blocking) then poll res_for_poll in this same thread
    let blocking_handle = tokio::task::spawn_blocking(move || {
      // call the JS handler (blocks until the callback is scheduled/executed)
      let call_res = tsfn.call(args, ThreadsafeFunctionCallMode::Blocking);

      // polling loop on the clone reserved for polling
      let mut delay_micros = 10u64; // mais agressivo
      let max_delay_micros = 2_000u64; // até 2ms
      loop {
        // leitura rápida
        if res_for_poll.get_data().is_some() {
          break;
        }
        if start.elapsed() >= global_timeout {
          break;
        }

        // sleep bloqueante (estamos dentro de spawn_blocking)
        if delay_micros <= 200 {
          // tiny sleeps — ajudará quando resposta for muito rápida
          std::thread::sleep(Duration::from_micros(delay_micros));
        } else {
          // para delays maiores, use millis (mais eficiente)
          std::thread::sleep(Duration::from_millis((delay_micros / 1000) as u64));
        }

        // exponencial até cap
        delay_micros = (delay_micros.saturating_mul(2)).min(max_delay_micros);
      }

      // return the call result for diagnostics
      call_res
    });

    match timeout(global_timeout, blocking_handle).await {
      Ok(join_result) => match join_result {
        Ok(call_result) => match call_result {
          napi::Status::Ok => {}
          other_status => {
            eprintln!("Error calling JS handler via tsfn: {:?}", other_status);
          }
        },
        Err(join_err) => {
          eprintln!(
            "spawn_blocking join error when calling tsfn: {:?}",
            join_err
          );
        }
      },
      Err(_) => {
        eprintln!("Timeout waiting for JS handler to complete (100ms)");
      }
    }
  }
}
