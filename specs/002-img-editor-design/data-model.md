# Data Model: img-editor

**Date**: 2025-12-30  
**Status**: Complete

## Overview

This document defines all data structures, types, and their relationships for the img-editor application. The data model is designed around non-destructive editing principles with immutable original images and operation sequences.

---

## Core Entities

### 1. EditOperation

The fundamental unit representing a single editing action.

**Rust Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditOperation {
    pub id: String,                    // Unique identifier (UUID)
    #[serde(rename = "type")]
    pub operation_type: OperationType, // Category of operation
    #[serde(flatten)]
    pub params: OperationParams,       // Operation-specific parameters
}
```

**TypeScript Equivalent**:
```typescript
interface EditOperation {
  id: string;
  type: OperationType;
  op: string;
  // Additional fields based on operation type
}
```

**Relationships**:
- Belongs to: HistoryManager (as part of operation stack)
- Used by: ImageState (for rendering)
- Serialized to: Frontend via Tauri IPC

**Validation Rules**:
- `id` must be non-empty UUID v4
- `operation_type` must be valid enum variant
- `params` must match operation type

**State Transitions**:
1. Created → Added to history stack
2. In history → Can be undone (moved to redo stack)
3. In redo → Can be redone (moved back to history)
4. Applied → Rendered in preview or export

---

### 2. OperationType

Categorizes operations into functional groups.

**Rust Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OperationType {
    #[serde(rename = "filter")]
    Filter,
    #[serde(rename = "adjustment")]
    Adjustment,
    #[serde(rename = "transform")]
    Transform,
    #[serde(rename = "crop")]
    Crop,
}
```

**TypeScript Equivalent**:
```typescript
type OperationType = "filter" | "adjustment" | "transform" | "crop";
```

**Categories**:
- **Filter**: Non-parametric effects (grayscale, sepia, blur)
- **Adjustment**: Parametric modifications (brightness, contrast, saturation)
- **Transform**: Geometric operations (rotate, flip)
- **Crop**: Region selection and extraction

---

### 3. OperationParams

Tagged union of all possible operation parameters.

**Rust Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum OperationParams {
    Filter {
        filter_type: FilterType,
        intensity: f32,              // 0.0 - 1.0
    },
    Adjustment(AdjustmentParams),
    Transform(TransformType),
    Crop {
        rect: CropRect,
        maintain_aspect_ratio: Option<f32>, // Optional aspect ratio
    },
}
```

**TypeScript Equivalent**:
```typescript
type OperationParams =
  | { op: "filter"; filter_type: FilterType; intensity: number }
  | { op: "adjustment"; brightness?: number; contrast?: number; saturation?: number; hue?: number; gamma?: number }
  | { op: "transform"; type: TransformType }
  | { op: "crop"; rect: CropRect; maintain_aspect_ratio?: number };
```

**Validation**:
- Filter intensity: 0.0 ≤ value ≤ 1.0
- Adjustment values: See AdjustmentParams
- Transform: Must be valid enum
- Crop rect: Must be within image bounds

---

### 4. FilterType

Specific filter operations.

**Rust Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterType {
    Grayscale,
    Sepia,
    Invert,
    Blur,
    Sharpen,
}
```

**TypeScript Equivalent**:
```typescript
type FilterType = "grayscale" | "sepia" | "invert" | "blur" | "sharpen";
```

**Characteristics**:
- **Grayscale**: Converts to single-channel luminance
- **Sepia**: Warm brownish tone
- **Invert**: Negates all color values
- **Blur**: Gaussian blur (radius controlled by intensity)
- **Sharpen**: Edge enhancement

---

### 5. AdjustmentParams

Parameters for color/tone adjustments.

**Rust Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdjustmentParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f32>,  // 0.0 - 2.0 (1.0 = no change)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contrast: Option<f32>,    // 0.0 - 2.0 (1.0 = no change)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saturation: Option<f32>,  // 0.0 - 2.0 (1.0 = no change)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hue: Option<i32>,         // -180 to +180 degrees
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gamma: Option<f32>,       // 0.1 - 3.0 (1.0 = no change)
}
```

**TypeScript Equivalent**:
```typescript
interface AdjustmentParams {
  brightness?: number;  // 0.0 - 2.0
  contrast?: number;    // 0.0 - 2.0
  saturation?: number;  // 0.0 - 2.0
  hue?: number;         // -180 to +180
  gamma?: number;       // 0.1 - 3.0
}
```

**Validation Rules**:
- All parameters are optional
- At least one parameter must be present
- Values must be within specified ranges

---

### 6. TransformType

Geometric transformation operations.

**Rust Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransformType {
    Rotate90,
    Rotate180,
    Rotate270,
    FlipHorizontal,
    FlipVertical,
}
```

