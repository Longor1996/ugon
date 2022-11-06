use super::*;

#[derive(Debug, Args)]
pub struct TreeMode {
    /// The ugon file to print.
    /// 
    /// Set to `-` to use STDIN.
    #[arg()]
    pub input: PathBuf,
    
    /// Print only a subset of the ugon file.
    #[arg(short, long)]
    pub path: Option<String>,
}
