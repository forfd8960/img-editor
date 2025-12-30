use crate::types::operations::EditOperation;
use std::sync::Mutex;

/// History manager for undo/redo operations
pub struct HistoryManager {
    history: Mutex<Vec<EditOperation>>,
    redo_stack: Mutex<Vec<EditOperation>>,
    #[allow(dead_code)]
    max_history: usize,
}

#[allow(dead_code)]
impl HistoryManager {
    /// Create a new history manager
    pub fn new() -> Self {
        Self {
            history: Mutex::new(Vec::new()),
            redo_stack: Mutex::new(Vec::new()),
            max_history: 50,
        }
    }

    /// Add an operation to history
    pub fn add_operation(&self, operation: EditOperation) -> Vec<EditOperation> {
        let mut history = self.history.lock().unwrap();
        let mut redo_stack = self.redo_stack.lock().unwrap();

        // Clear redo stack when new operation added
        redo_stack.clear();

        // Add to history
        history.push(operation);

        // Limit history size
        if history.len() > self.max_history {
            history.remove(0);
        }

        // Return current operations
        history.clone()
    }

    /// Undo last operation
    pub fn undo(&self) -> Option<Vec<EditOperation>> {
        let mut history = self.history.lock().unwrap();
        let mut redo_stack = self.redo_stack.lock().unwrap();

        if let Some(op) = history.pop() {
            redo_stack.push(op);
            return Some(history.clone());
        }

        None
    }

    /// Redo previously undone operation
    pub fn redo(&self) -> Option<Vec<EditOperation>> {
        let mut history = self.history.lock().unwrap();
        let mut redo_stack = self.redo_stack.lock().unwrap();

        if let Some(op) = redo_stack.pop() {
            history.push(op);
            return Some(history.clone());
        }

        None
    }

    /// Clear all history
    pub fn clear(&self) {
        let mut history = self.history.lock().unwrap();
        let mut redo_stack = self.redo_stack.lock().unwrap();

        history.clear();
        redo_stack.clear();
    }

    /// Get history count
    pub fn history_count(&self) -> usize {
        self.history.lock().unwrap().len()
    }

    /// Get redo count
    pub fn redo_count(&self) -> usize {
        self.redo_stack.lock().unwrap().len()
    }
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new()
    }
}
