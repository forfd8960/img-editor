use crate::core::export_engine::{export_image, ExportFormat};
use crate::types::errors::AppError;
use crate::state::image_state::ImageState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

/// Export parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportParams {
    pub output_path: String,
    pub format: String,
    pub quality: u8,
}

/// Export result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub path: String,
    pub file_size: u64,
    pub format: String,
}

/// Export the current image to a file
#[tauri::command]
pub async fn export_image_command(
    state: State<'_, ImageState>,
    params: ExportParams,
) -> Result<ExportResult, AppError> {
    // Validate format
    let path = PathBuf::from(&params.output_path);
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| AppError::InvalidOperation {
            details: "File path must have an extension (jpg, png, webp)".to_string(),
        })?;

    let format = ExportFormat::from_extension(extension)?;

    // Validate quality
    if params.quality == 0 || params.quality > 100 {
        return Err(AppError::InvalidOperation {
            details: format!("Quality must be 1-100, got {}", params.quality),
        });
    }

    // Get current image
    let current_arc = state.get_current()
        .ok_or_else(|| AppError::StateError {
            message: "No image loaded".to_string(),
        })?;
    let image = current_arc
        .as_ref()
        .as_ref()
        .ok_or_else(|| AppError::StateError {
            message: "No image loaded".to_string(),
        })?
        .clone();

    // Export in blocking thread
    let path_clone = path.clone();
    let quality = params.quality;
    
    let file_size = tokio::task::spawn_blocking(move || -> Result<u64, AppError> {
        export_image(&image, &path_clone, format, quality)
    })
    .await
    .map_err(|e: tokio::task::JoinError| AppError::ProcessingError {
        details: e.to_string(),
    })??;

    Ok(ExportResult {
        path: params.output_path,
        file_size,
        format: params.format,
    })
}
