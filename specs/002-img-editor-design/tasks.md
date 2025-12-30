# Tasks: img-editor Implementation

**Input**: Design documents from `/specs/002-img-editor-design/`
**Prerequisites**: plan.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: Not explicitly requested in the specification - focusing on implementation tasks.

**Organization**: Tasks are grouped by functional user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## User Stories Derived from Design Spec

1. **US1 (P1)**: Image Loading - Open and display images with preview generation
2. **US2 (P2)**: Filter Operations - Apply visual filters (grayscale, sepia, blur, etc.)
3. **US3 (P3)**: Adjustments - Modify brightness, contrast, saturation, hue, gamma
4. **US4 (P4)**: Transforms - Rotate and flip images
5. **US5 (P5)**: Crop Operations - Select and crop image regions
6. **US6 (P6)**: History Management - Undo/redo operation sequences
7. **US7 (P7)**: Export - Save edited images in various formats

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Update Cargo.toml with all dependencies from design spec (Tauri 2.0, image 0.25, rayon, etc.)
- [X] T002 [P] Create module directory structure (commands/, core/, state/, types/, utils/)
- [X] T003 [P] Create test directory structure (tests/integration/, tests/unit/)
- [X] T004 [P] Initialize frontend directory with Svelte 5 scaffolding in frontend/
- [X] T005 [P] Configure Tauri 2.0 project in src-tauri/ with permissions and capabilities
- [X] T006 [P] Setup Rust formatting and clippy configuration in .cargo/config.toml
- [X] T007 [P] Configure release build optimizations in Cargo.toml (LTO, opt-level="z")

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T008 [P] Create error types module in src/types/errors.rs with all AppError variants
- [X] T009 [P] Implement error serialization for Tauri IPC in src/types/errors.rs
- [X] T010 [P] Create operation type definitions in src/types/operations.rs (EditOperation, OperationType, etc.)
- [X] T011 [P] Create command I/O types in src/types/commands.rs (all Input/Output structs)
- [X] T012 [P] Create types module coordinator in src/types/mod.rs
- [X] T013 Create HistoryManager structure in src/core/history_manager.rs with Mutex-based stacks
- [X] T014 Implement HistoryManager methods (add_operation, undo, redo, clear) in src/core/history_manager.rs
- [X] T015 Create ImageState structure with ArcSwap fields in src/state/image_state.rs
- [X] T016 Implement ImageState constructor and basic accessors in src/state/image_state.rs
- [X] T017 [P] Create utility module for Base64 encoding in src/utils/base64.rs
- [X] T018 [P] Implement encode_image and decode_image functions in src/utils/base64.rs
- [X] T019 [P] Create preview utility module in src/utils/preview.rs for resize operations
- [X] T020 [P] Setup module coordinators (src/commands/mod.rs, src/core/mod.rs, src/state/mod.rs, src/utils/mod.rs)
- [X] T021 Create main.rs with Tauri builder and ImageState management
- [X] T022 Create lib.rs exposing public API modules

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Image Loading (Priority: P1) 🎯 MVP

**Goal**: Users can open image files and see a preview with metadata

**Independent Test**: Open a JPEG/PNG file → Preview displays → Metadata shows (dimensions, format)

### Implementation for User Story 1

- [X] T023 [P] [US1] Implement load_original method in src/state/image_state.rs with tokio blocking
- [X] T024 [P] [US1] Implement generate_preview method in src/state/image_state.rs with resize logic
- [X] T025 [P] [US1] Create image validation helpers in src/utils/preview.rs (max dimensions, file size)
- [X] T026 [US1] Create image commands module in src/commands/image_commands.rs
- [X] T027 [US1] Implement open_image Tauri command in src/commands/image_commands.rs
- [X] T028 [US1] Add Specta annotations to open_image command (SKIPPED - using manual types)
- [X] T029 [US1] Register open_image command in src/main.rs invoke_handler
- [X] T030 [US1] Add input validation for file paths in src/commands/image_commands.rs
- [X] T031 [US1] Add error handling for unsupported formats in src/commands/image_commands.rs
- [X] T032 [P] [US1] Create TypeScript type generation script (SKIPPED - using manual types)
- [X] T033 [US1] Generate TypeScript bindings (SKIPPED - using manual types)
- [X] T034 [P] [US1] Create Tauri service wrapper in frontend/src/services/tauri.ts
- [X] T035 [P] [US1] Implement openImage function in frontend/src/services/tauri.ts

