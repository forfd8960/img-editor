<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { save } from '@tauri-apps/plugin-dialog';
  import { 
    openImage, 
    applyOperation, 
    createFilterOperation, 
    createAdjustmentOperation,
    createTransformOperation,
    createCropOperation,
    validateAdjustment,
    validateCrop,
    exportImage,
    validateExport,
    undo,
    redo
  } from './services/tauri';
  import type { OpenImageOutput, FilterType, AdjustmentParams, TransformType, CropRect, ExportParams } from './services/tauri';

  let imageData: OpenImageOutput | null = null;
  let loading = false;
  let error: string | null = null;
  let blurRadius = 2.0;
  
  // Adjustment controls
  let brightness = 1.0;
  let contrast = 1.0;
  let saturation = 1.0;
  let hue = 0;
  let gamma = 1.0;
  
  // Crop controls
  let cropX = 0;
  let cropY = 0;
  let cropWidth = 100;
  let cropHeight = 100;
  
  // Export controls
  let exportFormat = 'jpeg';
  let exportQuality = 95;

  async function handleOpenImage() {
    try {
      loading = true;
      error = null;

      // Open file dialog
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Image',
          extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'webp', 'tiff', 'tif']
        }]
      });

      if (!selected) {
        loading = false;
        return;
      }

      const filePath = typeof selected === 'string' ? selected : selected.path;

      // Load image via Tauri backend
      const result = await openImage(filePath, 800, 600);
      imageData = result;
      
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to open image:', err);
    } finally {
      loading = false;
    }
  }

  async function applyFilter(filterType: 'grayscale' | 'sepia' | 'invert' | 'sharpen' | 'blur') {
    if (!imageData) return;
    
    try {
      loading = true;
      error = null;

      let filter: FilterType;
      if (filterType === 'blur') {
        filter = { type: 'blur', radius: blurRadius };
      } else {
        filter = { type: filterType };
      }

      const operation = createFilterOperation(filter);
      const result = await applyOperation(operation, 800);
      
      // Update preview
      imageData = {
        ...imageData,
        preview_base64: result.preview_base64,
        original_width: result.new_width,
        original_height: result.new_height,
      };
      
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to apply filter:', err);
    } finally {
      loading = false;
    }
  }

  async function applyAdjustment(type: 'brightness' | 'contrast' | 'saturation' | 'hue' | 'gamma' | 'all') {
    if (!imageData) return;
    
    try {
      loading = true;
      error = null;

      const params: AdjustmentParams = {};
      
      if (type === 'all') {
        params.brightness = brightness !== 1.0 ? brightness : undefined;
        params.contrast = contrast !== 1.0 ? contrast : undefined;
        params.saturation = saturation !== 1.0 ? saturation : undefined;
        params.hue = hue !== 0 ? hue : undefined;
        params.gamma = gamma !== 1.0 ? gamma : undefined;
      } else {
        switch (type) {
          case 'brightness': params.brightness = brightness; break;
          case 'contrast': params.contrast = contrast; break;
          case 'saturation': params.saturation = saturation; break;
          case 'hue': params.hue = hue; break;
          case 'gamma': params.gamma = gamma; break;
        }
      }

      // Validate before sending
      const validationError = validateAdjustment(params);
      if (validationError) {
        error = validationError;
        return;
      }

      const operation = createAdjustmentOperation(params);
      const result = await applyOperation(operation, 800);
      
      // Update preview
      imageData = {
        ...imageData,
        preview_base64: result.preview_base64,
        original_width: result.new_width,
        original_height: result.new_height,
      };
      
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to apply adjustment:', err);
    } finally {
      loading = false;
    }
  }

  function resetAdjustments() {
    brightness = 1.0;
    contrast = 1.0;
    saturation = 1.0;
    hue = 0;
    gamma = 1.0;
  }

  async function applyTransform(transformType: 'rotate90' | 'rotate180' | 'rotate270' | 'flip_horizontal' | 'flip_vertical') {
    if (!imageData) return;
    
    try {
      loading = true;
      error = null;

      const transform: TransformType = { type: transformType };
      const operation = createTransformOperation(transform);
      const result = await applyOperation(operation, 800);
      
      // Update preview with new dimensions
      imageData = {
        ...imageData,
        preview_base64: result.preview_base64,
        original_width: result.new_width,
        original_height: result.new_height,
      };
      
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to apply transform:', err);
    } finally {
      loading = false;
    }
  }

  async function handleUndo() {
    if (!imageData) return;
    
    try {
      loading = true;
      error = null;

      const result = await undo();
      
      // Update preview
      imageData = {
        ...imageData,
        preview_base64: result.preview_base64,
        original_width: result.new_width,
        original_height: result.new_height,
      };
      
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to undo:', err);
    } finally {
      loading = false;
    }
  }

  async function handleRedo() {
    if (!imageData) return;
    
    try {
      loading = true;
      error = null;

      const result = await redo();
      
      // Update preview
      imageData = {
        ...imageData,
        preview_base64: result.preview_base64,
        original_width: result.new_width,
        original_height: result.new_height,
      };
      
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to redo:', err);
    } finally {
      loading = false;
    }
  }

  async function applyCrop() {
    if (!imageData) return;
    
    try {
      loading = true;
      error = null;

      const rect: CropRect = {
        x: cropX,
        y: cropY,
        width: cropWidth,
        height: cropHeight
      };

      // Validate crop rectangle
      const validationError = validateCrop(rect, imageData.original_width, imageData.original_height);
      if (validationError) {
        error = validationError;
        return;
      }

      const operation = createCropOperation(rect);
      const result = await applyOperation(operation);
      
      // Update preview
      imageData = {
        ...imageData,
        preview_base64: result.preview_base64,
        original_width: result.new_width,
        original_height: result.new_height,
      };
      
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to apply crop:', err);
    } finally {
      loading = false;
    }
  }

  async function handleExport() {
    if (!imageData) return;
    
    try {
      loading = true;
      error = null;

      // Open save dialog
      const outputPath = await save({
        filters: [{
          name: 'Image',
          extensions: exportFormat === 'jpeg' ? ['jpg', 'jpeg'] : [exportFormat]
        }],
        defaultPath: `edited-image.${exportFormat === 'jpeg' ? 'jpg' : exportFormat}`
      });

      if (!outputPath) {
        loading = false;
        return;
      }

      const filePath = typeof outputPath === 'string' ? outputPath : outputPath.path;

      const params: ExportParams = {
        output_path: filePath,
        format: exportFormat,
        quality: exportQuality
      };

      // Validate export parameters
      const validationError = validateExport(params);
      if (validationError) {
        error = validationError;
        return;
      }

      const result = await exportImage(params);
      
      // Show success message
      const sizeKB = (result.file_size / 1024).toFixed(2);
      alert(`Image exported successfully!\nPath: ${result.path}\nSize: ${sizeKB} KB`);
      
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Failed to export image:', err);
    } finally {
      loading = false;
    }
  }
