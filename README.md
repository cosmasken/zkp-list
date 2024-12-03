# Zero-Knowledge Proof Example

This project demonstrates a simple implementation of a Zero-Knowledge Proof (ZKP) concept using Rust. It simulates a user and a server exchanging data for authentication without directly sharing sensitive information.

## Features
- Hashing using SHA256
- Random challenge generation
- Authentication simulation with deterministic checks

---

## Prerequisites

- **Rust**: Ensure Rust is installed on your system. If not, follow the instructions below.

### Install Rust

1. Visit the [official Rust installation page](https://www.rust-lang.org/tools/install).
2. Follow the installation steps for your platform:
   - **Linux/macOS**:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - **Windows**: Download and run the `rustup-init.exe` installer from the website.

3. After installation, verify Rust is installed:
   ```bash
   rustc --version


markdown
Copy code
# Zero-Knowledge Proof Example

This project demonstrates a simple implementation of a Zero-Knowledge Proof (ZKP) concept using Rust. It simulates a user and a server exchanging data for authentication without directly sharing sensitive information.

## Features
- Hashing using SHA256
- Random challenge generation
- Authentication simulation with deterministic checks

---

## Prerequisites

- **Rust**: Ensure Rust is installed on your system. If not, follow the instructions below.


markdown
Copy code
# Zero-Knowledge Proof Example

This project demonstrates a simple implementation of a Zero-Knowledge Proof (ZKP) concept using Rust. It simulates a user and a server exchanging data for authentication without directly sharing sensitive information.

## Features
- Hashing using SHA256
- Random challenge generation
- Authentication simulation with deterministic checks

---

## Prerequisites

- **Rust**: Ensure Rust is installed on your system. If not, follow the instructions below.

### Getting started

1. Donwload Repo
2. Build:
     ```bash
     cargo build
     ```
2. Run:
     ```bash
     cargo run
     ```

### How It Works
 - The server generates a random salt and challenge.

 - The user provides a password and computes a response (hash of the password and challenge).

 - The server validates the user's response against its own computation.

 - If the responses match, authentication succeeds.