**Checkpoint**: At this point, User Story 1 should be fully functional - can open and preview images

---

## Phase 4: User Story 2 - Filter Operations (Priority: P2)

**Goal**: Users can apply visual filters (grayscale, sepia, invert, blur, sharpen) to images

**Independent Test**: Load image → Apply grayscale filter → Preview updates → Can undo

### Implementation for User Story 2

- [ ] T036 [P] [US2] Create filters module in src/core/operations/filters.rs
- [ ] T037 [P] [US2] Implement grayscale filter in src/core/operations/filters.rs
- [ ] T038 [P] [US2] Implement sepia filter in src/core/operations/filters.rs
- [ ] T039 [P] [US2] Implement invert filter in src/core/operations/filters.rs
- [ ] T040 [P] [US2] Implement blur filter with radius parameter in src/core/operations/filters.rs
- [ ] T041 [P] [US2] Implement sharpen filter in src/core/operations/filters.rs
- [ ] T042 [US2] Create ImageProcessor structure in src/core/image_processor.rs
- [ ] T043 [US2] Implement apply_filter method with FilterType dispatch in src/core/image_processor.rs
- [ ] T044 [US2] Add Rayon parallel processing for filter operations in src/core/image_processor.rs
- [ ] T045 [US2] Implement apply_operation method in src/state/image_state.rs
- [ ] T046 [US2] Implement render_with_operations method in src/state/image_state.rs
- [ ] T047 [US2] Implement apply_operation Tauri command in src/commands/image_commands.rs
- [ ] T048 [US2] Add Specta annotations to apply_operation command in src/commands/image_commands.rs
- [ ] T049 [US2] Register apply_operation command in src/main.rs
- [ ] T050 [US2] Add filter parameter validation in src/commands/image_commands.rs
- [ ] T051 [P] [US2] Update TypeScript bindings for filter operations in frontend/src/types/bindings.ts
- [ ] T052 [P] [US2] Implement applyFilter function in frontend/src/services/tauri.ts

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Adjustments (Priority: P3)

**Goal**: Users can adjust brightness, contrast, saturation, hue, and gamma

**Independent Test**: Load image → Adjust brightness +20% → Preview updates → Values stay in range

### Implementation for User Story 3

- [ ] T053 [P] [US3] Create adjustments module in src/core/operations/adjustments.rs
- [ ] T054 [US3] Implement adjust_pixel helper function in src/core/operations/adjustments.rs
- [ ] T055 [US3] Implement brightness adjustment with Rayon parallelism in src/core/operations/adjustments.rs
- [ ] T056 [US3] Implement contrast adjustment with Rayon parallelism in src/core/operations/adjustments.rs
- [ ] T057 [US3] Implement saturation adjustment with HSL conversion in src/core/operations/adjustments.rs
- [ ] T058 [US3] Implement hue adjustment with HSL rotation in src/core/operations/adjustments.rs
- [ ] T059 [US3] Implement gamma correction in src/core/operations/adjustments.rs
- [ ] T060 [US3] Implement apply_adjustment method in src/core/image_processor.rs
- [ ] T061 [US3] Extend apply_operation to handle Adjustment params in src/state/image_state.rs
- [ ] T062 [US3] Add adjustment parameter validation (0.0-2.0 range) in src/commands/image_commands.rs
- [ ] T063 [US3] Add hue validation (-180 to 180) in src/commands/image_commands.rs
- [ ] T064 [US3] Add gamma validation (0.1 to 3.0) in src/commands/image_commands.rs
- [ ] T065 [P] [US3] Update TypeScript bindings for adjustments in frontend/src/types/bindings.ts
- [ ] T066 [P] [US3] Add validation helpers in frontend/src/types/bindings.ts

**Checkpoint**: Image loading, filters, and adjustments all working independently

---

## Phase 6: User Story 4 - Transforms (Priority: P4)

**Goal**: Users can rotate (90°, 180°, 270°) and flip (horizontal, vertical) images

**Independent Test**: Load image → Rotate 90° → Dimensions change → Can rotate back

### Implementation for User Story 4

