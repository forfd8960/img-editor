use super::operations::EditOperation;
use serde::{Deserialize, Serialize};

/// Open image command input
#[derive(Debug, Deserialize)]
pub struct OpenImageInput {
    pub file_path: String,
    pub preview_max_width: u32,
    pub preview_max_height: u32,
}

/// Open image command output
#[derive(Debug, Serialize)]
pub struct OpenImageOutput {
    pub preview_base64: String,
    pub original_width: u32,
    pub original_height: u32,
    pub format: String,
}

/// Apply operation command input
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ApplyOperationInput {
    pub operation: EditOperation,
    pub preview_width: Option<u32>,
}

/// Apply operation command output
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ApplyOperationOutput {
    pub preview_base64: String,
    pub new_width: u32,
    pub new_height: u32,
}

/// Preview command input
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PreviewInput {
    pub operations: Vec<EditOperation>,
    pub max_width: u32,
    pub max_height: u32,
}

/// Preview command output
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct PreviewOutput {
    pub preview_base64: String,
    pub width: u32,
    pub height: u32,
}

/// Export image command input
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ExportInput {
    pub output_path: String,
    pub operations: Vec<EditOperation>,
    pub format: String,
    pub quality: Option<u8>,
}

/// Export image command output
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ExportOutput {
    pub success: bool,
    pub output_path: String,
    pub file_size: u64,
}

/// History state output
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct HistoryState {
    pub can_undo: bool,
    pub can_redo: bool,
    pub history_count: usize,
    pub redo_count: usize,
}

/// Clear output
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ClearOutput {
    pub success: bool,
}
