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