- [ ] T067 [P] [US4] Create transform module in src/core/operations/transform.rs
- [ ] T068 [P] [US4] Implement rotate90, rotate180, rotate270 using image crate in src/core/operations/transform.rs
- [ ] T069 [P] [US4] Implement flip_horizontal and flip_vertical in src/core/operations/transform.rs
- [ ] T070 [US4] Implement apply_transform method in src/core/image_processor.rs
- [ ] T071 [US4] Extend apply_operation to handle Transform params in src/state/image_state.rs
- [ ] T072 [US4] Update dimension tracking after transforms in src/state/image_state.rs
- [ ] T073 [P] [US4] Update TypeScript bindings for transforms in frontend/src/types/bindings.ts
- [ ] T074 [P] [US4] Implement applyTransform function in frontend/src/services/tauri.ts

**Checkpoint**: All basic editing operations (filters, adjustments, transforms) functional

---

## Phase 7: User Story 5 - Crop Operations (Priority: P5)

**Goal**: Users can select and crop rectangular regions of images

**Independent Test**: Load image → Define crop rect (100,100,800,600) → Image cropped → Dimensions updated

### Implementation for User Story 5

- [ ] T075 [P] [US5] Create crop module in src/core/operations/crop.rs
- [ ] T076 [US5] Implement crop_image with bounds validation in src/core/operations/crop.rs
- [ ] T077 [US5] Implement aspect ratio maintenance logic in src/core/operations/crop.rs
- [ ] T078 [US5] Implement apply_crop method in src/core/image_processor.rs
- [ ] T079 [US5] Extend apply_operation to handle Crop params in src/state/image_state.rs
- [ ] T080 [US5] Add crop rectangle validation (within bounds) in src/commands/image_commands.rs
- [ ] T081 [US5] Add minimum dimension validation (1x1) in src/commands/image_commands.rs
- [ ] T082 [P] [US5] Update TypeScript bindings for crop operations in frontend/src/types/bindings.ts
- [ ] T083 [P] [US5] Implement cropImage function in frontend/src/services/tauri.ts
- [ ] T084 [P] [US5] Add crop rectangle validation helper in frontend/src/types/bindings.ts

**Checkpoint**: Full editing toolset available (filters, adjustments, transforms, crop)

---

## Phase 8: User Story 6 - History Management (Priority: P6)

**Goal**: Users can undo and redo any sequence of operations

**Independent Test**: Apply 5 operations → Undo 3 times → Redo 2 times → Correct state restored

### Implementation for User Story 6

- [ ] T085 [US6] Implement generate_preview Tauri command in src/commands/image_commands.rs
- [ ] T086 [US6] Add Specta annotations to generate_preview command in src/commands/image_commands.rs
- [ ] T087 [US6] Implement undo Tauri command in src/commands/image_commands.rs
- [ ] T088 [US6] Add Specta annotations to undo command in src/commands/image_commands.rs
- [ ] T089 [US6] Implement redo Tauri command in src/commands/image_commands.rs
- [ ] T090 [US6] Add Specta annotations to redo command in src/commands/image_commands.rs
- [ ] T091 [US6] Implement get_history_state Tauri command in src/commands/state_commands.rs
- [ ] T092 [US6] Add Specta annotations to get_history_state command in src/commands/state_commands.rs
- [ ] T093 [US6] Implement clear_image Tauri command in src/commands/state_commands.rs
- [ ] T094 [US6] Register undo, redo, get_history_state, clear_image in src/main.rs
- [ ] T095 [US6] Add error handling for empty history/redo stacks in src/commands/image_commands.rs
- [ ] T096 [P] [US6] Implement preview cache with operations hash in src/state/image_state.rs
- [ ] T097 [P] [US6] Update TypeScript bindings for history commands in frontend/src/types/bindings.ts
- [ ] T098 [P] [US6] Implement undo/redo functions in frontend/src/services/tauri.ts
- [ ] T099 [P] [US6] Implement getHistoryState function in frontend/src/services/tauri.ts

**Checkpoint**: Complete editing workflow with undo/redo working correctly

---

## Phase 9: User Story 7 - Export (Priority: P7)

**Goal**: Users can export edited images in JPEG, PNG, or WebP formats with quality settings

**Independent Test**: Apply operations → Export as JPEG quality 95 → File saved → Original unchanged

### Implementation for User Story 7

