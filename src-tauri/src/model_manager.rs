use anyhow::{Context, Result};
use ndarray::Array4;
use ort::{session::{Session, builder::GraphOptimizationLevel}, value::Value};
use std::path::PathBuf;
use std::time::Instant;
use tauri::AppHandle;

pub struct ModelManager {
    session: Option<Session>,
    model_loaded: bool,
    use_gpu: bool,
    backend: String,
}

impl ModelManager {
    pub fn new(use_gpu: bool) -> Self {
        Self {
            session: None,
            model_loaded: false,
            use_gpu,
            backend: if use_gpu { "GPU (DirectML)".to_string() } else { "CPU".to_string() },
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.model_loaded
    }

    pub async fn load_upscaler(&mut self, _app_handle: &AppHandle) -> Result<()> {
        // Skip if already loaded
        if self.model_loaded {
            log::info!("Model already loaded, skipping reload");
            return Ok(());
        }
        
        // Try multiple locations to find the model
        let model_name = "fast.onnx";
        let possible_locations = vec![
            // Location 1: Same directory as EXE (MSI install to Program Files)
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|p| p.join("models").join(model_name))),
            
            // Location 2: Current working directory (portable version)
            Some(PathBuf::from("models").join(model_name)),
            
            // Location 3: Relative to current directory (development mode)
            Some(PathBuf::from(".").join("models").join(model_name)),
        ];
        
        // Try each location
        let mut model_path: Option<PathBuf> = None;
        for location in possible_locations.into_iter().flatten() {
            log::info!("🔍 Checking for model at: {:?}", location);
            if location.exists() {
                log::info!("✅ Model found at: {:?}", location);
                model_path = Some(location);
                break;
            }
        }
        
        // If not found, return helpful error
        let model_path = model_path.ok_or_else(|| {
            let exe_dir = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("."));
            
            anyhow::anyhow!(
                "Model file not found. Tried multiple locations:\n\
                 1. EXE directory: {:?}\n\
                 2. Current directory: ./models/{}\n\
                 3. Relative path: models/{}\n\
                 \n\
                 For MSI install: Copy models folder to C:\\Program Files\\shakticore-photo-restore\\\n\
                 For portable: Ensure models folder is next to the EXE",
                exe_dir.join("models").join(model_name),
                model_name,
                model_name
            )
        })?;

        self.load_model_from_path(&model_path).await
    }
    
    async fn load_model_from_path(&mut self, model_path: &PathBuf) -> Result<()> {
        log::info!("Loading ONNX model from: {:?}", model_path);
        let start = Instant::now();
        
        // Load ONNX model with timeout protection
        let session = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            async {
                Session::builder()?
                    .with_optimization_level(GraphOptimizationLevel::Level3)?
                    .with_intra_threads(4)?  // Increased from 1 to 4 for better performance
                    .commit_from_file(model_path)
            }
        ).await
        .map_err(|_| anyhow::anyhow!("Model loading timed out after 30 seconds"))??;

        self.session = Some(session);
        self.model_loaded = true;
        
        let duration = start.elapsed();
        log::info!("Model loaded successfully in {:.2?}", duration);
        Ok(())
    }
    
    /// Run inference on input tensor
    pub fn upscale(&mut self, input: &Array4<f32>) -> Result<Array4<f32>> {
        if !self.model_loaded {
            return Err(anyhow::anyhow!("Model not loaded"));
        }
        
        let session = self.session.as_mut().unwrap();
        log::info!("Running inference on image...");
        
        let start = Instant::now();
        
        // Create ONNX input value
        let input_value = Value::from_array(input.clone())?;
        
        // Get input name
        let input_name = session.inputs[0].name.clone();
        
        // Run inference
        let outputs = session.run(ort::inputs![input_name => input_value])?;
        
        // Get first output
        let (_, output_value) = outputs.iter().next().context("No output from model")?;
        
        // Convert to ndarray
        let (shape, data) = output_value.try_extract_tensor::<f32>()?;
        let shape_vec: Vec<usize> = shape.iter().map(|&x| x as usize).collect();
        
        if shape_vec.len() != 4 {
            return Err(anyhow::anyhow!("Expected 4D output tensor, got {:?}", shape_vec));
        }
        
        let output = Array4::from_shape_vec(
            (shape_vec[0], shape_vec[1], shape_vec[2], shape_vec[3]), 
            data.to_vec()
        )?;
        
        let duration = start.elapsed();
        log::info!("Inference completed in {:.2?}", duration);
        
        Ok(output)
    }
}
