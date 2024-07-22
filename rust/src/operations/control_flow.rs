use primitive_types::U256;

use crate::opcodes;

pub struct InvalidJumpError;

pub fn jump(stack: &mut Vec<U256>, code: &Vec<u8>, pc: &mut usize) -> Result<(), InvalidJumpError> {
    let offset = stack.pop().unwrap().as_usize();
    if code[offset] != opcodes::JUMPDEST {
        return Err(InvalidJumpError);
    }

    for i in 1..=32 {
        let opcode = code[offset - i];
        if opcode <= opcodes::PUSH0 || opcode > opcodes::PUSH32 {
            continue;
        }

        let n_bytes = (opcode - opcodes::PUSH0) as usize;
        if n_bytes >= i {
            return Err(InvalidJumpError);
        }

        break;
    }

    *pc = offset;
    Ok(())
}

pub fn jump_if(
    stack: &mut Vec<U256>,
    code: &Vec<u8>,
    pc: &mut usize,
) -> Result<(), InvalidJumpError> {
    let offset = stack.pop().unwrap().as_usize();
    let condition = stack.pop().unwrap();
    if code[offset] != opcodes::JUMPDEST {
        return Err(InvalidJumpError);
    }

    for i in 1..=32 {
        let opcode = code[offset - i];
        if opcode <= opcodes::PUSH0 || opcode > opcodes::PUSH32 {
            continue;
        }

        let n_bytes = (opcode - opcodes::PUSH0) as usize;
        if n_bytes >= i {
            return Err(InvalidJumpError);
        }

        break;
    }

    if condition != U256::zero() {
        *pc = offset;
    }

    Ok(())
}
