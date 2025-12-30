# Research: img-editor Design Implementation

**Date**: 2025-12-30  
**Status**: Complete

## Overview

This document consolidates research findings for implementing the img-editor design specification. All technical decisions from the design spec are documented here with their rationale and alternatives considered.

---

## Technology Stack Decisions

### 1. Framework: Tauri 2.0

**Decision**: Use Tauri 2.0 as the application framework

**Rationale**:
- **Cross-platform support**: Single codebase for macOS, Windows, Linux, Android, and iOS
- **Performance**: Native Rust backend with minimal overhead
- **Security**: Built-in permission system and sandboxing
- **IPC Communication**: Type-safe communication via Specta
- **Small bundle size**: Significantly smaller than Electron
- **Modern web stack**: Allows using Svelte 5 for UI

**Alternatives Considered**:
- **Electron**: Rejected due to larger bundle size and higher memory usage
- **Qt**: Rejected due to licensing concerns and C++ complexity
- **Native per-platform**: Rejected due to maintenance overhead of multiple codebases

---

### 2. Image Processing: image + imageproc crates

**Decision**: Use `image` v0.25.x and `imageproc` v0.25.x

**Rationale**:
- **Format support**: image crate supports 20+ formats (PNG, JPEG, GIF, WebP, AVIF, etc.)
- **100% documentation**: Well-documented with extensive examples
- **Parallel support**: Built-in rayon integration for multi-threaded operations
- **Ecosystem maturity**: Widely used in production Rust applications
- **imageproc additions**: Advanced filters, geometric transforms, edge detection

**Alternatives Considered**:
- **opencv-rust**: Rejected due to complex C++ bindings and large dependency footprint
- **photon**: Rejected due to less comprehensive format support
- **Custom implementation**: Rejected as reinventing the wheel for standard operations

---

### 3. Concurrency: Rayon + ArcSwap

**Decision**: Use Rayon 1.10+ for parallel processing and ArcSwap 1.x for state sharing

**Rationale**:
- **Rayon**: Data parallelism with zero-cost abstractions, perfect for pixel operations
- **ArcSwap**: Lock-free atomic Arc swapping for read-heavy workloads
- **Performance**: Avoid mutex contention on frequently-read original image
- **Safety**: Both provide memory-safe concurrency without data races

**Alternatives Considered**:
- **Standard Mutex**: Rejected due to contention on high-frequency reads
- **RwLock**: Rejected as ArcSwap provides better performance for immutable data
- **crossbeam**: Considered but Rayon provides better ergonomics for image processing

---

### 4. Frontend: Svelte 5

**Decision**: Use Svelte 5 for the user interface

**Rationale**:
- **Reactivity**: Excellent for real-time slider updates and preview feedback
- **Performance**: Compiles to minimal vanilla JavaScript
- **Developer experience**: Simple syntax, less boilerplate than React/Vue
- **Size**: Smaller bundle size compared to React
- **CSS Filters**: Easy integration for real-time preview without backend calls

**Alternatives Considered**:
- **React**: Rejected due to larger bundle size and more boilerplate
- **Vue 3**: Considered viable but team preference for Svelte's approach
- **Vanilla JS**: Rejected due to development velocity and maintainability

---

### 5. Serialization: Serde + Specta

**Decision**: Use Serde 1.0 for serialization and Specta 2.x for TypeScript bindings

**Rationale**:
- **Serde**: De facto standard for Rust serialization with derive macros
- **Specta**: Automatically generates TypeScript types from Rust structs
- **Type safety**: Ensures frontend-backend contract consistency
- **Tauri integration**: First-class Specta support in Tauri 2.0

**Alternatives Considered**:
- **Manual TypeScript definitions**: Rejected due to maintenance burden and error risk
- **bincode**: Considered but JSON is better for debugging and web compatibility
- **ts-rs**: Alternative to Specta, but Specta has better Tauri integration

---

## Architecture Patterns

### 6. Non-Destructive Editing

**Decision**: Store original image immutably + operation sequence

**Rationale**:
- **Memory efficiency**: Store operations (KB) instead of image copies (MB)
- **Flexibility**: Can re-render from scratch or apply incremental changes
- **Undo/Redo**: Natural implementation by stack manipulation
- **Export**: Apply all operations to original for final output

**Alternatives Considered**:
- **Copy-on-write**: Rejected due to memory overhead for large images
- **Layer system**: Deferred to future iteration for complexity reasons

