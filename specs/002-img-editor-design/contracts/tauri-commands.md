# Tauri Command Contracts

**Version**: 1.0.0  
**Date**: 2025-12-30  
**Protocol**: Tauri IPC (JSON over MessagePack)

## Overview

This document defines all Tauri command contracts for the img-editor application. Commands are the primary interface between the Svelte frontend and Rust backend.

---

## Command: `open_image`

Opens an image file and returns a preview.

### Input

```json
{
  "path": "/absolute/path/to/image.jpg"
}
```

**TypeScript**:
```typescript
interface OpenImageInput {
  path: string;
}
```

**Validation**:
- `path` must be absolute file path
- File must exist and be readable
- File size ≤ 500MB
- Format must be supported (PNG, JPEG, GIF, BMP, WebP, TIFF, AVIF)

### Output (Success)

```json
{
  "preview_base64": "data:image/png;base64,iVBORw0KG...",
  "original_width": 3840,
  "original_height": 2160,
  "format": "Rgb8"
}
```

**TypeScript**:
```typescript
interface OpenImageOutput {
  preview_base64: string;
  original_width: number;
  original_height: number;
  format: string;
}
```

### Error Cases

| Error Type | HTTP Status Equivalent | Description |
|------------|------------------------|-------------|
| `image_load_error` | 400 | File corrupted or invalid format |
| `file_access_denied` | 403 | Permission denied |
| `unsupported_format` | 415 | Image format not supported |

**Example Error**:
```json
{
  "type": "image_load_error",
  "message": "Failed to decode image: invalid PNG header"
}
```

---

## Command: `apply_operation`

Applies a single editing operation to the current image.

### Input

```json
{
  "operation": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "type": "adjustment",
    "op": "adjustment",
    "brightness": 1.2,
    "contrast": 1.1
  },
  "preview_width": 1920
}
```

**TypeScript**:
```typescript
interface ApplyOperationInput {
  operation: EditOperation;
  preview_width?: number;
}
```

**Validation**:
- `operation.id` must be valid UUID v4
- Operation parameters must pass type-specific validation
- `preview_width` if provided must be > 0 and ≤ 16384

### Output (Success)

```json
{
  "preview_base64": "data:image/png;base64,iVBORw0KG...",
  "new_width": 1920,
  "new_height": 1080
}
```

**TypeScript**:
```typescript
interface ApplyOperationOutput {
  preview_base64: string;
  new_width: number;
  new_height: number;
}
```

### Error Cases

| Error Type | Description |
|------------|-------------|
| `invalid_operation` | Operation parameters out of range or malformed |
| `processing_error` | Image processing failed |
| `state_error` | No image loaded |

---

## Command: `generate_preview`

Generates a preview from a full operation sequence (used for undo/redo).

### Input

```json
{
  "operations": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "type": "filter",
      "op": "filter",
      "filter_type": "grayscale",
      "intensity": 1.0
    },
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "type": "adjustment",
      "op": "adjustment",
      "brightness": 1.2
    }
  ],
  "max_width": 1920,
  "max_height": 1920
}
```

**TypeScript**:
```typescript
interface PreviewInput {
  operations: EditOperation[];
  max_width: number;
  max_height: number;
}
```

**Validation**:
- `operations` can be empty array (returns original)
- `max_width` and `max_height` must be > 0

### Output (Success)

```json
{
  "preview_base64": "data:image/png;base64,iVBORw0KG...",
  "width": 1920,
  "height": 1080
}
```

**TypeScript**:
```typescript
interface PreviewOutput {
  preview_base64: string;
  width: number;
  height: number;
}
```

### Error Cases

Same as `apply_operation`.

---

## Command: `undo`

Undoes the last operation.

### Input

None (command takes no parameters)

**TypeScript**:
```typescript
// Invoked as: invoke('undo')
```

### Output (Success)

```json
{
  "preview_base64": "data:image/png;base64,iVBORw0KG...",
  "width": 1920,
  "height": 1080
}
```

**TypeScript**:
```typescript
interface PreviewOutput {
  preview_base64: string;
  width: number;
  height: number;
}
```

### Error Cases

| Error Type | Description |
|------------|-------------|
| `state_error` | Nothing to undo (history is empty) |

---

## Command: `redo`

Re-applies an undone operation.

### Input

None

**TypeScript**:
```typescript
// Invoked as: invoke('redo')
```

### Output (Success)

Same as `undo`

### Error Cases

| Error Type | Description |
|------------|-------------|
| `state_error` | Nothing to redo (redo stack is empty) |

---

## Command: `export_image`

Exports the final image with all operations applied.

### Input

```json
{
  "output_path": "/absolute/path/to/output.jpg",
  "operations": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "type": "filter",
      "op": "filter",
      "filter_type": "grayscale",
      "intensity": 1.0
    }
  ],
  "format": "jpeg",
  "quality": 90
}
```

