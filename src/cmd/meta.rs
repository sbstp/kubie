use clap::Parser;

use crate::settings::ContextHeaderBehavior;

#[derive(Debug, Parser)]
#[clap(version)]
pub enum Kubie {
    /// Spawn a shell in the given context. The shell is isolated from other shells.
    /// Kubie shells can be spawned recursively without any issue.
    #[clap(name = "ctx")]
    Context {
        /// Specify in which namespace of the context the shell is spawned.
        #[clap(short = 'n', long = "namespace")]
        namespace_name: Option<String>,

        /// Specify files from which to load contexts instead of using the installed ones.
        #[clap(short = 'f', long = "kubeconfig")]
        kubeconfigs: Vec<String>,

        /// Enter the context by spawning a new recursive shell.
        #[clap(short = 'r', long = "recursive")]
        recursive: bool,

        /// Name of the context to enter. Use '-' to switch back to the previous context.
        context_name: Option<String>,
    },

    /// Change the namespace in which the current shell operates. The namespace change does
    /// not affect other shells.
    #[clap(name = "ns")]
    Namespace {
        /// Enter the namespace by spawning a new recursive shell.
        #[clap(short = 'r', long = "recursive")]
        recursive: bool,

        /// Unsets the namespace in the currently active context.
        #[clap(short = 'u', long = "unset")]
        unset: bool,

        /// Name of the namespace to enter. Use '-' to switch back to the previous namespace.
        namespace_name: Option<String>,
    },

    /// View info about the current kubie shell, such as the context name and the
    /// current namespace.
    #[clap(name = "info")]
    Info(KubieInfo),

    /// Execute a command inside of the given context and namespace.
    #[clap(name = "exec", trailing_var_arg = true)]
    Exec {
        /// Name of the context in which to run the command.
        context_name: String,
        /// Namespace in which to run the command. This is mandatory to avoid potential errors.
        namespace_name: String,
        /// Exit early if a command fails when using a wildcard context.
        #[clap(short = 'e', long = "exit-early")]
        exit_early: bool,
        /// Overrides behavior.print_context_in_exec in Kubie settings file.
        #[clap(value_enum, long = "context-headers")]
        context_headers_flag: Option<ContextHeaderBehavior>,
        /// Command to run as well as its arguments.
        args: Vec<String>,
    },

    /// Check the Kubernetes config files for issues.
    #[clap(name = "lint")]
    Lint,

    /// Edit the given context.
    #[clap(name = "edit")]
    Edit {
        /// Name of the context to edit.
        context_name: Option<String>,
    },

    /// Edit kubie's config file.
    #[clap(name = "edit-config")]
    EditConfig,

    /// Check for a Kubie update and replace Kubie's binary if needed.
    /// This function can ask for sudo-mode.
    #[clap(name = "update")]
    Update,

    /// Delete a context. Automatic garbage collection will be performed.
    /// Dangling users and clusters will be removed.
    #[clap(name = "delete")]
    Delete {
        /// Name of the context to edit.
        context_name: Option<String>,
    },
}

#[derive(Debug, Parser)]
pub struct KubieInfo {
    #[clap(subcommand)]
    pub kind: KubieInfoKind,
}

/// Type of info the user is requesting.
#[derive(Debug, Parser)]
pub enum KubieInfoKind {
    /// Get the current shell's context name.
    #[clap(name = "ctx")]
    Context,
    /// Get the current shell's namespace name.
    #[clap(name = "ns")]
    Namespace,
    /// Get the current depth of contexts.
    #[clap(name = "depth")]
    Depth,
}
