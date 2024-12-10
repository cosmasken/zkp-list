## Getting Started

### Prerequisites

Make sure you have the following installed:
- Rust (with Cargo) - [Install Rust](https://www.rust-lang.org/tools/install)
- Node.js and npm - [Install Node.js](https://nodejs.org/)
- Actix Web (Rust) - No need for separate installation; the project uses it as a dependency.

### Backend Setup

1. Navigate to the project directory:

    ```bash
    cd zkp-list
    ```

2. Build and run the backend:

    ```bash
    cargo build && cargo run
    ```

    This will start the backend server on `http://127.0.0.1:8080`.

### Frontend Setup

1. Navigate to the `frontend` directory:

    ```bash
    cd frontend
    ```

2. Install dependencies:

    ```bash
    npm install
    ```

3. Build the frontend:

    ```bash
    npm run build
    ```

   This will generate the static files in the `build` folder.

4. The backend will serve the static files from the `build` directory. Open [http://localhost:8080](http://localhost:8080) in your browser to access the application.

### How It Works

- **Generate Proof**: The frontend allows users to input a value `x` and generate a ZKP for the equation `x^3 + x + 5 = 35`. The backend uses the **bellman** library to generate the proof for this equation.
  
- **Verify Proof**: After generating a proof, users can enter the expected output (`35` in this case) to verify the proof.

The backend serves as the API for proof generation and verification, while the frontend provides an interface for non-technical users to interact with the system.

### API Endpoints

- **POST `/generate-proof`**: 
  - Request Body: `{ "x": "3" }` (value of `x`).
  - Response: Returns the generated proof for the equation.
  
- **POST `/verify-proof`**: 
  - Request Body: `{ "proof": "generated-proof-string", "output": "35" }`.
  - Response: Returns whether the proof is valid or not.

### Technologies Used

- **Backend**: Actix Web, Rust, bellman (for Zero-Knowledge Proofs)
- **Frontend**: React, JavaScript, HTML, CSS
- **Zero-Knowledge Proof**: Using the **bellman** library for zk-SNARK proof generation and verification.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

### Credits

- **bellman** - The library used for Zero-Knowledge Proofs.
- **Actix** - Rust web framework for backend API.
- **React** - Frontend framework for building the user interface.