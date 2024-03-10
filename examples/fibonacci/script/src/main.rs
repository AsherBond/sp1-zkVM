use sp1_core::{utils, SP1Prover, SP1Stdin, SP1Verifier};
use std::time::Instant;

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup a tracer for logging.
    utils::setup_logger();

    // Create an input stream.
    let stdin = SP1Stdin::new();

    // Generate the proof for the given program.
    let start = Instant::now();
    SP1Prover::execute(ELF, stdin).expect("execution failed");
    let duration = start.elapsed().as_secs();
    println!("execution time = {}", duration);
    // let proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // // Verify proof.
    // SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // // Save the proof.
    // proof
    //     .save("proof-with-pis.json")
    //     .expect("saving proof failed");

    // println!("succesfully generated and verified proof for the program!")
}
