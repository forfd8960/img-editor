use serde::Serialize;
use thiserror::Error;

/// Application error types
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("图像加载失败: {0}")]
    ImageLoadError(String),

    #[error("图像保存失败: {0}")]
    ImageSaveError(String),

    #[error("不支持的图像格式: {format}")]
    UnsupportedFormat { format: String },

    #[error("文件访问被拒绝: {path}")]
    FileAccessDenied { path: String },

    #[error("无效的操作参数: {details}")]
    InvalidOperation { details: String },

    #[error("内存不足: 需要 {required} bytes, 可用 {available} bytes")]
    OutOfMemory { required: u64, available: u64 },

    #[error("图像处理错误: {details}")]
    ProcessingError { details: String },

    #[error("状态错误: {message}")]
    StateError { message: String },
}

// Implement Serialize for Tauri IPC
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("AppError", 2)?;
        
        match self {
            AppError::ImageLoadError(source) => {
                s.serialize_field("type", "image_load_error")?;
                s.serialize_field("message", source)?;
            }
            AppError::ImageSaveError(source) => {
                s.serialize_field("type", "image_save_error")?;
                s.serialize_field("message", source)?;
            }
            AppError::UnsupportedFormat { format } => {
                s.serialize_field("type", "unsupported_format")?;
                s.serialize_field("message", format)?;
            }
            AppError::FileAccessDenied { path } => {
                s.serialize_field("type", "file_access_denied")?;
                s.serialize_field("message", path)?;
            }
            AppError::InvalidOperation { details } => {
                s.serialize_field("type", "invalid_operation")?;
                s.serialize_field("message", details)?;
            }
            AppError::OutOfMemory { required, available } => {
                s.serialize_field("type", "out_of_memory")?;
                s.serialize_field("required", required)?;
                s.serialize_field("available", available)?;
            }
            AppError::ProcessingError { details } => {
                s.serialize_field("type", "processing_error")?;
                s.serialize_field("message", details)?;
            }
            AppError::StateError { message } => {
                s.serialize_field("type", "state_error")?;
                s.serialize_field("message", message)?;
            }
        }
        s.end()
    }
}
