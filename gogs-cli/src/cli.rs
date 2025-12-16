use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gog")]
#[command(about = "Gogs CLI for multi-agent development orchestration")]
#[command(version)]
#[command(long_about = "A command-line tool for interacting with Gogs issue trackers, \
designed for coordinating multiple AI coding agents across repositories.")]
pub struct Cli {
    /// Profile to use (overrides default)
    #[arg(long, global = true)]
    pub profile: Option<String>,

    /// Output in JSON format
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize configuration
    Init,

    /// Issue operations
    #[command(subcommand)]
    Issue(IssueCommand),

    /// Repository operations
    #[command(subcommand)]
    Repo(RepoCommand),
}

#[derive(Subcommand)]
pub enum IssueCommand {
    /// List issues
    #[command(
        long_about = "List issues from repositories.\n\n\
        Examples:\n  \
        gog issue list --all\n  \
        gog issue list --repo owner/project\n  \
        gog issue list --all --label bug"
    )]
    List {
        /// List issues across all repositories
        #[arg(long)]
        all: bool,

        /// Only show open issues (default)
        #[arg(long, conflicts_with = "closed")]
        open: bool,

        /// Only show closed issues
        #[arg(long)]
        closed: bool,

        /// Specific repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,

        /// Filter by label (can be repeated)
        #[arg(long)]
        label: Vec<String>,
    },

    /// Show issue details
    #[command(
        long_about = "Show detailed information about an issue including comments.\n\n\
        Examples:\n  \
        gog issue show 42 --repo owner/project"
    )]
    Show {
        /// Issue number
        number: i64,

        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },

    /// Create a new issue
    #[command(
        long_about = "Create a new issue in a repository.\n\n\
        Examples:\n  \
        gog issue create \"Fix bug\" --repo owner/project\n  \
        gog issue create \"New feature\" --repo owner/project --body \"Details here\""
    )]
    Create {
        /// Issue title
        title: String,

        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,

        /// Issue body
        #[arg(long)]
        body: Option<String>,

        /// Add labels (can be repeated)
        #[arg(long)]
        label: Vec<String>,
    },

    /// Add comment to issue
    #[command(
        long_about = "Add a comment to an existing issue.\n\n\
        Examples:\n  \
        gog issue comment 42 \"Working on this\" --repo owner/project"
    )]
    Comment {
        /// Issue number
        number: i64,

        /// Comment text
        text: String,

        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },

    /// Close an issue
    Close {
        /// Issue number
        number: i64,

        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },

    /// Reopen an issue
    Reopen {
        /// Issue number
        number: i64,

        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },

    /// Add label to issue
    Label {
        /// Issue number
        number: i64,

        /// Label name
        label: String,

        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },

    /// Remove label from issue
    Unlabel {
        /// Issue number
        number: i64,

        /// Label name
        label: String,

        /// Repository (owner/repo)
        #[arg(long)]
        repo: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum RepoCommand {
    /// List repositories accessible to the current profile
    List,
}
