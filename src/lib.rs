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
/// Note: D2 class arrays use semicolons to separate items (e.g., `class: [a; b]`)
#[wasm_bindgen]
pub fn sample_diagram() -> String {
    "classes: {\n  premium-base: {\n    style: {\n      font-size: 14\n      bold: true\n      shadow: true\n      border-radius: 8\n      stroke-width: 2\n    }\n  }\n  tier-client: {\n    style: {\n      fill: \"#DBEAFE\"\n      stroke: \"#3B82F6\"\n      font-color: \"#1E40AF\"\n    }\n  }\n  tier-service: {\n    style: {\n      fill: \"#F3F4F6\"\n      stroke: \"#6B7280\"\n      font-color: \"#374151\"\n    }\n  }\n  tier-data: {\n    style: {\n      fill: \"#EDE9FE\"\n      stroke: \"#8B5CF6\"\n      font-color: \"#5B21B6\"\n    }\n  }\n}\n\nstyle: {\n  fill: \"#FFFFFF\"\n}\n\ncloud: {\n  shape: cloud\n  class: [premium-base; tier-client]\n  style: {\n    border-radius: 12\n  }\n  Cloud Provider\n}\n\nserver: {\n  shape: cylinder\n  class: [premium-base; tier-service]\n  Application Server\n}\n\ndatabase: {\n  shape: cylinder\n  class: [premium-base; tier-data]\n  Database\n}\n\ncloud -> server: {\n  style: {\n    stroke: \"#3B82F6\"\n    stroke-width: 2\n  }\n}\n\nserver -> database: {\n  style: {\n    stroke: \"#8B5CF6\"\n    stroke-width: 2\n  }\n}".to_string()
}

/// Returns a sample architecture diagram
#[wasm_bindgen]
pub fn sample_architecture() -> String {
    "classes: {\n  premium-base: {\n    style: {\n      font-size: 14\n      bold: true\n      shadow: true\n      border-radius: 8\n      stroke-width: 2\n    }\n  }\n  tier-client: {\n    style: {\n      fill: \"#DBEAFE\"\n      stroke: \"#3B82F6\"\n      font-color: \"#1E40AF\"\n    }\n  }\n  tier-gateway: {\n    style: {\n      fill: \"#FEF3C7\"\n      stroke: \"#D97706\"\n      font-color: \"#92400E\"\n    }\n  }\n  tier-service: {\n    style: {\n      fill: \"#F3F4F6\"\n      stroke: \"#6B7280\"\n      font-color: \"#374151\"\n    }\n  }\n  tier-data: {\n    style: {\n      fill: \"#EDE9FE\"\n      stroke: \"#8B5CF6\"\n      font-color: \"#5B21B6\"\n    }\n  }\n  tier-cache: {\n    style: {\n      fill: \"#DCFCE7\"\n      stroke: \"#16A34A\"\n      font-color: \"#166534\"\n    }\n  }\n}\n\nstyle: {\n  fill: \"#FFFFFF\"\n}\n\nFrontend: {\n  shape: hexagon\n  class: [premium-base; tier-client]\n}\n\nAPI Gateway: {\n  shape: parallelogram\n  class: [premium-base; tier-gateway]\n}\n\nAuth Service: {\n  class: [premium-base; tier-service]\n}\n\nUser Service: {\n  class: [premium-base; tier-service]\n}\n\nPayment Service: {\n  class: [premium-base; tier-service]\n}\n\nDatabase: {\n  shape: cylinder\n  class: [premium-base; tier-data]\n}\n\nCache: {\n  shape: stored_data\n  class: [premium-base; tier-cache]\n}\n\nFrontend -> API Gateway: {\n  style: {\n    stroke: \"#3B82F6\"\n    stroke-width: 3\n  }\n}\n\nAPI Gateway -> Auth Service: {\n  style: {\n    stroke: \"#D97706\"\n    stroke-width: 2\n  }\n}\n\nAPI Gateway -> User Service: {\n  style: {\n    stroke: \"#D97706\"\n    stroke-width: 2\n  }\n}\n\nAPI Gateway -> Payment Service: {\n  style: {\n    stroke: \"#D97706\"\n    stroke-width: 2\n  }\n}\n\nAuth Service -> Database: {\n  style: {\n    stroke: \"#8B5CF6\"\n    stroke-width: 2\n  }\n}\n\nUser Service -> Database: {\n  style: {\n    stroke: \"#8B5CF6\"\n    stroke-width: 2\n  }\n}\n\nPayment Service -> Database: {\n  style: {\n    stroke: \"#8B5CF6\"\n    stroke-width: 2\n  }\n}\n\nUser Service -> Cache: {\n  style: {\n    stroke: \"#16A34A\"\n    stroke-width: 2\n  }\n}".to_string()
}

