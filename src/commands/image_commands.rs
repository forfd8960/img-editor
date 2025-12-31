use std::path::PathBuf;
use tauri::State;
use image::GenericImageView;

use crate::state::image_state::ImageState;
use crate::types::commands::{OpenImageInput, OpenImageOutput, ApplyOperationInput, ApplyOperationOutput};
use crate::types::errors::AppError;
use crate::types::operations::FilterType;
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

/// Apply an operation to the current image
#[tauri::command]
pub async fn apply_operation(
    input: ApplyOperationInput,
    state: State<'_, ImageState>,
) -> Result<ApplyOperationOutput, AppError> {
    // Validate filter parameters
    if let crate::types::operations::OperationType::Filter(FilterType::Blur { radius }) = &input.operation.operation {
        if *radius <= 0.0 || *radius > 100.0 {
            return Err(AppError::InvalidOperation {
                details: format!("Blur radius must be between 0 and 100, got {}", radius),
            });
        }
    }

    // Apply operation
    state.apply_operation(input.operation).await?;

    // Generate preview
    let preview_width = input.preview_width.unwrap_or(800);
    let preview_height = input.preview_width.unwrap_or(600);
    let preview_base64 = state.generate_preview(preview_width, preview_height).await?;

    // Get current image dimensions
    let current = state.get_current();
    let dimensions = current
        .and_then(|arc| arc.as_ref().as_ref().map(|img| img.dimensions()))
        .ok_or_else(|| AppError::StateError { message: "No image loaded".to_string() })?;

    Ok(ApplyOperationOutput {
        preview_base64,
        new_width: dimensions.0,
        new_height: dimensions.1,
    })
}

/// Undo the last operation
#[tauri::command]
pub async fn undo(
    state: State<'_, ImageState>,
) -> Result<ApplyOperationOutput, AppError> {
    // Perform undo
    state.undo().await?;

    // Generate preview
    let preview_base64 = state.generate_preview(800, 600).await?;

    // Get current image dimensions
    let current = state.get_current();
    let dimensions = current
        .and_then(|arc| arc.as_ref().as_ref().map(|img| img.dimensions()))
        .ok_or_else(|| AppError::StateError { message: "No image loaded".to_string() })?;

    Ok(ApplyOperationOutput {
        preview_base64,
        new_width: dimensions.0,
        new_height: dimensions.1,
    })
}

/// Redo the last undone operation
#[tauri::command]
pub async fn redo(
    state: State<'_, ImageState>,
) -> Result<ApplyOperationOutput, AppError> {
    // Perform redo
    state.redo().await?;

    // Generate preview
    let preview_base64 = state.generate_preview(800, 600).await?;

    // Get current image dimensions
    let current = state.get_current();
    let dimensions = current
        .and_then(|arc| arc.as_ref().as_ref().map(|img| img.dimensions()))
        .ok_or_else(|| AppError::StateError { message: "No image loaded".to_string() })?;

    Ok(ApplyOperationOutput {
        preview_base64,
        new_width: dimensions.0,
        new_height: dimensions.1,
    })
}
