use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub enum Crmps {
    /// Create a new project according to a specified template
    New {
        #[structopt(parse(from_os_str))]
        /// The name of the project
        name: PathBuf,

        #[structopt(parse(from_os_str))]
        /// The template to use
        template: PathBuf
    },
    
    /// Add a new template to the crmps database
    Add {
        #[structopt(parse(from_os_str))]
        /// The template to add
        template: PathBuf
    },
    
    /// Remove a template from the crmps database
    Rem {
        #[structopt(parse(from_os_str))]
        /// The template to remove
        name: PathBuf
    },
    
    /// Add the specified tag to the project
    Tag {
        #[structopt(parse(from_os_str))]
        /// The name of the project to tag
        name: PathBuf,
        
        #[structopt(short = "w", long = "with")]
        /// The tags to use
        tags: String
    },
    
    /// Remove the specified tag from the project
    Untag {
        #[structopt(parse(from_os_str))]
        /// The name of the project to untag
        name: PathBuf,

        /// The tags to remove
        tags: String
    },
    
    /// Find a project that has the given tags or by name
    Locate {
        /// The tags or name to search for
        search_term: String,

        #[structopt(parse(from_os_str))]
        /// The directory to search in
        search_dir: Option<PathBuf>,

        #[structopt(short = "d")]
        /// Specify to use the default directory
        use_default: bool
    },
    
    /// List the contents of a directory till the specified limit
    Ls {
        #[structopt(parse(from_os_str))]
        /// The directory to list out
        directory: PathBuf,

        /// The maximum recursion limit
        till: i32
    },
    
    /// Set the default directory
    Default {
        #[structopt(parse(from_os_str))]
        /// The directory to set as default for use with the -d flag
        directory: PathBuf
    },
    
    /// Register a subcommand or extension
    Register {
        /// The extension to register
        extension:Option<String>,

        /// The command to call when receiving an extension
        command: Option<String>,
        
        /// The subcommand name
        subcommand: Option<String>,
        
        /// The source of the subcommand
        file: Option<PathBuf>
    },
    
    #[structopt(external_subcommand)]
    /// External commands created by the user
    External(Vec<String>)
}
