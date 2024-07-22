mod opcodes;
mod operations;
mod helpers;

use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

const RAM_CAPACITY: usize = 1064;

pub fn evm(code: &Vec<u8>) -> EvmResult {
    let _memory: [U256; RAM_CAPACITY] = [U256::zero(); RAM_CAPACITY];
    let mut stack: Vec<U256> = Vec::new();
    let mut pc = 0;
    let mut success = true;
    let code_length = code.len();

    while pc < code_length {
        let opcode = code[pc];
        pc += 1;

        match opcode {
            opcodes::STOP => break,
            opcodes::PUSH0..=opcodes::PUSH32 => {
                let n_bytes = opcode - opcodes::PUSH0;
                operations::push(pc, n_bytes as usize, code, &mut stack);
                pc += n_bytes as usize;
            },
            opcodes::POP => _ = stack.pop(),
            opcodes::ADD => operations::add(&mut stack),
            opcodes::MUL => operations::mul(&mut stack),
            opcodes::SUB => operations::sub(&mut stack),
            opcodes::DIV => operations::div(&mut stack),
            opcodes::SDIV => operations::signed_div(&mut stack),
            opcodes::MOD => operations::modulo(&mut stack),
            opcodes::SMOD => operations::signed_modulo(&mut stack),
            opcodes::ADDMOD => operations::addmod(&mut stack),
            opcodes::MULMOD => operations::mulmod(&mut stack),
            opcodes::EXP => operations::exp(&mut stack),
            opcodes::SIGNEXTEND => operations::signextend(&mut stack),
            opcodes::LT => operations::less_than(&mut stack),
            opcodes::GT => operations::greater_than(&mut stack),
            opcodes::SLT => operations::signed_less_than(&mut stack),
            opcodes::SGT => operations::signed_greater_than(&mut stack),
            opcodes::EQ => operations::equal(&mut stack),
            opcodes::ISZERO => operations::is_zero(&mut stack),
            opcodes::AND => operations::and(&mut stack),
            opcodes::OR => operations::or(&mut stack),
            opcodes::XOR => operations::xor(&mut stack),
            opcodes::NOT => operations::not(&mut stack),
            opcodes::BYTE => operations::byte(&mut stack),
            opcodes::SHL => operations::shl(&mut stack),
            opcodes::SHR => operations::shr(&mut stack),
            opcodes::SAR => operations::sar(&mut stack),
            opcodes::DUP1..=opcodes::DUP16 => {
                let offset = opcode - opcodes::DUP1 + 1;
                operations::duplicate(offset as usize, &mut stack);
            },
            opcodes::SWAP1..=opcodes::SWAP16 => {
                let offset = opcode - opcodes::SWAP1 + 1;
                operations::swap(offset as usize, &mut stack);
            },
            opcodes::PC => stack.push((pc - 1).into()),
            _ => {
                success = false;
                break;
            },
        }
    }

    stack.reverse();

    EvmResult { stack , success }
}

