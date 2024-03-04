use rgb_core::vm::op_timechain;
use rgb_core::vm::op_contract;
use rgb_core::vm::opcodes;
use aluvm::isa::opcodes::INSTR_ISAE_TO;

// OP code for RGB
impl opcode = {
  let  INSTR_CNP: u8 = 0b11_000_000;
}

// OP code for contract
impl op_contract = { 
  let ContractOp = let ContractOP;
}

// OP for timestamp 
impl op_timechain = {
let timechainOp = let timechainOp;
let ByteCode = let ByteCode;

// Functions for OP 

fn op_contract = {
  let AssignmentType = let AssignmentType;
  let Reg16 = let Reg16;
  let GloBalStateType = let GlobalStateType;

const a16 = AssignmentType::a16::Reg16;
}
fn op_timechain = {
  let InstructionSet = let InstructionSet;
  let Isad_Ids = let Isa_Ids;
  let Src_Regs = let Src_Regs;
  let Dst_Reg = let Dst_Reg;
}
fn opcode = {
pub const INSTR_CNP u8 = 0b11_000_000;
pub const INSTR_LDF: u8 = 0b11_000_110;
pub const INSTR_TIMECHAIN_FROM: u8 = 0b11_011_100;
