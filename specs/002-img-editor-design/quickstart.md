# Quickstart Guide: img-editor Implementation

**Version**: 1.0.0  
**Date**: 2025-12-30  
**Branch**: `002-img-editor-design`

## Overview

This guide provides step-by-step instructions for implementing the img-editor design specification. Follow these phases in order to build the complete application.

---

## Prerequisites

### Required Tools

- **Rust**: 1.75+ with 2024 edition support
- **Node.js**: 18+ (for Tauri frontend)
- **Tauri CLI**: 2.0+
- **Git**: For version control

### Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify Rust 2024 edition support
rustc --version  # Should be 1.75+

# Install Tauri CLI
cargo install tauri-cli --version "^2.0"

# Install Node.js dependencies
npm install
```

---

## Phase 1: Project Setup

### 1.1 Update Cargo.toml

Replace the existing `Cargo.toml` with the dependency configuration from the design spec:

```toml
[package]
name = "img-editor"
version = "0.1.0"
edition = "2024"

[dependencies]
# Core
tauri = { version = "2.0.0", features = ["core", "window-all", "shell-open"] }
tauri-build = "2.0.0"

# Image Processing
image = { version = "0.25", features = ["png", "jpeg", "gif", "bmp", "ico", "webp", "tiff", "avif-dec", "rayon"] }
imageproc = "0.25"

# Concurrency
rayon = "1.10"
arcswap = "1"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
specta = { version = "2", features = ["typescript"] }

# Utils
base64 = "0.22"
thiserror = "2"
anyhow = "1"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]

[profile.release]
panic = "abort"
codegen-units = 1
lto = true
opt-level = "z"
```

### 1.2 Create Module Structure

```bash
# Create directory structure
mkdir -p src/{commands,core/operations,state,types,utils}
mkdir -p tests/{integration,unit}
mkdir -p frontend/src/{components,pages,services}
```

---

## Phase 2: Type Definitions

### 2.1 Create Error Types

Create `src/types/errors.rs`:

```rust
use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Error)]
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

