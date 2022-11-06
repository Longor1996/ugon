use super::*;

#[derive(Debug, Args)]
pub struct MakeMode {
    /// The file to convert to ugon.
    /// 
    /// Set to `-` to use STDIN.
    #[arg()]
    pub input: PathBuf,
    
    /// The ugon file to write to.
    /// 
    /// Set to `-` to use STDOUT.
    #[arg()]
    pub output: PathBuf,
    
    /// The format to read.
    #[arg(short = 'f', long = "format", default_value = "json")]
    pub format: Option<String>,
}
