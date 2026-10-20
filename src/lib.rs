use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

/// Configuration for rendering a D2 diagram
#[wasm_bindgen]
pub struct DiagramConfig {
    d2_source: String,
    layout: String,
    theme: i64,
    sketch: bool,
}

#[wasm_bindgen]
impl DiagramConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(d2_source: String) -> Self {
        Self {
            d2_source,
            layout: "tala".to_string(),
            theme: 0,
            sketch: false,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn d2_source(&self) -> String {
        self.d2_source.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_d2_source(&mut self, source: String) {
        self.d2_source = source;
    }

    #[wasm_bindgen(getter)]
    pub fn layout(&self) -> String {
        self.layout.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_layout(&mut self, layout: String) {
        self.layout = layout;
    }

    #[wasm_bindgen(getter)]
    pub fn theme(&self) -> i64 {
        self.theme
    }

    #[wasm_bindgen(setter)]
    pub fn set_theme(&mut self, theme: i64) {
        self.theme = theme;
    }

    #[wasm_bindgen(getter)]
    pub fn sketch(&self) -> bool {
        self.sketch
    }

    #[wasm_bindgen(setter)]
    pub fn set_sketch(&mut self, sketch: bool) {
        self.sketch = sketch;
    }
}

/// Result from rendering a diagram
#[wasm_bindgen]
pub struct RenderResult {
    svg: String,
    error: String,
}

#[wasm_bindgen]
impl RenderResult {
    #[wasm_bindgen(getter)]
    pub fn svg(&self) -> String {
        self.svg.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn error(&self) -> String {
        self.error.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn has_error(&self) -> bool {
        !self.error.is_empty()
    }
}

/// Helper: safely call dispose on a D2 instance
fn dispose_d2(d2: &JsValue) {
    if let Ok(dispose_val) = js_sys::Reflect::get(d2, &"dispose".into()) {
        if let Ok(dispose_fn) = dispose_val.dyn_into::<js_sys::Function>() {
            let _ = dispose_fn.call0(d2);
        }
    }
}

/// Convenience function: render D2 source to SVG directly
/// This creates a temporary D2 instance, renders, and returns the SVG
#[wasm_bindgen]
pub async fn render_d2_to_svg(d2_source: String, layout: String) -> RenderResult {
    let global = js_sys::global();

    let has_d2 = match js_sys::Reflect::has(&global, &"D2".into()) {
        Ok(v) => v,
        Err(_) => false,
    };
    if !has_d2 {
        return RenderResult {
            svg: String::new(),
            error: "D2 library not found. Include @d2lang/d2 before this module.".to_string(),
        };
    }

    let d2_class = match js_sys::Reflect::get(&global, &"D2".into()) {
        Ok(v) => v,
        Err(e) => {
            return RenderResult {
                svg: String::new(),
                error: format!("Failed to get D2 class: {:?}", e),
            }
        }
    };

    // Cast d2_class to Function for Reflect::construct
    let d2_fn: js_sys::Function = match d2_class.dyn_into() {
        Ok(v) => v,
        Err(_) => {
            return RenderResult {
                svg: String::new(),
                error: "D2 is not a constructor".to_string(),
            }
        }
    };

    // Use Reflect::construct to call `new D2()`
    let d2 = match js_sys::Reflect::construct(&d2_fn, &js_sys::Array::new()) {
        Ok(v) => v,
        Err(e) => {
            return RenderResult {
                svg: String::new(),
                error: format!("Failed to create D2 instance: {:?}", e),
            }
        }
    };

    // Build options object
    let options_obj = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&options_obj, &"layout".into(), &layout.into());

    // Call compile
    let compile_fn_val = match js_sys::Reflect::get(&d2, &"compile".into()) {
        Ok(v) => v,
        Err(e) => {
            dispose_d2(&d2);
            return RenderResult {
                svg: String::new(),
                error: format!("Failed to get compile method: {:?}", e),
            }
        }
    };

    let compile_fn: js_sys::Function = match compile_fn_val.dyn_into() {
        Ok(v) => v,
        Err(_) => {
            dispose_d2(&d2);
            return RenderResult {
                svg: String::new(),
                error: "compile is not a function".to_string(),
            }
        }
    };

    let compile_promise: js_sys::Promise = match compile_fn.call2(&d2, &d2_source.into(), &options_obj) {
        Ok(v) => v.into(),
        Err(e) => {
            dispose_d2(&d2);
            return RenderResult {
                svg: String::new(),
                error: format!("Failed to call compile: {:?}", e),
            }
        }
    };

    let compile_result = match JsFuture::from(compile_promise).await {
        Ok(result) => result,
        Err(e) => {
            dispose_d2(&d2);
            return RenderResult {
                svg: String::new(),
                error: format!("Compile error: {:?}", e),
            };
        }
    };

    let diagram = match js_sys::Reflect::get(&compile_result, &"diagram".into()) {
        Ok(v) => v,
        Err(e) => {
            dispose_d2(&d2);
            return RenderResult {
                svg: String::new(),
                error: format!("Missing diagram in compile result: {:?}", e),
            }
        }
    };

    let render_options = match js_sys::Reflect::get(&compile_result, &"renderOptions".into()) {
        Ok(v) => v,
        Err(e) => {
            dispose_d2(&d2);
            return RenderResult {
                svg: String::new(),
                error: format!("Missing renderOptions in compile result: {:?}", e),
            }
        }
    };

    // Call render
    let render_fn_val = match js_sys::Reflect::get(&d2, &"render".into()) {
        Ok(v) => v,
        Err(e) => {
            dispose_d2(&d2);
            return RenderResult {
                svg: String::new(),
                error: format!("Failed to get render method: {:?}", e),
            }
        }
    };

    let render_fn: js_sys::Function = match render_fn_val.dyn_into() {
        Ok(v) => v,
        Err(_) => {
            dispose_d2(&d2);
            return RenderResult {
                svg: String::new(),
                error: "render is not a function".to_string(),
            }
        }
    };

    let render_promise: js_sys::Promise = match render_fn.call2(&d2, &diagram, &render_options) {
        Ok(v) => v.into(),
        Err(e) => {
            dispose_d2(&d2);
            return RenderResult {
                svg: String::new(),
                error: format!("Failed to call render: {:?}", e),
            }
        }
    };

    let svg_result = match JsFuture::from(render_promise).await {
        Ok(result) => result,
        Err(e) => {
            dispose_d2(&d2);
            return RenderResult {
                svg: String::new(),
                error: format!("Render error: {:?}", e),
            };
        }
    };

    dispose_d2(&d2);

    let svg = svg_result.as_string().unwrap_or_default();
    console::log_1(&"Diagram rendered successfully".into());

    RenderResult {
        svg,
        error: String::new(),
    }
}

/// Returns a sample D2 diagram source for testing
#[wasm_bindgen]
pub fn sample_diagram() -> String {
    r#"cloud: {
  shape: cloud
  Cloud Provider
}
server: {
  shape: cylinder
  Application Server
}
database: {
  shape: database
  Database
}

cloud -> server
server -> database"#.to_string()
}

/// Returns a sample architecture diagram
#[wasm_bindgen]
pub fn sample_architecture() -> String {
    r#"Frontend: {shape: hexagon}
API Gateway: {shape: parallelogram}
Auth Service
User Service
Payment Service
Database: {shape: cylinder}
Cache: {shape: stored_data}

Frontend -> API Gateway
API Gateway -> Auth Service
API Gateway -> User Service
API Gateway -> Payment Service
Auth Service -> Database
User Service -> Database
Payment Service -> Database
User Service -> Cache"#.to_string()
}

/// Returns a sample sequence diagram
#[wasm_bindgen]
pub fn sample_sequence() -> String {
    r#"sequence: {
  shape: sequence_diagram
  User -> API
  API -> Database
  Database -> API: Result
  API -> User: Response
}"#.to_string()
}
