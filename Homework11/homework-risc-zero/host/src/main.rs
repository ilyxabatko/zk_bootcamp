// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{SECRET_NUM_CHECK_ELF, SECRET_NUM_CHECK_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    // First, we construct an executor environment
    let env = ExecutorEnv::builder().build().unwrap();

    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example:
    // let env = ExecutorEnv::builder().add_input(&vec).build().unwrap();
    let guest_input_number: u64 = 2233;
    let env = ExecutorEnv::builder().add_input(&[guest_input_number]).build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, SECRET_NUM_CHECK_ELF).unwrap();

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    println!("I know the secret number and I can prove it!");

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(SECRET_NUM_CHECK_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

}
