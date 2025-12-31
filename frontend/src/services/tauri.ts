import { invoke } from '@tauri-apps/api/core';

/**
 * Open image command input
 */
export interface OpenImageInput {
  file_path: string;
  preview_max_width: number;
  preview_max_height: number;
}

/**
 * Open image command output
 */
export interface OpenImageOutput {
  preview_base64: string;
  original_width: number;
  original_height: number;
  format: string;
}

/**
 * Filter types - matches Rust FilterType enum with tagged serialization
 */
export type FilterType =
  | { type: 'grayscale' }
  | { type: 'sepia' }
  | { type: 'invert' }
  | { type: 'blur'; radius: number }
  | { type: 'sharpen' };

/**
 * Adjustment parameters - matches Rust AdjustmentParams
 */
export interface AdjustmentParams {
  brightness?: number;  // 0.0-2.0
  contrast?: number;    // 0.0-2.0
  saturation?: number;  // 0.0-2.0
  hue?: number;         // -180 to 180
  gamma?: number;       // 0.1-3.0
}

/**
 * Transform types - matches Rust TransformType enum with tagged serialization
 */
export type TransformType =
  | { type: 'rotate90' }
  | { type: 'rotate180' }
  | { type: 'rotate270' }
  | { type: 'flip_horizontal' }
  | { type: 'flip_vertical' };

/**
 * Crop rectangle - matches Rust CropRect
 */
export interface CropRect {
  x: number;
  y: number;
  width: number;
  height: number;
}

/**
 * Operation types - matches Rust OperationType enum with adjacently tagged serialization
 */
export interface OperationType {
  operation_type: 'Filter' | 'Adjustment' | 'Transform' | 'Crop';
  params: FilterType | AdjustmentParams | TransformType | CropRect | Record<string, unknown>;
}

/**
 * Edit operation
 */
export interface EditOperation {
  id: string;
  operation: OperationType;
  timestamp: number;
}

/**
 * Apply operation input
 */
export interface ApplyOperationInput {
  operation: EditOperation;
  preview_width?: number;
}

/**
 * Apply operation output
 */
export interface ApplyOperationOutput {
  preview_base64: string;
  new_width: number;
  new_height: number;
}

/**
 * Application error from Rust backend
 */
export interface AppError {
  type: string;
  message: string;
}

/**
 * Open an image file and return metadata with preview
 * @param filePath - Absolute path to the image file
 * @param previewMaxWidth - Maximum width for preview (default: 800)
 * @param previewMaxHeight - Maximum height for preview (default: 600)
 * @returns Promise with image metadata and base64 preview
 */
export async function openImage(
  filePath: string,
  previewMaxWidth: number = 800,
  previewMaxHeight: number = 600
): Promise<OpenImageOutput> {
  try {
    const input: OpenImageInput = {
      file_path: filePath,
      preview_max_width: previewMaxWidth,
      preview_max_height: previewMaxHeight,
    };

    const result = await invoke<OpenImageOutput>('open_image', { input });
    return result;
  } catch (error) {
    // Re-throw with better error handling
    if (typeof error === 'string') {
      throw new Error(error);
    }
    throw error;
  }
}

/**
 * Apply a filter operation to the current image
 * @param operation - The edit operation to apply
 * @param previewWidth - Optional preview width (default: 800)
 * @returns Promise with updated preview
 */
export async function applyOperation(
  operation: EditOperation,
  previewWidth?: number
): Promise<ApplyOperationOutput> {
  try {
    const input: ApplyOperationInput = {
      operation,
      preview_width: previewWidth,
    };

    const result = await invoke<ApplyOperationOutput>('apply_operation', { input });
    return result;
  } catch (error) {
    if (typeof error === 'string') {
      throw new Error(error);
    }
    throw error;
  }
}

/**
 * Helper to create a filter operation
 */
export function createFilterOperation(filter: FilterType): EditOperation {
  return {
    id: crypto.randomUUID(),
    operation: {
      operation_type: 'Filter',
      params: filter,
    },
    timestamp: Date.now(),
  };
}

/**
 * Helper to create an adjustment operation
 */
