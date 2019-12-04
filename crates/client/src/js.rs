use wasm_bindgen::prelude::wasm_bindgen;

// Imports
#[wasm_bindgen]
extern "C" {
  #[allow(unsafe_code)]
  #[wasm_bindgen(js_namespace = console)]
  pub(crate) fn log(s: &str, style: &str);

  // Custom methods
  #[allow(unsafe_code)]
  #[wasm_bindgen(js_namespace = solitaire)]
  pub(crate) fn activate_tab(id: &str, idx: usize);

  #[allow(unsafe_code)]
  #[wasm_bindgen(js_namespace = solitaire)]
  pub(crate) fn notify(level: &str, content: &str);

  #[allow(unsafe_code)]
  #[wasm_bindgen(js_namespace = solitaire)]
  pub(crate) fn show_modal(id: &str);

  #[allow(unsafe_code)]
  #[wasm_bindgen(js_namespace = solitaire)]
  pub(crate) fn wire_textarea(id: &str);
}
#[wasm_bindgen]
#[derive(Debug)]
pub struct JsClient {
  ctx: std::rc::Rc<std::sync::RwLock<crate::ctx::ClientContext>>
}

impl Default for JsClient {
  fn default() -> Self {
    Self::new()
  }
}

#[wasm_bindgen]
impl JsClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    console_error_panic_hook::set_once();

    let ctx = match crate::ctx::ClientContext::new() {
      Ok(c) => c,
      Err(e) => panic!("Cannot construct ClientContext: {}", e)
    };

    JsClient { ctx }
  }

  #[allow(unused_qualifications)]
  pub fn on_event(&self, t: &str, k: &str, v: &str) {
    let mut ctx = self.ctx.write().unwrap();
    match ctx.on_event(t, k, v) {
      Ok(_) => {}
      Err(e) => error!("Error encountered running [on_event]: {}", e)
    };
  }
}