// Implement custom serialization for Tauri
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("AppError", 2)?;
        match self {
            AppError::ImageLoadError { source } => {
                s.serialize_field("type", "image_load_error")?;
                s.serialize_field("message", source)?;
            }
            AppError::ImageSaveError { source } => {
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
```

### 2.2 Create Operation Types

Create `src/types/operations.rs` with the definitions from `data-model.md`.

### 2.3 Create Command Types

Create `src/types/commands.rs` with the input/output types from the contracts.

---

## Phase 3: Core Logic

### 3.1 Implement History Manager

Create `src/core/history_manager.rs`:

```rust
use std::sync::Mutex;
use crate::types::operations::EditOperation;

pub struct HistoryManager {
    history: Mutex<Vec<EditOperation>>,
    redo_stack: Mutex<Vec<EditOperation>>,
    max_history: usize,
}

impl HistoryManager {
    pub fn new() -> Self {
        Self {
            history: Mutex::new(Vec::new()),
            redo_stack: Mutex::new(Vec::new()),
            max_history: 50,
        }
    }

    pub async fn add_operation(&self, operation: EditOperation) -> Vec<EditOperation> {
        let mut history = self.history.lock().unwrap();
        let mut redo_stack = self.redo_stack.lock().unwrap();
        
        redo_stack.clear();
        history.push(operation);
        
        if history.len() > self.max_history {
            history.remove(0);
        }
        
        history.clone()
    }

    pub async fn undo(&self) -> Option<Vec<EditOperation>> {
        let mut history = self.history.lock().unwrap();
        let mut redo_stack = self.redo_stack.lock().unwrap();
        
        if let Some(op) = history.pop() {
            redo_stack.push(op);
            return Some(history.clone());
        }
        
        None
    }

    pub async fn redo(&self) -> Option<Vec<EditOperation>> {
        let mut history = self.history.lock().unwrap();
        let mut redo_stack = self.redo_stack.lock().unwrap();
        
        if let Some(op) = redo_stack.pop() {
            history.push(op);
            return Some(history.clone());
        }
        
        None
    }

    pub async fn clear(&self) {
        self.history.lock().unwrap().clear();
        self.redo_stack.lock().unwrap().clear();
    }
}
```

### 3.2 Implement Image Processor

Create `src/core/image_processor.rs` with filter and adjustment logic.

### 3.3 Implement Export Engine

Create `src/core/export_engine.rs` with export functionality.

---

## Phase 4: State Management

### 4.1 Create Image State

Create `src/state/image_state.rs`:

```rust
use std::sync::Arc;
use arc_swap::ArcSwap;
use image::DynamicImage;
use crate::core::history_manager::HistoryManager;
use crate::types::operations::EditOperation;

pub struct ImageState {
    original_image: ArcSwap<Option<DynamicImage>>,
    current_image: ArcSwap<Option<DynamicImage>>,
    history: Arc<HistoryManager>,
}

impl ImageState {
    pub fn new() -> Self {
        Self {
            original_image: ArcSwap::new(Arc::new(None)),
            current_image: ArcSwap::new(Arc::new(None)),
            history: Arc::new(HistoryManager::new()),
        }
    }

    pub async fn load_original(&self, path: &str) -> Result<DynamicImage, anyhow::Error> {
        let img = tokio::task::spawn_blocking(move || {
            image::ImageReader::open(path)?.decode()
        })
        .await??;

        self.original_image.store(Arc::new(Some(img.clone())));
        self.current_image.store(Arc::new(Some(img.clone())));

        Ok(img)
    }

    // Add other methods as per design spec
}
```

---

## Phase 5: Tauri Commands

### 5.1 Create Image Commands

Create `src/commands/image_commands.rs`:

```rust
use crate::types::{commands::*, errors::AppError};
use crate::state::image_state::ImageState;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn open_image(
    input: OpenImageInput,
    state: State<'_, ImageState>,
) -> Result<OpenImageOutput, AppError> {
    // Implementation from design spec
    todo!()
}

#[tauri::command]
#[specta::specta]
pub async fn apply_operation(
    input: ApplyOperationInput,
    state: State<'_, ImageState>,
) -> Result<ApplyOperationOutput, AppError> {
    // Implementation from design spec
    todo!()
}

// Add other commands
```

### 5.2 Register Commands in main.rs

Update `src/main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;
mod state;
mod types;
mod utils;

use state::image_state::ImageState;

fn main() {
    tauri::Builder::default()
        .manage(ImageState::new())
        .invoke_handler(tauri::generate_handler![
            commands::image_commands::open_image,
            commands::image_commands::apply_operation,
            commands::image_commands::generate_preview,
            commands::image_commands::undo,
            commands::image_commands::redo,
            commands::export_commands::export_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Phase 6: Frontend Integration

### 6.1 Generate TypeScript Types

Create a build script to generate types:

```rust
// src/bin/generate-types.rs
use specta::collect_types;
use tauri_specta::ts;

fn main() {
    let types = collect_types![
        // List all command types here
    ];

    ts::export(types, "../frontend/src/types/bindings.ts").unwrap();
}
```

Run: `cargo run --bin generate-types`

### 6.2 Create Tauri Service

Create `frontend/src/services/tauri.ts`:

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { 
  OpenImageInput, 
  OpenImageOutput,
  ApplyOperationInput,
  ApplyOperationOutput
} from '../types/bindings';

export async function openImage(path: string): Promise<OpenImageOutput> {
  return await invoke('open_image', { path });
}

export async function applyOperation(
  operation: EditOperation,
  previewWidth?: number
): Promise<ApplyOperationOutput> {
  return await invoke('apply_operation', { operation, preview_width: previewWidth });
}

// Add other command wrappers
```

---

## Phase 7: Testing

### 7.1 Unit Tests

Create `tests/unit/history_manager_test.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_operation() {
        let manager = HistoryManager::new();
        let op = create_test_operation();
        
        let result = manager.add_operation(op).await;
        assert_eq!(result.len(), 1);
    }

    // Add more tests
}
```

### 7.2 Integration Tests

Create `tests/integration/commands_test.rs` to test full command flow.

### 7.3 Run Tests

```bash
cargo test
cargo test --release  # For performance tests
```

---

## Phase 8: Build & Run

### 8.1 Development Build

```bash
# Terminal 1: Backend
cargo run

# Terminal 2: Frontend (if separate)
cd frontend && npm run dev
```

Or use Tauri CLI:

```bash
cargo tauri dev
```

### 8.2 Production Build

```bash
cargo tauri build
```

Output location:
- macOS: `target/release/bundle/macos/img-editor.app`
- Windows: `target/release/bundle/msi/img-editor.msi`
- Linux: `target/release/bundle/appimage/img-editor.AppImage`

---

## Development Workflow

### Daily Development

1. **Start development server**: `cargo tauri dev`
2. **Make changes** to Rust or frontend code
3. **Hot reload** automatically applies changes
4. **Test** with `cargo test`
5. **Commit** frequently with descriptive messages

### Before Committing

```bash
# Format code
cargo fmt

# Check for errors
cargo clippy

# Run tests
cargo test

# Check types in frontend
cd frontend && npm run type-check
```

---

## Troubleshooting

### Common Issues

**Issue**: `image` crate compile errors  
**Solution**: Ensure all required features are enabled in `Cargo.toml`

**Issue**: Tauri commands not found  
**Solution**: Verify `generate_handler!` includes all commands

**Issue**: TypeScript type mismatches  
**Solution**: Regenerate types with `cargo run --bin generate-types`

**Issue**: Slow performance  
**Solution**: Build with `--release` flag for production-level optimization

---

## Performance Benchmarking

### Create Benchmarks

```bash
# Add criterion to dev-dependencies
cargo add --dev criterion

# Create bench file
mkdir benches
```

Create `benches/image_processing.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_filter(c: &mut Criterion) {
    c.bench_function("grayscale 1920x1080", |b| {
        b.iter(|| {
            // Benchmark grayscale operation
        });
    });
}

criterion_group!(benches, benchmark_filter);
criterion_main!(benches);
```

Run: `cargo bench`

---

## Next Steps

After completing the basic implementation:

1. **Add more filters**: Implement additional filter types
2. **Optimize performance**: Profile and optimize hot paths with Rayon
3. **Mobile support**: Test on Android/iOS using Tauri mobile
4. **Polish UI**: Improve Svelte components and styling
5. **User testing**: Gather feedback and iterate

---

## Resources

- [Design Spec](../0002-design.md)
- [Data Model](./data-model.md)
- [Contracts](./contracts/)
- [Tauri Documentation](https://v2.tauri.app/)
- [Rust image crate](https://docs.rs/image/latest/image/)

---

**Last Updated**: 2025-12-30
