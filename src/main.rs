extern crate sha2;
extern crate rand;

use sha2::{Sha256, Digest};
use rand::Rng;

// Commitment function: hashes the value with a random blinding factor.
fn commit(value: u64, r: u64) -> Vec<u8> {
    let input = format!("{}{}", value, r);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().to_vec()
}

// Prover function: Generates a commitment for `a`, `b` and sends a challenge response.
fn prover(a: u64, b: u64) -> (Vec<u8>, Vec<u8>, u64) {
    // Random blinding factor
    let r = rand::thread_rng().gen::<u64>();

    // Commitments for a and b
    let commit_a = commit(a, r);
    let commit_b = commit(b, r);

    // For simplicity, we reveal `a` along with the blinding factor
    (commit_a, commit_b, r)
}

// Verifier function: Checks if the sum `a + b = s` is correct and commitments are valid.
fn verifier(commit_a: Vec<u8>, commit_b: Vec<u8>, a: u64, b: u64, r: u64, s: u64) -> bool {
    // Recommit to a and b using the same blinding factor
    let commit_a_check = commit(a, r);
    let commit_b_check = commit(b, r);

    // Verify that the commitments are correct and the sum matches
    commit_a == commit_a_check && commit_b == commit_b_check && a + b == s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment() {
        let value = 42;
        let r = 12345;

        // Create the commitment for value with blinding factor r
        let commit1 = commit(value, r);
        let commit2 = commit(value, r);

        // Ensure that the commitment is consistent
        assert_eq!(commit1, commit2, "Commitments for the same value and blinding factor should be the same");
    }

    #[test]
    fn test_prover_and_verifier() {
        let a: u64 = 10;
        let b: u64 = 20;
        let sum = a + b;

        // Prover generates the commitment and response
        let (commit_a, commit_b, r) = prover(a, b);

        // Verifier checks the commitments and the sum
        let is_valid = verifier(commit_a, commit_b, a, b, r, sum);

        // Ensure the verification passes
        assert!(is_valid, "The ZKP verification failed: the commitments or sum are incorrect");
    }

    #[test]
    fn test_invalid_sum() {
        let a: u64 = 10;
        let b: u64 = 20;
        let sum = a + b;

        // Prover generates the commitment and response
        let (commit_a, commit_b, r) = prover(a, b);

        // Verifier checks the commitments and an invalid sum
        let invalid_sum = sum + 1;
        let is_valid = verifier(commit_a, commit_b, a, b, r, invalid_sum);

        // Ensure the verification fails with an invalid sum
        assert!(!is_valid, "The ZKP verification should fail for an invalid sum");
    }

    #[test]
    fn test_invalid_commitment() {
        let a: u64 = 10;
        let b: u64 = 20;
        let sum = a + b;

        // Prover generates the commitment and response
        let (commit_a, commit_b, r) = prover(a, b);

        // Modify the commitment for `a` to simulate an invalid commitment
        let invalid_commit_a = vec![0u8; commit_a.len()]; // Invalid commitment

        // Verifier checks the invalid commitment
        let is_valid = verifier(invalid_commit_a, commit_b, a, b, r, sum);

        // Ensure the verification fails with an invalid commitment
        assert!(!is_valid, "The ZKP verification should fail for an invalid commitment");
    }
}

fn main() {
    // Example numbers a, b, and the sum s
    let a: u64 = 5;
    let b: u64 = 7;
    let sum = a + b;

    // Step 1: Prover generates the commitments and prepares the response
    let (commit_a, commit_b, r) = prover(a, b);

    // Step 2: Verifier checks the commitments and sum
    let is_valid = verifier(commit_a, commit_b, a, b, r, sum);

    if is_valid {
        println!("ZKP Verified: The sum is correct, and the commitment is valid.");
    } else {
        println!("ZKP Failed: The commitment or sum is invalid.");
    }
}
