#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenerateThumbTask {
    pub dir: String,
    pub file: String,
}

// Define different event types (must be Send + Sync + 'static)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Task {
    GenerateThumb(GenerateThumbTask),

    // Test task events
    Log { message: String },
    UserAction { user_id: u32, action: String },
    SystemAlert { code: u16, description: String },
}
