mod peek;
mod opcode;
mod operand;
mod instruction;

pub(self) use peek::Peek;
pub(self) use opcode::peek_opcode;
pub(self) use operand::operand;
pub(in crate::dasm) use instruction::disasm_one;

pub use opcode::OPCode;
pub use instruction::{Instruction, Instructions};



mod disassembler;
mod instruction;
mod reader;

pub(crate) use reader::ByteCodeReader;

pub use instruction::{Instruction, Instructions, OPCode};
pub use disassembler::Disassembler;
