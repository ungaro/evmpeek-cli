use clap::Parser;

use futures::FutureExt;

use tokio::time::{Instant, Interval};
use tracing::{error, trace};

#[derive(Clone, Debug, Parser)]
pub struct NodeArgs {


    #[clap(
        long,
        short,
        help = "bytecode to parse",
        value_name = "BYTECODE"
    )]
    pub bytecode:  Option<String>,



    #[clap(
        long,
        short,
        help = "The contract address to disassemble",
        value_name = "ADDRESS"
    )]
    pub address: Option<String>,


    #[clap(
        long,
        short,
        help = "EVM compatible chain",
        value_name = "network",
    )]
    pub network: Option<String>,





    #[clap(
        long,
        short,
        help = "Writes output of `evmpeek` as json to user-specified file",
        value_name = "OUTPUT"
    )]
    pub output: Option<String>,

}



impl NodeArgs {


    /// Starts the node
    ///
    /// See also [crate::spawn()]
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        println!("hello world");
        println!("{}", self.bytecode.unwrap_or("No bytecode available".to_string()));
        

        Ok(())
    }
}