---

### 7. Hybrid Preview Strategy

**Decision**: CSS filters for live preview, Rust for export/save

**Rationale**:
- **60 FPS goal**: CSS filters run on GPU, no IPC overhead
- **Immediate feedback**: Slider changes reflect instantly
- **Quality**: Export uses high-precision Rust algorithms on original image
- **Battery efficiency**: Avoid constant backend calls on mobile

**Alternatives Considered**:
- **Pure backend**: Rejected due to IPC latency and battery drain
- **Pure frontend**: Rejected as Canvas operations can't match Rust precision
- **Web Workers**: Considered but CSS filters provide better performance

---

### 8. State Management: ArcSwap Pattern

**Decision**: Use ArcSwap for original/current image, Mutex for history

**Rationale**:
- **Read-heavy access**: Original image is read frequently but never modified
- **Lock-free reads**: ArcSwap provides zero-cost reads via atomic operations
- **Write-rare history**: Mutex acceptable for infrequent undo/redo operations
- **Memory sharing**: Multiple references to original without cloning

**Alternatives Considered**:
- **RwLock everywhere**: Rejected due to unnecessary locking overhead
- **Immutable data structures (im crate)**: Considered but ArcSwap simpler for this use case
- **Message passing**: Rejected as adds complexity without clear benefit

---

## Performance Strategies

### 9. Parallel Image Processing

**Decision**: Use Rayon for pixel-level parallelism

**Rationale**:
- **Multi-core utilization**: Automatic work-stealing across available cores
- **Simple API**: `par_pixels()` provides iterator-based parallelism
- **Safety**: Prevents data races at compile time
- **Scalability**: Performance scales linearly with core count

**Implementation Example**:
```rust
image.par_pixels_mut().for_each(|pixel| {
    // Apply transformation
});
```

**Alternatives Considered**:
- **Manual threading**: Rejected due to complexity and error-proneness
- **SIMD**: Deferred to future optimization, Rayon provides good baseline
- **GPU compute**: Deferred due to cross-platform complexity (Metal/Vulkan/DirectX)

---

### 10. Preview Caching

**Decision**: Cache Base64 preview with operations hash

**Rationale**:
- **Avoid redundant work**: Don't re-generate identical previews
- **Hash-based invalidation**: Simple equality check on operation sequence
- **Memory trade-off**: Store ~1-2MB cache vs. 100ms+ generation time

**Alternatives Considered**:
- **No caching**: Rejected due to poor UX on undo/redo
- **LRU cache**: Considered but single entry cache sufficient for this use case
- **Disk cache**: Rejected due to complexity and I/O overhead

---

## Security & Validation

### 11. Input Validation Strategy

**Decision**: Validate all operation parameters at command boundary

**Rationale**:
- **Defense in depth**: Don't trust frontend input
- **Early errors**: Fail fast with clear error messages
- **Resource protection**: Prevent memory exhaustion attacks
- **Type safety**: Serde handles basic type validation

**Validation Rules**:
- Brightness: 0.0 - 2.0
- Contrast: 0.0 - 2.0
- Image dimensions: ≤ 16384x16384
- File size: ≤ 500MB
- Crop coordinates: Within image bounds

**Alternatives Considered**:
- **Trust frontend**: Rejected as security risk
- **Runtime checks only**: Rejected in favor of upfront validation

---

### 12. Dependency Security

**Decision**: Use cargo-audit + cargo-deny in CI pipeline

**Rationale**:
- **Known vulnerabilities**: cargo-audit checks against RustSec database
- **License compliance**: cargo-deny verifies acceptable licenses
- **Supply chain**: Automated checks on every commit
- **Maintenance**: Regular `cargo update` with testing

**Alternatives Considered**:
- **Manual review**: Rejected due to scalability issues
- **Lock file only**: Rejected as doesn't catch new vulnerabilities

---

## Error Handling

### 13. Error Type Design

**Decision**: Use thiserror for domain errors, anyhow for internal errors

**Rationale**:
- **User-facing errors**: thiserror provides structured errors for frontend
- **Internal errors**: anyhow for convenience in backend logic
- **Serialization**: thiserror errors can be serialized to JSON for IPC
- **Context**: Both support error context with `.context()` method

