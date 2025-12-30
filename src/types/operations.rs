use serde::{Deserialize, Serialize};

/// Operation type enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
    Filter,
    Adjustment,
    Transform,
    Crop,
}

/// Filter types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterType {
    Grayscale,
    Sepia,
    Invert,
    Blur,
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

/// Operation parameters (tagged union)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum OperationParams {
    Filter {
        filter_type: FilterType,
        intensity: f32,
    },
    Adjustment(AdjustmentParams),
    Transform(TransformType),
    Crop {
        rect: CropRect,
        maintain_aspect_ratio: Option<f32>,
    },
}

/// Edit operation structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditOperation {
    pub id: String,
    #[serde(rename = "type")]
    pub operation_type: OperationType,
    #[serde(flatten)]
    pub params: OperationParams,
}