**TypeScript Equivalent**:
```typescript
type TransformType =
  | { type: "rotate90" }
  | { type: "rotate180" }
  | { type: "rotate270" }
  | { type: "flip_horizontal" }
  | { type: "flip_vertical" };
```

**Properties**:
- All transforms are lossless (no resampling)
- Can be combined in sequence
- Affect image dimensions (rotations)

---

### 7. CropRect

Defines a rectangular region for cropping.

**Rust Definition**:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CropRect {
    pub x: u32,      // Top-left X coordinate
    pub y: u32,      // Top-left Y coordinate
    pub width: u32,  // Width in pixels
    pub height: u32, // Height in pixels
}
```

**TypeScript Equivalent**:
```typescript
interface CropRect {
  x: number;
  y: number;
  width: number;
  height: number;
}
```

**Validation Rules**:
- All values must be positive
- `x + width` ≤ image width
- `y + height` ≤ image height
- Minimum dimensions: 1x1 pixel

---

## State Management Entities

### 8. ImageState

Primary state container for the application.

**Rust Definition**:
```rust
pub struct ImageState {
    /// Original image (immutable, shared)
    original_image: ArcSwap<Option<DynamicImage>>,
    
    /// Current preview image
    current_image: ArcSwap<Option<DynamicImage>>,
    
    /// History manager
    history: Arc<HistoryManager>,
    
    /// Preview cache
    preview_cache: ArcSwap<PreviewCache>,
}
```

**Relationships**:
- Owns: Original and current images
- References: HistoryManager (shared via Arc)
- Uses: PreviewCache

**Operations**:
- `load_original(path)`: Load image from file system
- `apply_operation(op)`: Add operation to history and re-render
- `generate_preview(max_width, max_height)`: Create scaled preview
- `undo()`: Remove last operation
- `redo()`: Re-apply undone operation

**Concurrency Model**:
- ArcSwap for lock-free reads of images
- Arc for shared history manager
- All operations are async

---

### 9. HistoryManager

Manages undo/redo stacks.

**Rust Definition**:
```rust
pub struct HistoryManager {
    history: Mutex<Vec<EditOperation>>,     // Undo stack
    redo_stack: Mutex<Vec<EditOperation>>,  // Redo stack
    max_history: usize,                      // 50 operations
}
```

**State Transitions**:
```
New Operation → Push to history, clear redo_stack
Undo → Pop from history, push to redo_stack
Redo → Pop from redo_stack, push to history
Clear → Empty both stacks
```

**Invariants**:
- History size ≤ max_history
- Operations maintain chronological order
- Redo stack cleared on new operation

---

### 10. PreviewCache

Caches generated previews to avoid redundant work.

**Rust Definition**:
```rust
#[derive(Debug, Clone)]
struct PreviewCache {
    base64: String,           // Cached Base64 preview
    width: u32,               // Preview dimensions
    height: u32,
    operations_hash: u64,     // Hash of operation sequence
}
```

**Cache Strategy**:
- Hash operations to detect changes
- Invalidate on any operation modification
- Single-entry cache (most recent preview)

**Memory Trade-off**:
- ~1-2MB storage for 1920px preview
- Saves 50-200ms generation time

---

## Command I/O Types

### 11. OpenImageInput / OpenImageOutput

**Input**:
```rust
#[derive(Debug, Deserialize)]
pub struct OpenImageInput {
    pub path: String,  // Absolute file path
}
```

**Output**:
```rust
#[derive(Debug, Serialize)]
pub struct OpenImageOutput {
    pub preview_base64: String,  // Base64-encoded preview
    pub original_width: u32,     // Original dimensions
    pub original_height: u32,
    pub format: String,          // Image format (e.g., "Rgb8")
}
```

---

### 12. ApplyOperationInput / ApplyOperationOutput

**Input**:
```rust
#[derive(Debug, Deserialize)]
pub struct ApplyOperationInput {
    pub operation: EditOperation,  // Operation to apply
    pub preview_width: Option<u32>, // Optional preview constraint
}
```

**Output**:
```rust
#[derive(Debug, Serialize)]
pub struct ApplyOperationOutput {
    pub preview_base64: String,  // Updated preview
    pub new_width: u32,          // New dimensions (may change with crop/rotate)
    pub new_height: u32,
}
```

---

### 13. PreviewInput / PreviewOutput

**Input**:
```rust
#[derive(Debug, Deserialize)]
pub struct PreviewInput {
    pub operations: Vec<EditOperation>,  // Full operation sequence
    pub max_width: u32,                  // Preview constraints
    pub max_height: u32,
}
```

**Output**:
```rust
#[derive(Debug, Serialize)]
pub struct PreviewOutput {
    pub preview_base64: String,
    pub width: u32,
    pub height: u32,
}
```

---

### 14. ExportInput / ExportOutput

**Input**:
```rust
#[derive(Debug, Deserialize)]
pub struct ExportInput {
    pub output_path: String,             // Destination file path
    pub operations: Vec<EditOperation>,  // Operations to apply
    pub format: String,                  // "jpeg", "png", "webp"
    pub quality: Option<u8>,             // 0-100 for JPEG/WebP
}
```

**Output**:
```rust
#[derive(Debug, Serialize)]
pub struct ExportOutput {
    pub success: bool,
    pub output_path: String,
    pub file_size: u64,  // Size in bytes
}
```

---

## Error Types

### 15. AppError

**Rust Definition**:
```rust
#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("图像加载失败: {source}")]
    ImageLoadError { source: String },
    
    #[error("图像保存失败: {source}")]
    ImageSaveError { source: String },
    
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
```

**TypeScript Equivalent**:
```typescript
type AppError =
  | { type: "image_load_error"; message: string }
  | { type: "image_save_error"; message: string }
  | { type: "unsupported_format"; message: string }
  | { type: "file_access_denied"; message: string }
  | { type: "invalid_operation"; message: string }
  | { type: "out_of_memory"; required: number; available: number }
  | { type: "processing_error"; message: string }
  | { type: "state_error"; message: string };