- [ ] T100 [P] [US7] Create export_engine module in src/core/export_engine.rs
- [ ] T101 [US7] Implement export method with format dispatch in src/core/export_engine.rs
- [ ] T102 [US7] Implement JPEG encoding with quality parameter in src/core/export_engine.rs
- [ ] T103 [US7] Implement PNG encoding with compression in src/core/export_engine.rs
- [ ] T104 [US7] Implement WebP encoding with quality parameter in src/core/export_engine.rs
- [ ] T105 [US7] Implement apply_all_operations pipeline in src/core/export_engine.rs
- [ ] T106 [US7] Create export commands module in src/commands/export_commands.rs
- [ ] T107 [US7] Implement export_image Tauri command in src/commands/export_commands.rs
- [ ] T108 [US7] Add Specta annotations to export_image command in src/commands/export_commands.rs
- [ ] T109 [US7] Register export_image command in src/main.rs
- [ ] T110 [US7] Add output path validation (writable location) in src/commands/export_commands.rs
- [ ] T111 [US7] Add format validation (jpeg/png/webp only) in src/commands/export_commands.rs
- [ ] T112 [US7] Add quality parameter validation (1-100) in src/commands/export_commands.rs
- [ ] T113 [US7] Add file size calculation and return in src/commands/export_commands.rs
- [ ] T114 [P] [US7] Update TypeScript bindings for export command in frontend/src/types/bindings.ts
- [ ] T115 [P] [US7] Implement exportImage function in frontend/src/services/tauri.ts

**Checkpoint**: All user stories complete - full image editing application functional

---

## Phase 10: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T116 [P] Add logging infrastructure with tracing crate in src/lib.rs
- [ ] T117 [P] Add operation logging in src/core/image_processor.rs
- [ ] T118 [P] Add command logging in all command modules
- [ ] T119 [P] Implement memory usage monitoring in src/state/image_state.rs
- [ ] T120 [P] Add progressive quality degradation on memory pressure in src/state/image_state.rs
- [ ] T121 [P] Create unit tests for HistoryManager in tests/unit/history_manager_test.rs
- [ ] T122 [P] Create unit tests for ImageProcessor filters in tests/unit/image_processor_test.rs
- [ ] T123 [P] Create unit tests for Base64 utilities in tests/unit/base64_test.rs
- [ ] T124 [P] Create integration test for image loading workflow in tests/integration/image_load_test.rs
- [ ] T125 [P] Create integration test for filter application in tests/integration/filter_test.rs
- [ ] T126 [P] Create integration test for export workflow in tests/integration/export_test.rs
- [ ] T127 [P] Add security audit with cargo-audit in CI
- [ ] T128 [P] Add dependency license checking with cargo-deny
- [ ] T129 [P] Create README.md with quickstart instructions
- [ ] T130 [P] Document all public APIs with rustdoc comments
- [ ] T131 Run complete validation per quickstart.md Phase 1-8
- [ ] T132 Create release build and test cross-platform (macOS, Windows, Linux)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) - BLOCKS all user stories
- **User Stories (Phases 3-9)**: All depend on Foundational phase completion
  - Can proceed in parallel if team capacity allows
  - Recommended sequential: US1 → US2 → US3 → US4 → US5 → US6 → US7
- **Polish (Phase 10)**: Depends on desired user stories being complete

### User Story Dependencies

- **US1 (Image Loading)**: Foundation only - no other story dependencies
- **US2 (Filters)**: Depends on US1 (needs image loading) - can extend independently
- **US3 (Adjustments)**: Depends on US1 - can run parallel with US2
- **US4 (Transforms)**: Depends on US1 - can run parallel with US2/US3
- **US5 (Crop)**: Depends on US1 - can run parallel with US2/US3/US4
- **US6 (History)**: Depends on US1-US5 (needs operations to undo)
- **US7 (Export)**: Depends on US1-US5 (needs operations to apply)

### Within Each User Story

Standard pattern for all stories:
1. Core module implementation (operations/)
2. ImageProcessor integration
3. ImageState integration
4. Command layer (Tauri commands)
5. Input validation
6. TypeScript bindings
7. Frontend service wrapper

### Parallel Opportunities

**Setup Phase**: T002, T003, T004, T005, T006, T007 can all run in parallel

