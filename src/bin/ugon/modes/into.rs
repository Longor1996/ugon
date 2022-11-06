use super::*;

#[derive(Debug, Args)]
pub struct IntoMode {
    /// The ugon file to convert.
    /// 
    /// Set to `-` to use STDIN.
    #[arg()]
    pub input: PathBuf,
    
    /// The file to write to.
    /// 
    /// Set to `-` to use STDOUT.
    #[arg()]
    pub output: PathBuf,
    
    /// The format to write.
    #[arg(short = 'f', long = "format", default_value = "json")]
    pub format: Option<String>,
    
    /// Convert only a subset of the ugon tree.
    #[arg(short = 'p', long = "path", default_value = ".")]
    pub path: Option<String>,
}
