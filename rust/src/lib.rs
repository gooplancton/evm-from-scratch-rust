mod helpers;
mod opcodes;
mod operations;
pub mod state;

use primitive_types::U256;
use state::BlockchainState;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(code: &Vec<u8>, chain_state: &BlockchainState) -> EvmResult {
    let mut memory: Vec<u8> = Vec::new();
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
            }
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
            opcodes::KECCAK256 => operations::keccak256(&mut stack, &mut memory),
            opcodes::BYTE => operations::byte(&mut stack),
            opcodes::SHL => operations::shl(&mut stack),
            opcodes::SHR => operations::shr(&mut stack),
            opcodes::SAR => operations::sar(&mut stack),
            opcodes::DUP1..=opcodes::DUP16 => {
                let offset = opcode - opcodes::DUP1 + 1;
                operations::duplicate(offset as usize, &mut stack);
            }
            opcodes::SWAP1..=opcodes::SWAP16 => {
                let offset = opcode - opcodes::SWAP1 + 1;
                operations::swap(offset as usize, &mut stack);
            }
            opcodes::PC => stack.push((pc - 1).into()),
            opcodes::GAS => stack.push(U256::MAX),
            opcodes::JUMP => {
                let res = operations::jump(&mut stack, code, &mut pc);
                if let Err(_) = res {
                    success = false;
                    break;
                }
            }
            opcodes::JUMPI => {
                let res = operations::jump_if(&mut stack, code, &mut pc);
                if let Err(_) = res {
                    success = false;
                    break;
                }
            }
            opcodes::JUMPDEST => continue,
            opcodes::MSIZE => operations::memsize(&mut stack, &memory),
            opcodes::MSTORE => operations::memstore(&mut stack, &mut memory),
            opcodes::MSTORE8 => operations::memstore8(&mut stack, &mut memory),
            opcodes::MLOAD => operations::memload(&mut stack, &mut memory),
            opcodes::ADDRESS => {
                let address = chain_state.tx.clone().unwrap().to.unwrap();
                stack.push(address.parse().unwrap());
            }
            opcodes::CALLER => {
                let caller = chain_state.tx.clone().unwrap().from.unwrap();
                stack.push(caller.parse().unwrap());
            }
            opcodes::ORIGIN => {
                let origin = chain_state.tx.clone().unwrap().origin.unwrap();
                stack.push(origin.parse().unwrap());
            }
            opcodes::GASPRICE => {
                let gasprice = chain_state.tx.clone().unwrap().gasprice.unwrap();
                stack.push(gasprice.parse().unwrap());
            }
            opcodes::BLOCKHASH => continue, // NOTE: not implemented
            opcodes::BASEFEE => {
                let basefee = chain_state.block.clone().unwrap().basefee.unwrap();
                stack.push(basefee.parse().unwrap());
            }
            opcodes::COINBASE => {
                let coinbase = chain_state.block.clone().unwrap().coinbase.unwrap();
                stack.push(coinbase.parse().unwrap());
            }
            opcodes::TIMESTAMP => {
                let timestamp = chain_state.block.clone().unwrap().timestamp.unwrap();
                stack.push(timestamp.parse().unwrap());
            }
            opcodes::NUMBER => {
                let number = chain_state.block.clone().unwrap().number.unwrap();
                stack.push(number.parse().unwrap());
            }
            opcodes::DIFFICULTY => {
                let difficulty = chain_state.block.clone().unwrap().difficulty.unwrap();
                stack.push(difficulty.parse().unwrap());
            }
            opcodes::GASLIMIT => {
                let gaslimit = chain_state.block.clone().unwrap().gaslimit.unwrap();
                stack.push(gaslimit.parse().unwrap());
            }
            opcodes::CHAINID => {
                let chainid = chain_state.block.clone().unwrap().chainid.unwrap();
                stack.push(chainid.parse().unwrap());
            },
            _ => {
                success = false;
                break;
            }
        }
    }

    stack.reverse();

    EvmResult { stack, success }
}
