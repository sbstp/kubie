use structopt::clap;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(global_setting(clap::AppSettings::VersionlessSubcommands))]
pub enum Kubie {
    /// Spawn a shell in the given context. The shell is isolated from other shells.
    /// Kubie shells can be spawned recursively without any issue.
    #[structopt(name = "ctx")]
    Context {
        /// Specify in which namespace of the context the shell is spawned.
        #[structopt(short = "n", long = "namespace")]
        namespace_name: Option<String>,

        /// Enter the context by spawning a new recursive shell.
        #[structopt(short = "r", long = "recursive")]
        recursive: bool,

        /// Name of the context to enter. Use '-' to switch back to the previous context.
        context_name: Option<String>,
    },

    /// Change the namespace in which the current shell operates. The namespace change does
    /// not affect other shells.
    #[structopt(name = "ns")]
    Namespace {
        /// Enter the namespace by spawning a new recursive shell.
        #[structopt(short = "r", long = "recursive")]
        recursive: bool,

        /// Name of the namespace to enter. Use '-' to switch back to the previous namespace.
        namespace_name: Option<String>,
    },

    /// View info about the current kubie shell, such as the context name and the
    /// current namespace.
    #[structopt(name = "info")]
    Info(KubieInfo),

    /// Execute a command inside of the given context and namespace.
    #[structopt(name = "exec", setting(clap::AppSettings::TrailingVarArg))]
    Exec {
        /// Name of the context in which to run the command.
        context_name: String,
        /// Namespace in which to run the command. This is mandatory to avoid potential errors.
        namespace_name: String,
        /// Exit early if a command fails when using a wildcard context.
        #[structopt(short = "e", long = "exit-early")]
        exit_early: bool,
        /// Command to run as well as its arguments.
        args: Vec<String>,
    },

    /// Check the Kubernetes config files for issues.
    #[structopt(name = "lint")]
    Lint,

    /// Edit the given context.
    #[structopt(name = "edit")]
    Edit {
        /// Name of the context to edit.
        context_name: Option<String>,
    },

    /// Edit kubie's config file.
    #[structopt(name = "edit-config")]
    EditConfig,

    /// Check for a Kubie update and replace Kubie's binary if needed.
    /// This function can ask for sudo-mode.
    #[structopt(name = "update")]
    Update,

    /// Delete a context. Automatic garbage collection will be performed.
    /// Dangling users and clusters will be removed.
    #[structopt(name = "delete")]
    Delete {
        /// Name of the context to edit.
        context_name: Option<String>,
    },
}

#[derive(Debug, StructOpt)]
pub struct KubieInfo {
    #[structopt(subcommand)]
    pub kind: KubieInfoKind,
}

/// Type of info the user is requesting.
#[derive(Debug, StructOpt)]
pub enum KubieInfoKind {
    /// Get the current shell's context name.
    #[structopt(name = "ctx")]
    Context,
    /// Get the current shell's namespace name.
    #[structopt(name = "ns")]
    Namespace,
    /// Get the current depth of contexts.
    #[structopt(name = "depth")]
    Depth,
}
