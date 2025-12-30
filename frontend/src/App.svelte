<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { openImage } from './services/tauri';
  import type { OpenImageOutput } from './services/tauri';

  let imageData: OpenImageOutput | null = null;
  let loading = false;
  let error: string | null = null;

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
</script>

<main>
  <div class="header">
    <h1>Image Editor</h1>
    <button on:click={handleOpenImage} disabled={loading}>
      {loading ? 'Loading...' : 'Open Image'}
    </button>
  </div>

  {#if error}
    <div class="error">
      <p>Error: {error}</p>
    </div>
  {/if}

  {#if imageData}
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

  h1 {
    margin: 0;
    font-size: 1.5rem;
    color: #333;
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
    max-height: calc(100vh - 200px);
    object-fit: contain;
    background: white;
    border-radius: 4px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }
</style>
