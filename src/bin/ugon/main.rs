use std::path::PathBuf;

use clap::*;

/// Tool to create, convert and process UGON files in various ways.
/// 
/// See: https://github.com/Longor1996/ugon
#[derive(Debug, Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
pub struct MainArgs {
    /// Disable logging to stderr?
    #[arg(long, short = 'q', global = true)]
    quiet: bool,
    
    /// Enable or disable features.
    #[arg(long, short = 'F', global = true, default_value = "*")]
    feats: String,
    
    /// What should the tool do?
    #[command(subcommand)]
    command: MainMode,
}

#[derive(Debug, Subcommand)]
pub enum MainMode {
    /// Create a ugon file from a given file.
    #[command(name = "make", short_flag = 'm')]
    Make(modes::make::MakeMode),
    
    /// Convert the given ugon file to another file.
    #[command(name = "into", short_flag = 'i')]
    Into(modes::into::IntoMode),
    
    /// Print to stdout as human readable tree.
    #[command(name = "tree", short_flag = 't')]
    Tree(modes::tree::TreeMode),
    
    /// Print to stdout as grep-able lines.
    #[command(name = "grep", short_flag = 'g')]
    Grep(modes::grep::GrepMode),
    
    /// Make changes to the given ugon file.
    /// 
    /// Changes can either be written back or saved as a patch file.
    #[command(name = "edit", short_flag = 'e')]
    Edit(modes::edit::EditMode),
    
    /// Opens the ugon specification in a web-browser.
    Spec
}

pub mod io {
    pub mod input;
    pub mod output;
}

pub mod path;

pub mod modes {
    use super::*;
    pub mod make;
    pub mod into;
    pub mod tree;
    pub mod grep;
    pub mod edit;
}

fn main() {
    let cli: MainArgs = MainArgs::parse();
    
    match cli.command {
        MainMode::Make(opts) => todo!("'make' mode"),
        MainMode::Into(opts) => todo!("'into' mode"),
        MainMode::Tree(opts) => todo!("'tree' mode"),
        MainMode::Grep(opts) => todo!("'grep' mode"),
        MainMode::Edit(opts) => todo!("'edit' mode"),
        MainMode::Spec => todo!("'spec' action"),
    }
}
