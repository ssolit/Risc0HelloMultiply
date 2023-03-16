// TODO: Update the name of the method loaded by the prover. E.g., if the method is `multiply`, replace `METHOD_NAME_ID` with `MULTIPLY_ID` and replace `METHOD_NAME_PATH` with `MULTIPLY_PATH`
use methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::Prover;
// use risc0_zkvm::serde::{from_slice, to_vec};

fn main() {
    // Make the prover.
    let mut prover = Prover::new(MULTIPLY_ELF, MULTIPLY_ID)
        .expect("Prover should be constructed from valid method source code and corresponding image ID");

    // TODO: Implement communication with the guest here

    // Run prover & generate receipt
    let receipt = prover.run()
        .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");

    // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    receipt.verify(MULTIPLY_ID)
        .expect("Code you have proven should successfully verify; did you specify the correct image ID?");

    // TODO: Implement code for transmitting or serializing the receipt for other parties to verify here
}