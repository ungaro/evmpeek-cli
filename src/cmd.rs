
use clap::Parser;

use futures::FutureExt;

use tokio::time::{Instant, Interval};
use tracing::{error, trace};
use crate::dasm::{ByteCodeReader, Instructions};
use crate::dasm::{Disassembler, Instruction, OPCode};

use std::fmt::Write;


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
        


        let bytes = hex::decode(self.bytecode.unwrap_or("No bytecode available".to_string()))?;

        let dasm = Disassembler::new(&bytes);
        let instructions = dasm.disassemble();
    
        for ins in instructions {
            let mut buffer = String::new();
            write!(&mut buffer, "{:>08X}| {:X?}", ins.offset, ins.opcode)?;
            if let Some(operand) = ins.operand {
                write!(&mut  buffer, " 0x{}", hex::encode(operand))?;
            }
            println!("{}", buffer);
        }
    


        Ok(())
    }
}
