extern crate rand;
extern crate ed25519_dalek;

use rand::rngs::OsRng;
use actix_web::{post, get, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use bulletproofs::PedersenGens;
use curve25519_dalek::scalar::Scalar;

#[derive(Serialize, Deserialize)]
struct CommitRequest {
    secret1: u64,
    secret2: u64,
    public_key: Option<String>, // Optional public key for proof of knowledge
}

#[derive(Serialize, Deserialize)]
struct CommitResponse {
    commitment1: String,
    commitment2: String,
    sum_commitment: String,
    randomness1: String,
    randomness2: String,
}

#[derive(Serialize, Deserialize)]
struct ProofResponse {
    challenge: String, // Verifier's challenge
}

#[derive(Serialize, Deserialize)]
struct ProofVerifyRequest {
    response: String,       // Prover's response
    challenge: String,      // Verifier's challenge
    public_key: String,     // Prover's public key
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))
}

#[post("/commit")]
async fn commit(req: web::Json<CommitRequest>) -> impl Responder {
    let mut rng = OsRng;

    // Generate Pedersen Generators
    let pedersen_gens = PedersenGens::default();

    // Convert secrets to Scalars
    let secret1 = Scalar::from(req.secret1);
    let secret2 = Scalar::from(req.secret2);

    // Generate random blinding factors
    let randomness1 = Scalar::random(&mut rng);
    let randomness2 = Scalar::random(&mut rng);

    // Create commitments
    let commitment1 = pedersen_gens.commit(secret1, randomness1);
    let commitment2 = pedersen_gens.commit(secret2, randomness2);

    // Sum commitments
    let sum = req.secret1 + req.secret2;
    let sum_commitment = pedersen_gens.commit(Scalar::from(sum), randomness1 + randomness2);

    HttpResponse::Ok().json(CommitResponse {
        commitment1: hex::encode(commitment1.compress().to_bytes()),
        commitment2: hex::encode(commitment2.compress().to_bytes()),
        sum_commitment: hex::encode(sum_commitment.compress().to_bytes()),
        randomness1: hex::encode(randomness1.to_bytes()),
        randomness2: hex::encode(randomness2.to_bytes()),
    })
}

#[post("/challenge")]
async fn generate_challenge(req: web::Json<CommitRequest>) -> impl Responder {
    use rand::Rng;
    let mut rng = OsRng;

    if let Some(public_key) = &req.public_key {
        let challenge: u64 = rng.gen_range(1..1_000_000); // Generate a random challenge
        HttpResponse::Ok().json(ProofResponse {
            challenge: challenge.to_string(),
        })
    } else {
        HttpResponse::BadRequest().body("Public key required for proof of knowledge")
    }
}

#[post("/verify_proof")]
async fn verify_proof(req: web::Json<ProofVerifyRequest>) -> impl Responder {
    let public_key_bytes = match hex::decode(&req.public_key) {
        Ok(bytes) => bytes,
        Err(_) => return HttpResponse::BadRequest().body("Invalid public key encoding."),
    };

    let response: u64 = match req.response.parse() {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().body("Invalid response."),
    };

    let challenge: u64 = match req.challenge.parse() {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().body("Invalid challenge."),
    };

    // Example: Verify response
    // Adjust this logic to suit the actual proof scheme.
    if response == challenge + public_key_bytes[0] as u64 { // Replace with actual verification logic
        HttpResponse::Ok().body("Proof verified successfully!")
    } else {
        HttpResponse::BadRequest().body("Verification failed.")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(commit)
            .service(generate_challenge)
            .service(verify_proof)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
