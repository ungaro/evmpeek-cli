//! The `evmpeek` cli
mod cmd;
mod dasm;

use crate::cmd::NodeArgs;

use clap::{CommandFactory, Parser, Subcommand};

pub const VERSION_MESSAGE: &str = "v0.0.1";


/// A fast local evm disassambler.
#[derive(Debug, Parser)]
#[clap(name = "evmpeek", version = crate::VERSION_MESSAGE, next_display_order = None)]



/// `evmpeek 0.0.1`

pub struct App {
    #[clap(flatten)]
    pub node: NodeArgs,

    #[clap(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Clone, Debug, Subcommand, Eq, PartialEq)]
pub enum Commands {
    #[clap(visible_alias = "com", about = "Generate shell completions script.")]
    Completions {
        #[clap(value_enum)]
        shell: clap_complete::Shell,
    },
    #[clap(visible_alias = "fig", about = "Generate Fig autocompletion spec.")]
    GenerateFigSpec,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::parse();
    //app.node.evm_opts.resolve_rpc_alias();

    if let Some(ref cmd) = app.cmd {
        match cmd {
            Commands::Completions { shell } => {
                clap_complete::generate(
                    *shell,
                    &mut App::command(),
                    "evmpeek_cli",
                    &mut std::io::stdout(),
                );
            }
            Commands::GenerateFigSpec => clap_complete::generate(
                clap_complete_fig::Fig,
                &mut App::command(),
                "evmpeek_cli",
                &mut std::io::stdout(),
            ),
        }
        return Ok(())
    }

    let _ = fdlimit::raise_fd_limit();
    app.node.run().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_help() {
        let _: App = App::parse_from(["evmpeek", "--help"]);
    }

    #[test]
    fn can_parse_completions() {
        let args: App = App::parse_from(["evmpeek", "completions", "bash"]);
        assert_eq!(args.cmd, Some(Commands::Completions { shell: clap_complete::Shell::Bash }));
    }
}