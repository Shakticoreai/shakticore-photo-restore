// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod image_processor;
mod model_manager;

use image_processor::ImageProcessor;
use model_manager::ModelManager;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tauri::{State, AppHandle};
use walkdir::WalkDir;

struct AppState {
    model_manager: Mutex<ModelManager>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProcessRequest {
    folder_path: String,
    mode: String, // "fast" or "quality"
}

#[derive(Debug, Serialize)] // Kept Debug for consistency, removed Deserialize as it's not used for responses
struct ProcessResponse {
    success: bool,
    message: String,
    processed_count: usize,
    elapsed_seconds: f64,
    average_time_per_image: f64,
    output_folder: String,
}

#[tauri::command]
async fn process_images(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    request: ProcessRequest,
) -> Result<ProcessResponse, String> {
    // Determine model path based on mode
    let _model_path = match request.mode.as_str() {
        "fast" => "models/fast.onnx",
        "quality" => "models/quality.onnx",
        _ => {
            return Ok(ProcessResponse {
                success: false,
                message: "Invalid mode. Use 'fast' or 'quality'".to_string(),
                processed_count: 0,
                elapsed_seconds: 0.0,
                average_time_per_image: 0.0,
                output_folder: String::new(),
            })
        }
    };

    // Load model only if not already loaded
    let start_time = std::time::Instant::now();
    
    let mut model_manager = state.model_manager.lock().await;
    
    // Check if model is already loaded to avoid reloading
    if !model_manager.is_loaded() {
        log::info!("📦 Loading model for first time...");
        model_manager
            .load_upscaler(&app_handle)
            .await
            .map_err(|e| format!("Failed to load model: {}", e))?;
    } else {
        log::info!("✅ Model already loaded, reusing (much faster!)");
    }

    // Process all images in folder
    let mut processed_count = 0;
    let supported_extensions = ["jpg", "jpeg", "png", "bmp"];

    for entry in WalkDir::new(&request.folder_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| supported_extensions.contains(&ext.to_lowercase().as_str()))
                    .unwrap_or(false)
                // Skip files that already have "_enhanced" in the name
                && !e.path()
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|stem| stem.contains("_enhanced"))
                    .unwrap_or(false)
        })
    {
        let input_path = entry.path();
        
        // PROTECTION 1: Skip files larger than 10MB (likely already enhanced)
        if let Ok(metadata) = std::fs::metadata(input_path) {
            let size_mb = metadata.len() as f64 / 1_000_000.0;
            if size_mb > 10.0 {
                log::warn!("⚠️ Skipping {}: File too large ({:.1} MB). Likely already enhanced. Use original source.", 
                    input_path.display(), size_mb);
                continue;
            }
        }
        
        // PROTECTION 2: Check for multiple _enhanced in filename
        if let Some(filename) = input_path.file_name().and_then(|n| n.to_str()) {
            let enhanced_count = filename.matches("_enhanced").count();
            if enhanced_count >= 2 {
                log::warn!("⚠️ Skipping {}: Multiple enhancements detected. Use original source.", 
                    input_path.display());
                continue;
            }
        }
        
        let output_path = create_output_path(input_path);

        match process_single_image(
            &mut *model_manager,
            input_path.to_str().unwrap(),
            output_path.to_str().unwrap(),
        )
        {
            Ok(_) => {
                processed_count += 1;
                println!("Processed: {}", input_path.display());
            }
            Err(e) => {
                eprintln!("Failed to process {}: {}", input_path.display(), e);
            }
        }
    }

    let elapsed = start_time.elapsed();
    let elapsed_seconds = elapsed.as_secs_f64();
    let average_time = if processed_count > 0 {
        elapsed_seconds / processed_count as f64
    } else {
        0.0
    };
    
    log::info!("Processed {} images in {:.2} seconds (avg: {:.2}s per image)", 
        processed_count, elapsed_seconds, average_time);

    Ok(ProcessResponse {
        success: true,
        message: format!("Successfully processed {} images", processed_count),
        processed_count,
        elapsed_seconds,
        average_time_per_image: average_time,
        output_folder: request.folder_path.clone(),
    })
}

fn process_single_image(
    model: &mut ModelManager,
    input_path: &str,
    output_path: &str,
) -> anyhow::Result<()> {
    // Load and preprocess
    let input_tensor = ImageProcessor::load_and_preprocess(input_path)?;

    // Run inference
    let output_tensor = model.upscale(&input_tensor)?;

    // Postprocess and save
    let output_img = ImageProcessor::postprocess_to_image(&output_tensor)?;
    output_img.save(output_path)?;

    Ok(())
}

fn create_output_path(input_path: &std::path::Path) -> std::path::PathBuf {
    let parent = input_path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let stem = input_path.file_stem().unwrap_or_default();
    let extension = input_path.extension().unwrap_or_default();

    parent.join(format!(
        "{}_enhanced.{}",
        stem.to_string_lossy(),
        extension.to_string_lossy()
    ))
}

#[tauri::command]
fn open_output_folder(folder_path: String) -> Result<(), String> {
    std::process::Command::new("explorer")
        .arg(folder_path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            model_manager: Mutex::new(ModelManager::new(true)),
        })
        .invoke_handler(tauri::generate_handler![process_images, open_output_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
