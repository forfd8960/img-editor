use serde::{Deserialize, Serialize};

/// Operation parameters (tagged union)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation_type", content = "params", rename_all = "PascalCase")]
pub enum OperationType {
    Filter(FilterType),
    Adjustment(AdjustmentParams),
    Transform(TransformType),
    Crop(CropRect),
}

/// Filter types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FilterType {
    Grayscale,
    Sepia,
    Invert,
    Blur { radius: f32 },
    Sharpen,
}

/// Adjustment parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdjustmentParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contrast: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saturation: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hue: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gamma: Option<f32>,
}

/// Transform types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransformType {
    Rotate90,
    Rotate180,
    Rotate270,
    FlipHorizontal,
    FlipVertical,
}

/// Crop rectangle
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CropRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Edit operation structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditOperation {
    pub id: String,
    pub operation: OperationType,
    pub timestamp: i64,
}