</script>

<main>
  <div class="header">
    <h1>Image Editor</h1>
    <div class="header-buttons">
      <button on:click={handleOpenImage} disabled={loading}>
        {loading ? 'Loading...' : 'Open Image'}
      </button>
      {#if imageData}
        <button on:click={handleUndo} disabled={loading} class="secondary">
          ↶ Undo
        </button>
        <button on:click={handleRedo} disabled={loading} class="secondary">
          ↷ Redo
        </button>
      {/if}
    </div>
  </div>

  {#if error}
    <div class="error">
      <p>Error: {error}</p>
    </div>
  {/if}

  {#if imageData}
    <div class="controls">
      <div class="filter-group">
        <h3>Filters</h3>
        <div class="filter-buttons">
          <button on:click={() => applyFilter('grayscale')} disabled={loading}>
            Grayscale
          </button>
          <button on:click={() => applyFilter('sepia')} disabled={loading}>
            Sepia
          </button>
          <button on:click={() => applyFilter('invert')} disabled={loading}>
            Invert
          </button>
          <button on:click={() => applyFilter('sharpen')} disabled={loading}>
            Sharpen
          </button>
        </div>
      </div>

      <div class="filter-group">
        <h3>Blur</h3>
        <div class="blur-control">
          <input 
            type="range" 
            min="0.1" 
            max="10" 
            step="0.1" 
            bind:value={blurRadius}
            disabled={loading}
          />
          <span>{blurRadius.toFixed(1)}px</span>
          <button on:click={() => applyFilter('blur')} disabled={loading}>
            Apply Blur
          </button>
        </div>
      </div>

      <div class="filter-group adjustments">
        <h3>Adjustments</h3>
        
        <div class="adjustment-control">
          <label>
            <span>Brightness</span>
            <span class="value">{brightness.toFixed(2)}</span>
          </label>
          <input 
            type="range" 
            min="0.0" 
            max="2.0" 
            step="0.01" 
            bind:value={brightness}
            disabled={loading}
          />
        </div>

        <div class="adjustment-control">
          <label>
            <span>Contrast</span>
            <span class="value">{contrast.toFixed(2)}</span>
          </label>
          <input 
            type="range" 
            min="0.0" 
            max="2.0" 
            step="0.01" 
            bind:value={contrast}
            disabled={loading}
          />
        </div>

        <div class="adjustment-control">
          <label>
            <span>Saturation</span>
            <span class="value">{saturation.toFixed(2)}</span>
          </label>
          <input 
            type="range" 
            min="0.0" 
            max="2.0" 
            step="0.01" 
            bind:value={saturation}
            disabled={loading}
          />
        </div>

        <div class="adjustment-control">
          <label>
            <span>Hue</span>
            <span class="value">{hue}°</span>
          </label>
          <input 
            type="range" 
            min="-180" 
            max="180" 
            step="1" 
            bind:value={hue}
            disabled={loading}
          />
        </div>

        <div class="adjustment-control">
          <label>
            <span>Gamma</span>
            <span class="value">{gamma.toFixed(2)}</span>
          </label>
          <input 
            type="range" 
            min="0.1" 
            max="3.0" 
            step="0.01" 
            bind:value={gamma}
            disabled={loading}
          />
        </div>

        <div class="adjustment-buttons">
          <button on:click={() => applyAdjustment('all')} disabled={loading}>
            Apply All
          </button>
          <button on:click={resetAdjustments} disabled={loading} class="secondary">
            Reset
          </button>
        </div>
      </div>

      <div class="filter-group">
        <h3>Transform</h3>
        <div class="transform-buttons">
          <button on:click={() => applyTransform('rotate90')} disabled={loading}>
            ↻ 90°
          </button>
          <button on:click={() => applyTransform('rotate180')} disabled={loading}>
            ↻ 180°
          </button>
          <button on:click={() => applyTransform('rotate270')} disabled={loading}>
            ↺ 90°
          </button>
          <button on:click={() => applyTransform('flip_horizontal')} disabled={loading}>
            ↔ Flip H
          </button>
          <button on:click={() => applyTransform('flip_vertical')} disabled={loading}>
            ↕ Flip V
          </button>
        </div>
      </div>

      <div class="filter-group crop-section">
        <h3>Crop</h3>
        <div class="crop-controls">
          <div class="crop-input-group">
            <label>
              <span>X</span>
              <input 
                type="number" 
                min="0" 
                max="{imageData.original_width - 1}"
                bind:value={cropX}
                disabled={loading}
              />
            </label>
            <label>
              <span>Y</span>
              <input 
                type="number" 
                min="0" 
                max="{imageData.original_height - 1}"
                bind:value={cropY}
                disabled={loading}
              />
            </label>
          </div>
          <div class="crop-input-group">
            <label>
              <span>Width</span>
              <input 
                type="number" 
                min="1" 
                max="{imageData.original_width}"
                bind:value={cropWidth}
                disabled={loading}
              />
            </label>
            <label>
              <span>Height</span>
              <input 
                type="number" 
                min="1" 
                max="{imageData.original_height}"
                bind:value={cropHeight}
                disabled={loading}
              />
            </label>
          </div>
          <button on:click={applyCrop} disabled={loading} class="apply-crop">
            ✂ Apply Crop
          </button>
        </div>
      </div>
    </div>

    <div class="export-section">
      <h2>Export Image</h2>
      <div class="export-controls">
        <div class="export-format">
          <label>
            <span>Format</span>
            <select bind:value={exportFormat} disabled={loading}>
              <option value="jpeg">JPEG</option>
              <option value="png">PNG</option>
              <option value="webp">WebP</option>
            </select>
          </label>
        </div>
        
        <div class="export-quality">
          <label>
            <span>Quality</span>
            <span class="value">{exportQuality}</span>
          </label>
          <input 
            type="range" 
            min="1" 
            max="100" 
            step="1" 
            bind:value={exportQuality}
            disabled={loading}
          />
        </div>
        
        <button on:click={handleExport} disabled={loading} class="export-button">
          💾 Save Image
        </button>
      </div>
    </div>

    <div class="image-container">
      <div class="metadata">
        <span>Format: {imageData.format}</span>
        <span>Size: {imageData.original_width} x {imageData.original_height}</span>
      </div>
      <img 
        src={imageData.preview_base64} 
        alt="Preview"
        class="preview-image"
      />
    </div>
  {:else if !loading}
    <div class="placeholder">
      <p>Click "Open Image" to get started</p>
    </div>
  {/if}
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #f5f5f5;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
    background: white;
    border-bottom: 1px solid #e0e0e0;
  }

  .header-buttons {
    display: flex;
    gap: 0.5rem;
  }

  h1 {
    margin: 0;
    font-size: 1.5rem;
    color: #333;
  }

  h3 {
    margin: 0 0 0.5rem 0;
    font-size: 0.9rem;
    color: #666;
    font-weight: 600;
  }

  button {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    background: #4A90E2;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.2s;
  }

  button:hover:not(:disabled) {
    background: #357ABD;
  }

  button:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .controls {
    padding: 1rem 2rem;
    background: white;
    border-bottom: 1px solid #e0e0e0;
    display: flex;
    gap: 2rem;
    flex-wrap: wrap;
  }

  .filter-group {
    display: flex;
    flex-direction: column;
  }

  .filter-buttons {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .filter-buttons button {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
  }

  .blur-control {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .blur-control input[type="range"] {
    width: 150px;
  }

  .blur-control span {
    min-width: 50px;
    font-size: 0.875rem;
    color: #666;
  }

  .blur-control button {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
  }

  .adjustments {
    min-width: 250px;
  }

  .adjustment-control {
    margin-bottom: 1rem;
  }

  .adjustment-control label {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.25rem;
    font-size: 0.875rem;
    color: #666;
  }

  .adjustment-control .value {
    font-weight: 600;
    color: #4A90E2;
  }

  .adjustment-control input[type="range"] {
    width: 100%;
    margin: 0;
  }

  .adjustment-buttons {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .adjustment-buttons button {
    flex: 1;
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
  }

  button.secondary {
    background: #757575;
  }

  button.secondary:hover:not(:disabled) {
    background: #616161;
  }

  .transform-buttons {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .transform-buttons button {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    min-width: 70px;
  }

  .crop-section {
    min-width: 250px;
  }

  .crop-controls {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .crop-input-group {
    display: flex;
    gap: 1rem;
  }

  .crop-input-group label {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .crop-input-group label span {
    font-size: 0.875rem;
    color: #666;
    font-weight: 500;
  }

  .crop-input-group input[type="number"] {
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  .crop-input-group input[type="number"]:focus {
    outline: none;
    border-color: #4A90E2;
  }

  .apply-crop {
    margin-top: 0.5rem;
    width: 100%;
  }

  .export-section {
    padding: 1.5rem 2rem;
    background: #f8f9fa;
    border-top: 2px solid #4A90E2;
    border-bottom: 1px solid #e0e0e0;
  }

  .export-section h2 {
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
    color: #333;
  }

  .export-controls {
    display: flex;
    align-items: flex-end;
    gap: 1.5rem;
    flex-wrap: wrap;
  }

  .export-format {
    min-width: 150px;
  }

  .export-format label {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .export-format label span {
    font-size: 0.875rem;
    color: #666;
    font-weight: 500;
  }

  .export-format select {
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.875rem;
    background: white;
    cursor: pointer;
  }

  .export-format select:focus {
    outline: none;
    border-color: #4A90E2;
  }

  .export-quality {
    flex: 1;
    min-width: 200px;
  }

  .export-quality label {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.5rem;
    font-size: 0.875rem;
    color: #666;
  }

  .export-quality .value {
    font-weight: 600;
    color: #4A90E2;
  }

  .export-quality input[type="range"] {
    width: 100%;
  }

  .export-button {
    padding: 0.75rem 2rem;
    font-size: 1rem;
    font-weight: 600;
    background: #2e7d32;
    min-width: 150px;
  }

  .export-button:hover:not(:disabled) {
    background: #1b5e20;
  }

  .error {
    margin: 1rem 2rem;
    padding: 1rem;
    background: #ffebee;
    border-left: 4px solid #f44336;
    border-radius: 4px;
  }

  .error p {
    margin: 0;
    color: #c62828;
  }

  .placeholder {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #999;
  }

  .image-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 2rem;
    overflow: auto;
  }

  .metadata {
    display: flex;
    gap: 2rem;
    margin-bottom: 1rem;
    padding: 0.75rem 1rem;
    background: white;
    border-radius: 4px;
    font-size: 0.9rem;
    color: #666;
  }

  .preview-image {
    max-width: 100%;
    max-height: calc(100vh - 300px);
    object-fit: contain;
    background: white;
    border-radius: 4px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }
</style>
