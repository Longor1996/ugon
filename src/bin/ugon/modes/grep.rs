use super::*;

#[derive(Debug, Args)]
pub struct GrepMode {
    /// The ugon file to print.
    /// 
    /// Set to `-` to use STDIN.
    #[arg()]
    pub input: PathBuf,
    
    /// Recreate a ugon file from grep'd output.
    #[arg(short, long)]
    pub ungrep: bool,
    
    /// Print only a subset of the ugon file.
    #[arg(short, long, default_value = ".")]
    pub path: Option<String>,
}