**Foundational Phase**: 
- T008-T012 (all type definitions) can run in parallel
- T017-T020 (all utility modules) can run in parallel

**User Story 1**: T023-T025, T032-T035 can run in parallel within their groups

**User Story 2**: T036-T041 (all filter implementations) can run in parallel

**User Story 3**: T053-T059 (all adjustment implementations) can run in parallel

**User Story 4**: T067-T069 (all transform implementations) can run in parallel

**User Story 5**: T082-T084 (frontend tasks) can run in parallel

**User Story 6**: T096-T099 (frontend tasks) can run in parallel

**User Story 7**: T102-T104 (format encoders) can run in parallel

**Polish Phase**: T116-T130 (most polish tasks) can run in parallel

**Parallel User Stories** (if team capacity):
- After Foundation: US2, US3, US4, US5 can all be worked on in parallel by different developers
- US6 waits for at least US1-US2 to be complete
- US7 waits for at least US1-US2 to be complete

---

## Parallel Example: User Story 2 (Filters)

```bash
# Launch all filter implementations in parallel:
Task T036: "Create filters module in src/core/operations/filters.rs"
Task T037: "Implement grayscale filter in src/core/operations/filters.rs"
Task T038: "Implement sepia filter in src/core/operations/filters.rs"
Task T039: "Implement invert filter in src/core/operations/filters.rs"
Task T040: "Implement blur filter with radius parameter in src/core/operations/filters.rs"
Task T041: "Implement sharpen filter in src/core/operations/filters.rs"

# Then sequentially:
Task T042: "Create ImageProcessor structure" (depends on filters)
Task T043: "Implement apply_filter method" (depends on T042)
# ... and so on
```

---

## Implementation Strategy

### MVP First (User Story 1-2 Only)

1. Complete Phase 1: Setup (T001-T007)
2. Complete Phase 2: Foundational (T008-T022) **CRITICAL**
3. Complete Phase 3: User Story 1 - Image Loading (T023-T035)
4. **VALIDATE**: Test image loading independently
5. Complete Phase 4: User Story 2 - Filters (T036-T052)
6. **VALIDATE**: Test filters on loaded images
7. **Deploy/Demo MVP**: Basic image editor with filters

### Full Feature Set

Continue through US3-US7 in priority order:
- US3: Adjustments (brightness, contrast, etc.)
- US4: Transforms (rotate, flip)
- US5: Crop
- US6: History (undo/redo)
- US7: Export

Each story adds value without breaking previous functionality.

### Parallel Team Strategy

With 3 developers after Foundation complete:
- **Dev A**: US1 (Image Loading) → US6 (History) → US7 (Export)
- **Dev B**: US2 (Filters) → US3 (Adjustments)
- **Dev C**: US4 (Transforms) → US5 (Crop)

Stories integrate at Phase 10 for final polish.

---

## Task Summary

- **Total Tasks**: 132
- **Phase 1 (Setup)**: 7 tasks
- **Phase 2 (Foundation)**: 15 tasks (BLOCKING)
- **Phase 3 (US1 - Image Loading)**: 13 tasks
- **Phase 4 (US2 - Filters)**: 17 tasks
- **Phase 5 (US3 - Adjustments)**: 14 tasks
- **Phase 6 (US4 - Transforms)**: 8 tasks
- **Phase 7 (US5 - Crop)**: 10 tasks
- **Phase 8 (US6 - History)**: 15 tasks
- **Phase 9 (US7 - Export)**: 16 tasks
- **Phase 10 (Polish)**: 17 tasks

**Parallel Opportunities**: 45+ tasks marked with [P] can run simultaneously
**MVP Scope**: Phases 1-4 (52 tasks) = Image loading + Filters
**Suggested MVP**: Complete through US2 for demo-ready image editor

---

## Notes

- All tasks follow strict checklist format: `- [ ] [ID] [P?] [Story?] Description with path`
- Each user story is independently completable and testable
- Foundation (Phase 2) MUST complete before any user story work begins
- Commit frequently after each task or logical group
- Run `cargo test` and `cargo clippy` before each commit
- Generate TypeScript bindings after Rust type changes
- Validate each story independently at checkpoints

---

**Generated**: 2025-12-30  
**Format Validation**: ✅ All 132 tasks follow checklist format with IDs, labels, and file paths
