#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here
    let secret_num_input: u64 = env::read();
    let solution: u64 = 2234;

    assert_eq!(secret_num_input.checked_add(1).unwrap(), solution, "You don't know the secret number!");
}
