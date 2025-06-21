#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenerateThumbTask {
    pub dir: String,
    pub id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalyzeVideoTask {
    pub dir: String,
    pub id: String,
}

// Define different event types (must be Send + Sync + 'static)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Task {
    GenerateThumb(GenerateThumbTask),
    AnalyzeVideo(AnalyzeVideoTask),

    // Test task events
    Log { message: String },
    UserAction { user_id: u32, action: String },
    SystemAlert { code: u16, description: String },
}
