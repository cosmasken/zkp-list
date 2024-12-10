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