/// Returns a sample sequence diagram
#[wasm_bindgen]
pub fn sample_sequence() -> String {
    "classes: {\n  premium-base: {\n    style: {\n      font-size: 14\n      bold: true\n      stroke-width: 2\n    }\n  }\n  tier-client: {\n    style: {\n      fill: \"#DBEAFE\"\n      stroke: \"#3B82F6\"\n      font-color: \"#1E40AF\"\n    }\n  }\n  tier-gateway: {\n    style: {\n      fill: \"#FEF3C7\"\n      stroke: \"#D97706\"\n      font-color: \"#92400E\"\n    }\n  }\n  tier-data: {\n    style: {\n      fill: \"#EDE9FE\"\n      stroke: \"#8B5CF6\"\n      font-color: \"#5B21B6\"\n    }\n  }\n}\n\nstyle: {\n  fill: \"#FFFFFF\"\n}\n\nsequence: {\n  shape: sequence_diagram\n  \n  User: {\n    class: [premium-base; tier-client]\n  }\n  \n  API: {\n    class: [premium-base; tier-gateway]\n  }\n  \n  Database: {\n    class: [premium-base; tier-data]\n  }\n  \n  User -> API: {\n    style: {\n      stroke: \"#3B82F6\"\n      stroke-width: 2\n    }\n  }\n  \n  API -> Database: {\n    style: {\n      stroke: \"#D97706\"\n      stroke-width: 2\n    }\n  }\n  \n  Database -> API: Result: {\n    style: {\n      stroke: \"#8B5CF6\"\n      stroke-width: 2\n      stroke-dash: 4\n    }\n  }\n  \n  API -> User: Response: {\n    style: {\n      stroke: \"#3B82F6\"\n      stroke-width: 2\n      stroke-dash: 4\n    }\n  }\n}".to_string()
}

