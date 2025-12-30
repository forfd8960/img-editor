# Implementation Plan: img-editor Design Implementation

**Branch**: `002-img-editor-design` | **Date**: 2025-12-30 | **Spec**: [0002-design.md](../0002-design.md)
**Input**: Feature specification from `/specs/0002-design.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

img-editor is a high-performance, non-destructive image editor built with Tauri v2, implementing 60 FPS UI responsiveness and cross-platform support (macOS, Windows, Linux, Android, iOS). The design uses a hybrid approach: CSS filters for real-time preview in the frontend, and Rust-based pixel operations only during export/save. The system maintains original high-resolution images while storing edit history as operation sequences instead of image copies, achieving both performance and memory efficiency.

## Technical Context

**Language/Version**: Rust 2024 edition  
**Primary Dependencies**: Tauri 2.0+, image 0.25.x, imageproc 0.25.x, rayon 1.10+, arcswap 1.x, serde 1.0.x, specta 2.x  
**Storage**: File system (for original images and export), in-memory state management with ArcSwap  
**Testing**: cargo test, integration tests for contracts, unit tests for core modules  
**Target Platform**: macOS, Windows, Linux, Android, iOS (via Tauri 2.0)  
**Project Type**: Cross-platform desktop/mobile application with Rust backend and Svelte 5 frontend  
**Performance Goals**: 60 FPS UI, multi-core parallel image processing, <200ms preview generation  
**Constraints**: Non-destructive editing (preserve original), memory optimization (Arc-based sharing), <500MB max file size  
**Scale/Scope**: Single-user desktop/mobile app, support for 20+ image formats, history limit of 50 operations

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

The constitution file is a template and has not been customized for this project. No specific gates or principles have been defined yet. This implementation will proceed with general software engineering best practices:

- **Test-First Development**: Unit and integration tests will be written
- **Modular Design**: Code organized into clear modules (commands, core, state, types, utils)
- **Error Handling**: Comprehensive error types and handling strategy defined
- **Documentation**: Inline documentation and external design specification
- **Security**: Input validation, resource limits, dependency auditing

**Status**: ✅ PASS (No custom constitution gates defined)

## Project Structure

### Documentation (this feature)

```text
specs/002-img-editor-design/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Cross-platform application (Tauri desktop + mobile)
src/
├── main.rs              # Application entry point
├── lib.rs               # Library entry
├── commands/            # Tauri command layer
│   ├── mod.rs
│   ├── image_commands.rs     # Image operations
│   ├── export_commands.rs    # Export operations
│   └── state_commands.rs     # State management
├── core/                # Core business logic
│   ├── mod.rs
│   ├── image_processor.rs    # Image processing engine
│   ├── history_manager.rs    # History/undo manager
│   ├── export_engine.rs      # Export engine
│   └── operations/           # Operation implementations
│       ├── mod.rs
│       ├── filters.rs        # Filter operations
│       ├── adjustments.rs    # Adjustment operations
│       ├── transform.rs      # Transform operations
│       └── crop.rs           # Crop operations
├── state/               # State management
│   ├── mod.rs
│   ├── app_state.rs          # Application state
│   └── image_state.rs        # Image state (ArcSwap)
├── types/               # Type definitions
│   ├── mod.rs
│   ├── operations.rs         # Operation types
│   ├── errors.rs             # Error types
│   └── commands.rs           # Command I/O types
└── utils/               # Utility functions
    ├── mod.rs
    ├── base64.rs             # Base64 encoding
    └── preview.rs            # Preview generation

tests/
├── integration/         # Integration tests
│   ├── commands_test.rs
│   ├── export_test.rs
│   └── operations_test.rs
└── unit/                # Unit tests
    ├── image_processor_test.rs
    ├── history_manager_test.rs
    └── utils_test.rs

frontend/
├── src/
│   ├── components/      # UI components
│   ├── pages/           # Application pages
│   └── services/        # Tauri IPC services
└── tests/               # Frontend tests
```

**Structure Decision**: This is a cross-platform application using Tauri 2.0's unified architecture. The Rust backend provides commands via IPC to a Svelte 5 frontend. The structure follows Tauri conventions with clear separation between command layer (IPC interface), core domain logic, and state management.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations detected. The constitution is a template without custom rules.