export function createAdjustmentOperation(params: AdjustmentParams): EditOperation {
  return {
    id: crypto.randomUUID(),
    operation: {
      operation_type: 'Adjustment',
      params,
    },
    timestamp: Date.now(),
  };
}

/**
 * Validate adjustment parameters
 */
export function validateAdjustment(params: AdjustmentParams): string | null {
  if (params.brightness !== undefined && (params.brightness < 0.0 || params.brightness > 2.0)) {
    return 'Brightness must be between 0.0 and 2.0';
  }
  if (params.contrast !== undefined && (params.contrast < 0.0 || params.contrast > 2.0)) {
    return 'Contrast must be between 0.0 and 2.0';
  }
  if (params.saturation !== undefined && (params.saturation < 0.0 || params.saturation > 2.0)) {
    return 'Saturation must be between 0.0 and 2.0';
  }
  if (params.hue !== undefined && (params.hue < -180 || params.hue > 180)) {
    return 'Hue must be between -180 and 180';
  }
  if (params.gamma !== undefined && (params.gamma < 0.1 || params.gamma > 3.0)) {
    return 'Gamma must be between 0.1 and 3.0';
  }
  return null;
}

/**
 * Helper to create a transform operation
 */
export function createTransformOperation(transform: TransformType): EditOperation {
  return {
    id: crypto.randomUUID(),
    operation: {
      operation_type: 'Transform',
      params: transform,
    },
    timestamp: Date.now(),
  };
}

/**
 * Helper to create a crop operation
 */
export function createCropOperation(rect: CropRect): EditOperation {
  return {
    id: crypto.randomUUID(),
    operation: {
      operation_type: 'Crop',
      params: rect,
    },
    timestamp: Date.now(),
  };
}

/**
 * Validate crop rectangle
 */
export function validateCrop(rect: CropRect, imgWidth: number, imgHeight: number): string | null {
  if (rect.width <= 0 || rect.height <= 0) {
    return 'Crop dimensions must be at least 1x1';
  }
  if (rect.x >= imgWidth || rect.y >= imgHeight) {
    return `Crop position (${rect.x}, ${rect.y}) is outside image bounds`;
  }
  if (rect.x + rect.width > imgWidth || rect.y + rect.height > imgHeight) {
    return 'Crop rectangle extends beyond image bounds';
  }
  return null;
}

/**
 * Undo the last operation
 */
export async function undo(): Promise<ApplyOperationOutput> {
  try {
    const result = await invoke<ApplyOperationOutput>('undo');
    return result;
  } catch (error) {
    if (typeof error === 'string') {
      throw new Error(error);
    }
    throw error;
  }
}

/**
 * Redo the last undone operation
 */
export async function redo(): Promise<ApplyOperationOutput> {
  try {
    const result = await invoke<ApplyOperationOutput>('redo');
    return result;
  } catch (error) {
    if (typeof error === 'string') {
      throw new Error(error);
    }
    throw error;
  }
}

/**
 * Export parameters
 */
export interface ExportParams {
  output_path: string;
  format: string;
  quality: number;
}

/**
 * Export result
 */
export interface ExportResult {
  path: string;
  file_size: number;
  format: string;
}

/**
 * Export the current image to a file
 */
export async function exportImage(params: ExportParams): Promise<ExportResult> {
  try {
    const result = await invoke<ExportResult>('export_image_command', { params });
    return result;
  } catch (error) {
    if (typeof error === 'string') {
      throw new Error(error);
    }
    throw error;
  }
}

/**
 * Validate export parameters
 */
export function validateExport(params: ExportParams): string | null {
  if (!params.output_path || params.output_path.trim() === '') {
    return 'Output path is required';
  }

  const validFormats = ['jpeg', 'jpg', 'png', 'webp'];
  if (!validFormats.includes(params.format.toLowerCase())) {
    return `Invalid format: ${params.format}. Supported: JPEG, PNG, WebP`;
  }

  if (params.quality < 1 || params.quality > 100) {
    return `Quality must be 1-100, got ${params.quality}`;
  }

  return null;
}