/// Returns the app version from Cargo.toml
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Returns a sample AWS architecture diagram
#[wasm_bindgen]
pub fn sample_aws() -> String {
    "classes: {\n  premium-base: {\n    style: {\n      font-size: 14\n      bold: true\n      shadow: true\n      border-radius: 8\n      stroke-width: 2\n    }\n  }\n  tier-compute: {\n    style: {\n      fill: \"#DBEAFE\"\n      stroke: \"#3B82F6\"\n      font-color: \"#1E40AF\"\n    }\n  }\n  tier-serverless: {\n    style: {\n      fill: \"#FEF3C7\"\n      stroke: \"#D97706\"\n      font-color: \"#92400E\"\n    }\n  }\n  tier-messaging: {\n    style: {\n      fill: \"#F3F4F6\"\n      stroke: \"#6B7280\"\n      font-color: \"#374151\"\n    }\n  }\n  tier-storage: {\n    style: {\n      fill: \"#CCFBF1\"\n      stroke: \"#0D9488\"\n      font-color: \"#134E4A\"\n    }\n  }\n  tier-data: {\n    style: {\n      fill: \"#EDE9FE\"\n      stroke: \"#8B5CF6\"\n      font-color: \"#5B21B6\"\n    }\n  }\n  tier-gateway: {\n    style: {\n      fill: \"#FEF3C7\"\n      stroke: \"#D97706\"\n      font-color: \"#92400E\"\n    }\n  }\n  tier-ml: {\n    style: {\n      fill: \"#FCE7F3\"\n      stroke: \"#DB2777\"\n      font-color: \"#831843\"\n    }\n  }\n  tier-monitoring: {\n    style: {\n      fill: \"#F3F4F6\"\n      stroke: \"#6B7280\"\n      font-color: \"#374151\"\n    }\n  }\n  tier-danger: {\n    style: {\n      fill: \"#FEE2E2\"\n      stroke: \"#DC2626\"\n      font-color: \"#7F1D1D\"\n    }\n  }\n}\n\nstyle: {\n  fill: \"#FFFFFF\"\n}\n\ndirection: right\n\n# Top left section\nEC2_Instance: {\n  label: \"EC2 Instance\"\n  class: [premium-base; tier-compute]\n  \n  IngestionApp: \"Ingestion App\" {\n    class: [premium-base; tier-compute]\n  }\n  ChunkingApp: \"Chunking Orchestration App\" {\n    class: [premium-base; tier-compute]\n  }\n}\n\nS3Staging: \"Amazon S3 Staging Bucket\" {\n  class: [premium-base; tier-storage]\n  shape: page\n}\n\nSQSIngest: \"Amazon SQS Ingest Queue\" {\n  class: [premium-base; tier-messaging]\n  shape: queue\n}\n\nLambdaTrigger: \"AWS Lambda Trigger Function\" {\n  class: [premium-base; tier-serverless]\n  shape: hexagon\n}\n\nLambdaTrigger -> EC2_Instance.IngestionApp: {\n  style: {\n    stroke: \"#D97706\"\n    stroke-width: 2\n  }\n}\n\nEC2_Instance.IngestionApp -> SQSIngest: {\n  style: {\n    stroke: \"#3B82F6\"\n    stroke-width: 2\n  }\n}\n\nSQSIngest -> EC2_Instance.ChunkingApp: {\n  style: {\n    stroke: \"#6B7280\"\n    stroke-width: 2\n  }\n}\n\nS3Staging -> EC2_Instance: {\n  style: {\n    stroke: \"#0D9488\"\n    stroke-width: 2\n  }\n}\n\n# Top right section\nSQSLargeFileQueue: \"Amazon SQS Large File Queue\" {\n  class: [premium-base; tier-messaging]\n  shape: queue\n}\n\nSQSSmallFileQueue: \"Amazon SQS Small File Queue\" {\n  class: [premium-base; tier-messaging]\n  shape: queue\n}\n\nSQSSingleFileQueue: \"Amazon SQS Single File Queue\" {\n  class: [premium-base; tier-messaging]\n  shape: queue\n}\n\nLargeFileChunkingAppEC2: \"Large File Chunking App on EC2\" {\n  class: [premium-base; tier-compute]\n  shape: rectangle\n}\n\nLambdaSmallFileChunking: \"Small File Chunking Lambda\" {\n  class: [premium-base; tier-serverless]\n  shape: hexagon\n}\n\nImageConversionLambdaFunction: \"Image Conversion Lambda Function\" {\n  class: [premium-base; tier-serverless]\n  shape: hexagon\n}\n\nDLQ: \"DLQ\" {\n  class: [premium-base; tier-danger]\n  style.stroke-dash: 5\n  \n  DLQ1: \"DLQ\" {\n    class: [premium-base; tier-danger]\n    shape: queue\n  }\n  DLQ2: \"DLQ\" {\n    class: [premium-base; tier-danger]\n    shape: queue\n  }\n  DLQ3: \"DLQ\" {\n    class: [premium-base; tier-danger]\n    shape: queue\n  }\n}\n\nEC2_Instance.ChunkingApp -> SQSLargeFileQueue: {\n  style: {\n    stroke: \"#3B82F6\"\n    stroke-width: 2\n  }\n}\n\nEC2_Instance.ChunkingApp -> SQSSmallFileQueue: {\n  style: {\n    stroke: \"#3B82F6\"\n    stroke-width: 2\n  }\n}\n\nEC2_Instance.ChunkingApp -> SQSSingleFileQueue: {\n  style: {\n    stroke: \"#3B82F6\"\n    stroke-width: 2\n  }\n}\n\nSQSLargeFileQueue -> LargeFileChunkingAppEC2: {\n  style: {\n    stroke: \"#6B7280\"\n    stroke-width: 2\n  }\n}\n\nLargeFileChunkingAppEC2 -> SQSSmallFileQueue: {\n  style: {\n    stroke: \"#6B7280\"\n    stroke-width: 2\n  }\n}\n\nSQSSmallFileQueue -> LambdaSmallFileChunking: {\n  style: {\n    stroke: \"#6B7280\"\n    stroke-width: 2\n  }\n}\n\nLambdaSmallFileChunking -> SQSSingleFileQueue: {\n  style: {\n    stroke: \"#D97706\"\n    stroke-width: 2\n  }\n}\n\nSQSSingleFileQueue -> ImageConversionLambdaFunction: {\n  style: {\n    stroke: \"#6B7280\"\n    stroke-width: 2\n  }\n}\n\nSQSLargeFileQueue -> DLQ.DLQ1: {\n  style: {\n    stroke: \"#DC2626\"\n    stroke-width: 2\n    stroke-dash: 4\n  }\n}\n\nSQSSmallFileQueue -> DLQ.DLQ2: {\n  style: {\n    stroke: \"#DC2626\"\n    stroke-width: 2\n    stroke-dash: 4\n  }\n}\n\nSQSSingleFileQueue -> DLQ.DLQ3: {\n  style: {\n    stroke: \"#DC2626\"\n    stroke-width: 2\n    stroke-dash: 4\n  }\n}\n\n# Bottom right section\nS3Images: \"Amazon S3 Images Bucket\" {\n  class: [premium-base; tier-storage]\n  shape: page\n}\n\nEC2DLQFailsafeApp: \"DLQ Failsafe App on EC2\" {\n  class: [premium-base; tier-compute]\n  shape: rectangle\n}\n\nSQSConvertedImageQueue: \"Amazon SQS Converted Image Queue\" {\n  class: [premium-base; tier-messaging]\n  shape: queue\n}\n\nLambdaInferenceInvocation: \"Inference Invocation Lambda Function\" {\n  class: [premium-base; tier-serverless]\n  shape: hexagon\n}\n\nAurora: \"Amazon Aurora\" {\n  class: [premium-base; tier-data]\n  shape: cylinder\n}\n\nAmazonAPIGateway: \"Amazon API Gateway\" {\n  class: [premium-base; tier-gateway]\n  shape: rectangle\n}\n\nAmazonSageMaker: \"Amazon SageMaker Endpoint\" {\n  class: [premium-base; tier-ml]\n  shape: rectangle\n}\n\nDLQ.DLQ1 -> EC2DLQFailsafeApp: {\n  style: {\n    stroke: \"#DC2626\"\n    stroke-width: 2\n  }\n}\n\nDLQ.DLQ2 -> EC2DLQFailsafeApp: {\n  style: {\n    stroke: \"#DC2626\"\n    stroke-width: 2\n  }\n}\n\nDLQ.DLQ3 -> EC2DLQFailsafeApp: {\n  style: {\n    stroke: \"#DC2626\"\n    stroke-width: 2\n  }\n}\n\nEC2DLQFailsafeApp -> S3Images: {\n  style: {\n    stroke: \"#3B82F6\"\n    stroke-width: 2\n  }\n}\n\nImageConversionLambdaFunction -> S3Images: {\n  style: {\n    stroke: \"#D97706\"\n    stroke-width: 2\n  }\n}\n\nS3Images -> SQSConvertedImageQueue: {\n  style: {\n    stroke: \"#0D9488\"\n    stroke-width: 2\n  }\n}\n\nSQSConvertedImageQueue -> LambdaInferenceInvocation: {\n  style: {\n    stroke: \"#6B7280\"\n    stroke-width: 2\n  }\n}\n\nLambdaInferenceInvocation -> Aurora: {\n  style: {\n    stroke: \"#8B5CF6\"\n    stroke-width: 2\n  }\n}\n\n# Bottom left section\nCloudWatch: \"Amazon CloudWatch\" {\n  class: [premium-base; tier-monitoring]\n  shape: rectangle\n}\n\nSNS1: \"Amazon SNS\" {\n  class: [premium-base; tier-monitoring]\n  shape: circle\n}\n\nSESEmail: \"Email Notification\" {\n  class: [premium-base; tier-monitoring]\n  shape: rectangle\n}\n\nCloudWatch -> DLQ: {\n  style: {\n    stroke: \"#6B7280\"\n    stroke-width: 2\n  }\n}\n\nCloudWatch -> SNS1: {\n  style: {\n    stroke: \"#6B7280\"\n    stroke-width: 2\n  }\n}\n\nSNS1 -> SESEmail: {\n  style: {\n    stroke: \"#6B7280\"\n    stroke-width: 2\n  }\n}\n".to_string()
}