**TypeScript**:
```typescript
interface ExportInput {
  output_path: string;
  operations: EditOperation[];
  format: "jpeg" | "png" | "webp";
  quality?: number;  // 0-100, only for JPEG/WebP
}
```

**Validation**:
- `output_path` must be writable location
- `format` must be "jpeg", "png", or "webp"
- `quality` if provided must be 1-100

### Output (Success)

```json
{
  "success": true,
  "output_path": "/absolute/path/to/output.jpg",
  "file_size": 2457600
}
```

**TypeScript**:
```typescript
interface ExportOutput {
  success: boolean;
  output_path: string;
  file_size: number;  // bytes
}
```

### Error Cases

| Error Type | Description |
|------------|-------------|
| `file_access_denied` | Cannot write to output path |
| `image_save_error` | Encoding failed |
| `unsupported_format` | Invalid format specified |
| `processing_error` | Operation application failed |

---

## Command: `get_history_state`

Returns current undo/redo stack state (for UI indicators).

### Input

None

### Output (Success)

```json
{
  "can_undo": true,
  "can_redo": false,
  "history_count": 5,
  "redo_count": 0
}
```

**TypeScript**:
```typescript
interface HistoryState {
  can_undo: boolean;
  can_redo: boolean;
  history_count: number;
  redo_count: number;
}
```

---

## Command: `clear_image`

Clears the current image and history (reset to initial state).

### Input

None

### Output (Success)

```json
{
  "success": true
}
```

**TypeScript**:
```typescript
interface ClearOutput {
  success: boolean;
}
```

---

## Type Definitions

### EditOperation (Full Definition)

```typescript
type EditOperation =
  | FilterOperation
  | AdjustmentOperation
  | TransformOperation
  | CropOperation;

interface FilterOperation {
  id: string;
  type: "filter";
  op: "filter";
  filter_type: "grayscale" | "sepia" | "invert" | "blur" | "sharpen";
  intensity: number;  // 0.0 - 1.0
}

interface AdjustmentOperation {
  id: string;
  type: "adjustment";
  op: "adjustment";
  brightness?: number;   // 0.0 - 2.0
  contrast?: number;     // 0.0 - 2.0
  saturation?: number;   // 0.0 - 2.0
  hue?: number;          // -180 to 180
  gamma?: number;        // 0.1 - 3.0
}

interface TransformOperation {
  id: string;
  type: "transform";
  op: "transform";
  transform_type: {
    type: "rotate90" | "rotate180" | "rotate270" | "flip_horizontal" | "flip_vertical";
  };
}

interface CropOperation {
  id: string;
  type: "crop";
  op: "crop";
  rect: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
  maintain_aspect_ratio?: number;
}
```

### AppError (Full Definition)

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

## Usage Examples

### Opening an Image

```typescript
import { invoke } from '@tauri-apps/api/core';

try {
  const result = await invoke<OpenImageOutput>('open_image', {
    path: '/Users/user/Desktop/photo.jpg'
  });
  
  console.log(`Loaded ${result.original_width}x${result.original_height} image`);
  displayPreview(result.preview_base64);
} catch (error) {
  if (error.type === 'unsupported_format') {
    alert('This image format is not supported');
  } else {
    console.error('Failed to open image:', error);
  }
}
```

### Applying a Filter

```typescript
import { v4 as uuidv4 } from 'uuid';

const operation: FilterOperation = {
  id: uuidv4(),
  type: 'filter',
  op: 'filter',
  filter_type: 'grayscale',
  intensity: 1.0
};

const result = await invoke<ApplyOperationOutput>('apply_operation', {
  operation,
  preview_width: 1920
});

displayPreview(result.preview_base64);
```

### Exporting Final Image

```typescript
const result = await invoke<ExportOutput>('export_image', {
  output_path: '/Users/user/Desktop/edited.jpg',
  operations: currentOperations,  // From state management
  format: 'jpeg',
  quality: 95
});

console.log(`Saved ${result.file_size} bytes to ${result.output_path}`);
```

---

## Performance Considerations

### Base64 Encoding Size

| Preview Size | Approx Base64 Size |
|--------------|-------------------|
| 640x360 | ~300 KB |
| 1280x720 | ~1.2 MB |
| 1920x1080 | ~2.5 MB |
| 3840x2160 | ~10 MB |

**Recommendation**: Use `preview_width: 1920` for desktop, `preview_width: 1280` for mobile.

### Command Response Times (Target)

| Command | Cold (First Call) | Warm (Cached) |
|---------|-------------------|---------------|
| `open_image` | <500ms | <200ms |
| `apply_operation` | <200ms | <100ms |
| `generate_preview` | <200ms | <50ms (cached) |
| `undo/redo` | <100ms | <50ms |
| `export_image` | Varies by size | N/A |

---

## Versioning

API version is implicit from Tauri application version. Breaking changes will require major version bump.

**Current Version**: 1.0.0  
**Stability**: Draft (implementation pending)

---

**Generated**: 2025-12-30
