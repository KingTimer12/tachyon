use std::{
  sync::Arc,
  time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use napi::{
  bindgen_prelude::{FnArgs, FromNapiValue, Function, JsObjectValue, JsValuesTuple},
  Env, Result,
};

use crate::{
  core::{
    methods::Method, request::TachyonRequest, response::TachyonResponse,
    wrapper::ThreadsafeFunctionWrapper,
  },
  Tachyon,
};

fn make_js_wrapper<'env>(
  env: &'env Env,
  callback: Function<'env, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
) -> Result<Function<'env, FnArgs<(TachyonRequest, TachyonResponse)>, ()>> {
  // id único baseado em tempo (você pode usar rand se preferir)
  let rand_id: u64 = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_nanos() as u64;
  let tmp_name = format!("__tachyon_cb_{}", rand_id);

  // obter global object e setar a função lá
  let mut global = env.get_global()?;
  // registra a função do usuário em globalThis under tmp_name
  global.set_named_property(&tmp_name, callback)?;

  // script que cria o wrapper que descarta retorno e captura rejeições
  let script = format!(
    r#"
    (function() {{
      const orig = globalThis["{tmp_name}"];
      const wrapper = function(req, res) {{
        try {{
          const ret = orig(req, res);
          if (ret && typeof ret.then === 'function') {{
            ret.catch(err => {{
              try {{ console.error('handler promise rejected:', err); }} catch(e){{ }}
            }});
          }}
        }} catch (e) {{
          try {{ console.error('handler threw:', e); }} catch(_){{ }}
        }}
        return undefined;
      }};
      wrapper;
    }})()
    "#
  );

  // executa script e converte o resultado em Function com o mesmo lifetime
  let val: napi::Unknown = env.run_script(&script)?;
  let wrapper_fn: Function<'env, FnArgs<(TachyonRequest, TachyonResponse)>, ()> =
    Function::from_unknown(val)?;

  Ok(wrapper_fn)
}

#[async_trait]
pub trait TachyonHandler: Send + Sync {
  async fn call(&self, req: TachyonRequest, res: TachyonResponse);
}

pub struct TachyonRouter {
  method: u8,
  handler: Arc<dyn TachyonHandler>,
  optimized: bool,
}

impl TachyonRouter {
  pub fn new(method: u8, handler: Arc<dyn TachyonHandler>) -> Self {
    Self {
      method,
      handler,
      optimized: false,
    }
  }

  pub fn optimize_for_speed(&mut self) {
    self.optimized = true;
  }

  pub fn method(&self) -> u8 {
    self.method
  }

  pub fn handler(&self) -> Arc<dyn TachyonHandler> {
    Arc::clone(&self.handler)
  }
}

pub trait HTTPCall {
  fn call(
    &self,
    route: String,
    method: Method,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()>;
}

impl HTTPCall for Tachyon {
  fn call(
    &self,
    route: String,
    method: Method,
    callback: Function<'static, FnArgs<(TachyonRequest, TachyonResponse)>, ()>,
  ) -> Result<()> {
    let env = callback.env();
    let env_owned = Env::from_raw(env);
    let wrapper_fn = make_js_wrapper(&env_owned, callback)?;
    let handler = wrapper_fn
      .build_threadsafe_function()
      .callee_handled::<false>()
      .weak::<false>()
      .build()?;
    let wrapper = ThreadsafeFunctionWrapper::new(handler);
    let route_key = format!("{}:{}", method.id(), route);

    // Create optimized route with pre-allocated response buffer
    let mut router = TachyonRouter::new(method.id(), Arc::new(wrapper));
    router.optimize_for_speed();

    // Fast write lock for route insertion
    self.get_routes().insert(route_key, router);

    Ok(())
  }
}
