use std::path::PathBuf;

#[derive(Default)]
pub struct CodeEditorState {
    /// Path to the currently open file.
    pub open_file: Option<PathBuf>,
    /// `None` means the list has not been loaded yet.
    pub file_list: Option<Vec<PathBuf>>,
    /// The content at the point where the file was last modified.
    pub content: String,
    /// Whether the current content has unsaved changes.
    pub dirty: bool,
}