**Error Categories**:
1. **Recoverable**: File access, memory warnings → Degrade gracefully
2. **Unrecoverable**: Corrupted image, unsupported format → Hard error
3. **User errors**: Invalid parameters → Validation error with details

**Alternatives Considered**:
- **Result<T, String>**: Rejected due to lack of type safety
- **Custom Error enum**: thiserror provides this via derive macro

---

## Export & Quality

### 14. Export Format Strategy

**Decision**: Support JPEG (with quality), PNG, WebP as initial formats

**Rationale**:
- **JPEG**: Ubiquitous, small file size, quality parameter (0-100)
- **PNG**: Lossless, transparency support
- **WebP**: Modern format with better compression than JPEG
- **Extensibility**: image crate makes adding formats trivial

**Quality Settings**:
- JPEG: 90 (default), user-configurable 1-100
- PNG: Lossless compression level
- WebP: 90 (default), user-configurable

**Alternatives Considered**:
- **AVIF**: Deferred to v2 due to limited browser support
- **HEIF**: Deferred due to patent concerns
- **TIFF**: Deferred as less common for consumer use

---

### 15. Progressive Enhancement

**Decision**: Degrade preview quality on memory pressure

**Rationale**:
- **Mobile support**: Devices with limited RAM should still work
- **User experience**: Better to show lower quality than crash
- **Detection**: Monitor memory usage, reduce preview dimensions if needed

**Degradation Levels**:
1. Full quality: 1920px max dimension
2. Reduced: 1280px max dimension
3. Low: 640px max dimension
4. Minimal: 320px max dimension

**Alternatives Considered**:
- **Hard limits**: Rejected as poor UX on capable devices
- **No degradation**: Rejected due to mobile crash risk

---

## Testing Strategy

### 16. Test Coverage

**Decision**: Unit tests for core logic, integration tests for commands

**Rationale**:
- **Unit tests**: Image processor, history manager, utils (fast, isolated)
- **Integration tests**: Full command flow including state management
- **Contract tests**: Ensure command I/O matches TypeScript types
- **Property tests**: Use proptest for image operation correctness

**Coverage Goals**:
- Core modules: 80%+ coverage
- Commands: 90%+ coverage (critical path)
- Utils: 70%+ coverage

**Alternatives Considered**:
- **E2E only**: Rejected as slow and brittle
- **Unit only**: Rejected as misses integration issues

---

## Build & Optimization

### 17. Release Profile

**Decision**: Use aggressive optimization for release builds

**Configuration**:
```toml
[profile.release]
panic = "abort"        # Smaller binary, faster unwind
codegen-units = 1      # Better optimization
lto = true             # Link-time optimization
opt-level = "z"        # Optimize for size
```

**Rationale**:
- **Binary size**: Critical for mobile distribution
- **Performance**: LTO enables cross-crate optimizations
- **Trade-off**: Longer compile time acceptable for release builds

**Alternatives Considered**:
- `opt-level = 3`: Rejected as "z" provides 20-30% smaller binaries
- `codegen-units = 16`: Rejected in favor of optimization quality

---

## Future Considerations

### Deferred Features

1. **GPU Acceleration**: Metal/Vulkan compute for filters
   - **Rationale**: CPU parallelism sufficient for v1, GPU adds complexity
   - **Revisit**: When performance profiling shows CPU bottleneck

2. **Layer System**: Photoshop-style layers
   - **Rationale**: Significant complexity, operation sequence sufficient for v1
   - **Revisit**: User feedback indicates need

3. **Batch Processing**: Process multiple images
   - **Rationale**: Single-image focus for v1 simplicity
   - **Revisit**: User demand demonstrates need

4. **Cloud Sync**: Sync edits across devices
   - **Rationale**: Offline-first approach for v1
   - **Revisit**: After mobile launch success

5. **Plugin System**: Third-party filters
   - **Rationale**: Security and stability concerns
   - **Revisit**: After core functionality is stable

---

## References

- [Tauri 2.0 Documentation](https://v2.tauri.app/)
- [image crate docs](https://docs.rs/image/latest/image/)
- [imageproc crate docs](https://docs.rs/imageproc/latest/imageproc/)
- [Rayon parallel iterators](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html)
- [ArcSwap documentation](https://docs.rs/arc-swap/latest/arc_swap/)
- [Svelte 5 features](https://svelte.dev/)
- [Specta TypeScript generation](https://github.com/oscartbeaumont/specta)

---

**Completed**: 2025-12-30
