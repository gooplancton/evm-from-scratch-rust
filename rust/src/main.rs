use colored::Colorize;
use evm::evm;
use evm::state;
use evm::EvmLog;
use primitive_types::U256;
use serde::Deserialize;
use std::borrow::Borrow;
/**
 * EVM From Scratch
 * Rust template
 *
 * To work on EVM From Scratch in Rust:
 *
 * - Install Rust: https://www.rust-lang.org/tools/install
 * - Edit `rust/lib.rs`
 * - Run `cd rust && cargo run` to run the tests
 *
 * Hint: most people who were trying to learn Rust and EVM at the same
 * gave up and switched to JavaScript, Python, or Go. If you are new
 * to Rust, implement EVM in another programming language first.
 */
use std::mem;

#[derive(Debug, Deserialize)]
struct Evmtest {
    name: String,
    hint: String,
    code: Code,
    expect: Expect,
    tx: Option<state::TxData>,
    block: Option<state::BlockData>,
    state: Option<state::ContractsStateData>,
}

#[derive(Debug, Deserialize)]
struct Code {
    asm: String,
    bin: String,
}

#[derive(Debug, Deserialize)]
struct Expect {
    stack: Option<Vec<String>>,
    logs: Option<Vec<EvmLog>>,
    success: bool,
    #[serde(rename = "return")]
    ret: Option<String>,
}

fn main() {
    let text = std::fs::read_to_string("../evm.json").unwrap();
    let data: Vec<Evmtest> = serde_json::from_str(&text).unwrap();

    let total = data.len();

    for (index, mut test) in data.into_iter().enumerate() {
        println!();
        println!("======================================");
        println!();
        println!("Test {} of {}: {}", index + 1, total, test.name);

        let code: Vec<u8> = hex::decode(&test.code.bin).unwrap();
        let mut chain_state = state::BlockchainState {
            tx: mem::take(&mut test.tx).unwrap_or_default(),
            block: mem::take(&mut test.block).unwrap_or_default(),
            contracts_state: mem::take(&mut test.state).unwrap_or_default(),
        };

        let result = evm(&code, &mut chain_state);

        let mut expected_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
        }

        let mut matching = result.stack.len() == expected_stack.len();
        if matching {
            for i in 0..result.stack.len() {
                if result.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }

        let logs_ok =
            test.expect.logs.is_none() || test.expect.logs.as_ref().unwrap() == &result.logs;

        let ret_ok = test.expect.ret == result.ret;

        matching = ret_ok && logs_ok && matching && result.success == test.expect.success;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm.purple());

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected return: {:?}", test.expect.ret);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            if let Some(ref expected_logs) = test.expect.logs {
                println!("Expected logs: [");
                for log in expected_logs {
                    println!("  {:#?},", log);
                }
                println!("]\n");
            }

            println!("Actual success: {:?}", result.success);
            println!("Actual return: {:?}", result.ret);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("Actual logs: [");
            for log in result.logs {
                println!("  {:#?},", log);
            }
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("{}", "PASS".green());
    }
    println!("Congratulations!");
}
