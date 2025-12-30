use arc_swap::ArcSwap;
use image::DynamicImage;
use std::path::Path;
use std::sync::Arc;

use crate::core::history_manager::HistoryManager;
use crate::types::errors::AppError;
use crate::utils::preview::resize_to_fit;

/// Preview cache structure
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct PreviewCache {
    base64: String,
    width: u32,
    height: u32,
    operations_hash: u64,
}

impl Default for PreviewCache {
    fn default() -> Self {
        Self {
            base64: String::new(),
            width: 0,
            height: 0,
            operations_hash: 0,
        }
    }
}

/// Image state manager
pub struct ImageState {
    /// Original image (immutable, shared via ArcSwap)
    original_image: ArcSwap<Option<DynamicImage>>,

    /// Current image for preview
    current_image: ArcSwap<Option<DynamicImage>>,

    /// History manager
    history: Arc<HistoryManager>,

    /// Preview cache
    #[allow(dead_code)]
    preview_cache: ArcSwap<PreviewCache>,
}

impl ImageState {
    /// Create new image state
    pub fn new() -> Self {
        Self {
            original_image: ArcSwap::new(Arc::new(None)),
            current_image: ArcSwap::new(Arc::new(None)),
            history: Arc::new(HistoryManager::new()),
            preview_cache: ArcSwap::new(Arc::new(PreviewCache::default())),
        }
    }

    /// Get reference to original image
    #[allow(dead_code)]
    pub fn get_original(&self) -> Option<Arc<Option<DynamicImage>>> {
        Some(self.original_image.load_full())
    }

    /// Get reference to current image
    #[allow(dead_code)]
    pub fn get_current(&self) -> Option<Arc<Option<DynamicImage>>> {
        Some(self.current_image.load_full())
    }

    /// Get reference to history manager
    #[allow(dead_code)]
    pub fn history(&self) -> &Arc<HistoryManager> {
        &self.history
    }

    /// Store original image
    pub fn set_original(&self, image: DynamicImage) {
        self.original_image.store(Arc::new(Some(image.clone())));
        self.current_image.store(Arc::new(Some(image)));
    }

    /// Store current image
    #[allow(dead_code)]
    pub fn set_current(&self, image: DynamicImage) {
        self.current_image.store(Arc::new(Some(image)));
    }

    /// Clear all state
    #[allow(dead_code)]
    pub fn clear(&self) {
        self.original_image.store(Arc::new(None));
        self.current_image.store(Arc::new(None));
        self.history.clear();
        self.preview_cache.store(Arc::new(PreviewCache::default()));
    }

    /// Load original image from file path
    pub async fn load_original<P: AsRef<Path>>(&self, path: P) -> Result<(u32, u32, String), AppError> {
        let path = path.as_ref().to_path_buf();
        
        // Get format before moving path
        let format_str = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_uppercase())
            .unwrap_or_else(|| "UNKNOWN".to_string());
        
        // Load image in blocking thread
        let image = tokio::task::spawn_blocking(move || {
            image::open(&path).map_err(|e| AppError::ImageLoadError(e.to_string()))
        })
        .await
        .map_err(|e| AppError::ProcessingError { details: e.to_string() })??;

        let width = image.width();
        let height = image.height();

        // Store original and current
        self.set_original(image);
        
        // Clear history on new image
        self.history.clear();

        Ok((width, height, format_str))
    }

    /// Generate preview with maximum dimensions
    pub async fn generate_preview(&self, max_width: u32, max_height: u32) -> Result<String, AppError> {
        // Load current image
        let current = self.current_image.load();
        let image = current
            .as_ref()
            .as_ref()
            .ok_or_else(|| AppError::StateError { message: "No image loaded".to_string() })?;

        // Clone for async processing
        let image_clone = image.clone();
        
        // Resize and encode in blocking thread
        let base64 = tokio::task::spawn_blocking(move || {
            let preview = resize_to_fit(&image_clone, max_width, max_height);
            crate::utils::base64::encode_image(&preview)
        })
        .await
        .map_err(|e| AppError::ProcessingError { details: e.to_string() })??;

        Ok(base64)
    }
}

impl Default for ImageState {
    fn default() -> Self {
        Self::new()
    }
}
