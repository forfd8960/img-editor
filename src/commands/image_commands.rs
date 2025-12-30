use std::path::PathBuf;
use tauri::State;

use crate::state::image_state::ImageState;
use crate::types::commands::{OpenImageInput, OpenImageOutput};
use crate::types::errors::AppError;
use crate::utils::preview::{validate_file_size, validate_format};

/// Open an image file and return metadata with preview
#[tauri::command]
pub async fn open_image(
    input: OpenImageInput,
    state: State<'_, ImageState>,
) -> Result<OpenImageOutput, AppError> {
    // Validate file path
    let path = PathBuf::from(&input.file_path);
    if !path.exists() {
        return Err(AppError::ImageLoadError(format!(
            "File not found: {}",
            input.file_path
        )));
    }

    // Validate file is a file (not directory)
    if !path.is_file() {
        return Err(AppError::ImageLoadError(format!(
            "Path is not a file: {}",
            input.file_path
        )));
    }

    // Validate format
    validate_format(&path)?;

    // Validate file size
    validate_file_size(&path)?;

    // Load original image
    let (width, height, format) = state.load_original(&path).await?;

    // Generate preview
    let preview_base64 = state
        .generate_preview(input.preview_max_width, input.preview_max_height)
        .await?;

    Ok(OpenImageOutput {
        original_width: width,
        original_height: height,
        format,
        preview_base64,
    })
}