```

---

## Data Flow Diagrams

### Operation Application Flow

```
Frontend                    Backend
   |                           |
   |-- ApplyOperationInput --->|
   |                           |
   |                     [Validate Operation]
   |                           |
   |                     [Add to History]
   |                           |
   |                     [Render Preview]
   |                           |
   |                     [Generate Base64]
   |                           |
   |<-- ApplyOperationOutput --|
   |                           |
```

### Export Flow

```
Frontend                    Backend
   |                           |
   |-- ExportInput ----------->|
   |                           |
   |                     [Load Original]
   |                           |
   |                     [Apply All Operations]
   |                           |
   |                     [Encode to Format]
   |                           |
   |                     [Write to Disk]
   |                           |
   |<-- ExportOutput ----------|
   |                           |
```

---

## Validation Rules Summary

| Entity | Field | Validation |
|--------|-------|------------|
| AdjustmentParams | brightness | 0.0 ≤ value ≤ 2.0 |
| AdjustmentParams | contrast | 0.0 ≤ value ≤ 2.0 |
| AdjustmentParams | saturation | 0.0 ≤ value ≤ 2.0 |
| AdjustmentParams | hue | -180 ≤ value ≤ 180 |
| AdjustmentParams | gamma | 0.1 ≤ value ≤ 3.0 |
| FilterType | intensity | 0.0 ≤ value ≤ 1.0 |
| CropRect | all fields | > 0 |
| CropRect | x + width | ≤ image.width |
| CropRect | y + height | ≤ image.height |
| Image | dimensions | ≤ 16384x16384 |
| File | size | ≤ 500MB |
| HistoryManager | history.len() | ≤ 50 |

---

## Memory Characteristics

| Entity | Size (approx) | Lifetime | Storage |
|--------|---------------|----------|---------|
| EditOperation | ~200 bytes | Until cleared | History stack |
| DynamicImage (original) | 4 × width × height | Application session | ArcSwap |
| DynamicImage (preview) | Variable | Cache duration | ArcSwap |
| PreviewCache | ~2MB | Until invalidated | ArcSwap |
| HistoryManager | ~10KB (50 ops) | Application session | Mutex |

---

## Future Extensions

### Planned Entities

1. **Layer**: For layer-based editing
   ```rust
   struct Layer {
       id: String,
       name: String,
       operations: Vec<EditOperation>,
       visible: bool,
       opacity: f32,
       blend_mode: BlendMode,
   }
   ```

2. **Project**: For saving/loading edit sessions
   ```rust
   struct Project {
       version: String,
       original_path: String,
       layers: Vec<Layer>,
       export_settings: ExportSettings,
   }
   ```

3. **ExportSettings**: Persistent export configuration
   ```rust
   struct ExportSettings {
       default_format: String,
       default_quality: u8,
       preserve_metadata: bool,
   }
   ```

---

**Completed**: 2025-12-30
