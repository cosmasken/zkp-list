use rand::Rng; // Random number generator
use sha2::{Sha256, Digest}; // For SHA-256 hashing
use std::fmt::Write; // For converting hash bytes to hex

// Simulated user structure
struct User {
    username: String, // The username of the user
    password: String, // The user's password
}

// Simulated server structure
struct Server {
    salt: String, // A salt value for additional randomness (unused in current logic)
}

impl Server {
    // Generate a random challenge for the user
    fn generate_challenge(&self) -> String {
        // Generate a random 16-character alphanumeric string
        rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }

    // Authenticate the user by verifying their response to the challenge
    fn authenticate(&self, user: &User, response: &str, challenge: &str) -> bool {
        // Compute the expected response using the user's password and the challenge
        let expected_response = hash(&user.password, challenge);

        // Compare the expected response with the provided response
        response == expected_response
    }
}

// A simple hashing function using SHA-256
fn hash(input: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input); // Add the input (password) to the hash
    hasher.update(salt);  // Add the salt or challenge to the hash
    let result = hasher.finalize();

    // Convert the hash bytes into a hexadecimal string
    let mut hex_output = String::new();
    for byte in result {
        write!(&mut hex_output, "{:02x}", byte).unwrap();
    }
    hex_output
}

fn main() {
    // Create a simulated user
    let user = User {
        username: "Lauren".to_string(),
        password: "placeholder_password".to_string(), // Never use real passwords in examples
    };

    // Create a simulated server
    let server = Server {
        salt: rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(16)
            .map(char::from)
            .collect(),
    };

    // Generate a challenge from the server
    let challenge = server.generate_challenge();

    // User computes a response to the challenge
    let response = hash(&user.password, &challenge);

    // Server verifies the response
    if server.authenticate(&user, &response, &challenge) {
        println!("User {} authenticated successfully.", user.username);
    } else {
        println!("User {} authentication failed.", user.username);
    }
}

// Unit tests for the authentication logic
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_function() {
        let input = "Password123";
        let salt = "random_salt";
        let expected_output = hash(input, salt);
        assert_eq!(expected_output, hash(input, salt)); // Hash should be deterministic
    }

    #[test]
    fn test_authenticate_success() {
        let user = User {
            username: "TestUser".to_string(),
            password: "SecurePass".to_string(),
        };
        let server = Server {
            salt: "random_salt".to_string(),
        };

        let challenge = server.generate_challenge();
        let response = hash(&user.password, &challenge);

        assert!(server.authenticate(&user, &response, &challenge));
    }

    #[test]
    fn test_authenticate_failure_wrong_password() {
        let user = User {
            username: "TestUser".to_string(),
            password: "SecurePass".to_string(),
        };
        let server = Server {
            salt: "random_salt".to_string(),
        };

        let challenge = server.generate_challenge();
        let response = hash("WrongPass", &challenge); // Simulate incorrect password

        assert!(!server.authenticate(&user, &response, &challenge));
    }

    #[test]
    fn test_authenticate_failure_tampered_challenge() {
        let user = User {
            username: "TestUser".to_string(),
            password: "SecurePass".to_string(),
        };
        let server = Server {
            salt: "random_salt".to_string(),
        };

        let challenge = server.generate_challenge();
        let tampered_challenge = "tampered_challenge";
        let response = hash(&user.password, tampered_challenge); // Simulate tampered challenge

        assert!(!server.authenticate(&user, &response, &challenge));
    }
}
