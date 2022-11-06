use super::*;

#[derive(Debug, Args)]
pub struct EditMode {
    /// The ugon file to edit.
    /// 
    /// Must be a valid file, cannot use STDIN.
    #[arg()]
    pub input: PathBuf,
    
    /// Make editing relative to the given subset of the file.
    #[arg(short, long, default_value = ".")]
    pub path: Option<String>,
}
