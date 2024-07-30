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

pub fn evm(code: &Vec<u8>, chain_state: &mut BlockchainState) -> EvmResult {
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
            opcodes::MSIZE => operations::memsize(&mut stack, &memory),
            opcodes::MSTORE => operations::memstore(&mut stack, &mut memory),
            opcodes::MSTORE8 => operations::memstore8(&mut stack, &mut memory),
            opcodes::MLOAD => operations::memload(&mut stack, &mut memory),
            opcodes::ADDRESS => stack.push(chain_state.tx.to.unwrap().into()),
            opcodes::CALLER => stack.push(chain_state.tx.from.unwrap().into()),
            opcodes::ORIGIN => stack.push(chain_state.tx.origin.unwrap().into()),
            opcodes::GASPRICE => stack.push(chain_state.tx.gasprice.unwrap().into()),
            opcodes::BASEFEE => stack.push(chain_state.block.basefee.unwrap().into()),
            opcodes::COINBASE => stack.push(chain_state.block.coinbase.unwrap().into()),
            opcodes::TIMESTAMP => stack.push(chain_state.block.timestamp.unwrap().into()),
            opcodes::NUMBER => stack.push(chain_state.block.number.unwrap().into()),
            opcodes::DIFFICULTY => stack.push(chain_state.block.difficulty.unwrap().into()),
            opcodes::GASLIMIT => stack.push(chain_state.block.gaslimit.unwrap().into()),
            opcodes::CHAINID => stack.push(chain_state.block.chainid.unwrap().into()),
            opcodes::CALLVALUE => operations::call_value(&mut stack, chain_state),
            opcodes::CALLDATALOAD => operations::call_data_load(&mut stack, chain_state),
            opcodes::CALLDATASIZE => operations::call_data_size(&mut stack, chain_state),
            opcodes::CALLDATACOPY => {
                operations::call_data_copy(&mut stack, &mut memory, chain_state)
            },
            opcodes::CODESIZE => stack.push(code.len().into()),
            opcodes::CODECOPY => operations::code_copy(&mut stack, &mut memory, code),
            opcodes::EXTCODESIZE => operations::external_code_size(&mut stack, chain_state),
            opcodes::EXTCODECOPY => operations::external_code_copy(&mut stack, &mut memory, chain_state),
            opcodes::EXTCODEHASH => operations::external_code_hash(&mut stack, chain_state),
            opcodes::BALANCE => operations::get_balance(&mut stack, chain_state),
            opcodes::SELFBALANCE => operations::self_balance(&mut stack, chain_state),
            opcodes::JUMPDEST => continue,
            opcodes::BLOCKHASH => continue, // NOTE: not implemented
            _ => {
                success = false;
                break;
            }
        }
    }

    stack.reverse();

    EvmResult { stack, success }
}
