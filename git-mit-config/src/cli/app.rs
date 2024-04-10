use clap::Parser;
use clap_complete::Shell;
use mit_commit_message_lints::scope::Scope;

#[derive(Parser, Clone, Eq, PartialEq)]
#[clap(author, version, about)]
#[clap(bin_name = "git-mit-config")]
pub struct CliArgs {
    #[clap(long, value_enum, value_parser)]
    pub completion: Option<Shell>,
    #[clap(subcommand)]
    pub action: Option<Action>,
}

#[derive(clap::Subcommand, Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub enum Action {
    /// Manage active lints
    Lint {
        #[clap(subcommand)]
        action: Lint,
    },
    /// Manage mit configuration
    Mit {
        #[clap(subcommand)]
        action: Mit,
    },
    /// Manage relates-to settings
    RelatesTo {
        #[clap(subcommand)]
        action: RelatesTo,
    },
}

#[derive(clap::Subcommand, Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub enum Lint {
    /// Generate the config file for your current settings
    Generate {
        #[clap(long, value_enum, value_parser, default_value = "local")]
        scope: Scope,
    },
    /// List the available lints
    Available {
        #[clap(long, value_enum, value_parser, default_value = "local")]
        scope: Scope,
    },
    /// List the enabled lints
    Enabled {
        #[clap(long, value_enum, value_parser, default_value = "local")]
        scope: Scope,
    },
    /// Get status of a lint
    Status {
        #[clap(long, value_enum, value_parser, default_value = "local")]
        scope: Scope,
        #[clap()]
        lints: Vec<mit_lint::Lint>,
    },
    /// Enable a lint
    Enable {
        #[clap(long, value_enum, value_parser, default_value = "local")]
        scope: Scope,
        /// The lint to enable
        #[clap()]
        lints: Vec<mit_lint::Lint>,
    },
    /// Disable a lint
    Disable {
        #[clap(long, value_enum, value_parser, default_value = "local")]
        scope: Scope,
        /// The lint to disable
        #[clap()]
        lints: Vec<mit_lint::Lint>,
    },
}

#[derive(clap::Subcommand, Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub enum Mit {
    /// Update or add an initial in the mit configuration
    Set {
        #[clap(long, value_enum, value_parser, default_value = "local")]
        scope: Scope,
        /// Initial of the mit to update or add
        #[clap()]
        initials: String,
        /// Name to use for the mit in format "Forename Surname"
        #[clap()]
        name: String,
        /// Email to use for the mit
        #[clap()]
        email: String,
        /// Signing key to use for this user
        #[clap()]
        signingkey: Option<String>,
    },
    /// Generate a file version of available authors
    Generate {
        /// Path to a file where mit initials, emails and names can be found
        #[clap(
            short,
            long,
            env = "GIT_MIT_AUTHORS_CONFIG",
            default_value = "$HOME/.config/git-mit/mit.toml"
        )]
        config: String,
        /// Execute a command to generate the mit configuration, stdout will be
        /// captured and used instead of the file, if both this and the file is
        /// present, this takes precedence
        #[clap(short, long, env = "GIT_MIT_AUTHORS_EXEC")]
        exec: Option<String>,
    },
    /// List available authors
    Available {
        /// Path to a file where mit initials, emails and names can be found
        #[clap(
            short,
            long,
            env = "GIT_MIT_AUTHORS_CONFIG",
            default_value = "$HOME/.config/git-mit/mit.toml"
        )]
        config: String,
        /// Execute a command to generate the mit configuration, stdout will be
        /// captured and used instead of the file, if both this and the file is
        /// present, this takes precedence
        #[clap(short, long, env = "GIT_MIT_AUTHORS_EXEC")]
        exec: Option<String>,
    },
    /// Print example mit toml file
    Example,
}

#[derive(clap::Subcommand, Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub enum RelatesTo {
    /// Use a template for the relates-to trailer
    Template {
        #[clap(long, short, value_enum, value_parser, default_value = "local")]
        scope: Scope,
        /// A `TinyTemplate` template with a single value variable that will be
        /// applied to the relates-to trailer
        #[clap(
            index = 1,
            env = "GIT_MIT_RELATES_TO_TEMPLATE",
            default_value = "{ value }"
        )]
        template: String,
    },
}
