#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenerateThumbTask {
    pub dir: String,
    pub id: String,
    /** The index of the file being processed in dir */
    pub i: u32,
    /** The total number of files in the directory */
    pub total: u32,
    pub started: bool, // true if the task has started processing
}

// Define different event types (must be Send + Sync + 'static)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Task {
    GenerateThumb(GenerateThumbTask),
}
