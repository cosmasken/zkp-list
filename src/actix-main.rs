use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use rand::Rng;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct ZkProofRequest {
    username: String,
    password_hash: String,
    proof: String,
}

#[derive(Serialize, Deserialize)]
struct ChallengeResponse {
    challenge: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))
}

struct AppState {
    user_data: Mutex<HashMap<String, String>>, // Simulate a user database
    challenges: Mutex<HashMap<String, String>>, // Store ongoing challenges
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        user_data: Mutex::new({
            let mut users = HashMap::new();
            users.insert("Alice".to_string(), hash_password("Alice123"));
            users.insert("Bob".to_string(), hash_password("BobPassword"));
            users
        }),
        challenges: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .service(generate_challenge)
            .service(verify_proof)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[post("/challenge")]
async fn generate_challenge(
    app_state: web::Data<AppState>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let fallback_username = String::new();
    let username = query.get("username").unwrap_or(&fallback_username);
   // let username = query.get("username").unwrap_or(&"".to_string());

    if username.is_empty() {
        return HttpResponse::BadRequest().body("Username is required");
    }

    let rng = rand::thread_rng();
    let challenge: String = rng
        .sample_iter(rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    app_state
        .challenges
        .lock()
        .unwrap()
        .insert(username.clone(), challenge.clone());

    HttpResponse::Ok().json(ChallengeResponse { challenge })
}

#[post("/verify")]
async fn verify_proof(
    app_state: web::Data<AppState>,
    proof_request: web::Json<ZkProofRequest>,
) -> impl Responder {
    let stored_hash = {
        let user_data = app_state.user_data.lock().unwrap();
        user_data.get(&proof_request.username).cloned()
    };

    let challenge = {
        let challenges = app_state.challenges.lock().unwrap();
        challenges.get(&proof_request.username).cloned()
    };

    if let (Some(stored_hash), Some(challenge)) = (stored_hash, challenge) {
        let expected_proof = hash_with_challenge(stored_hash, &challenge);

        // Verify that the provided proof matches the expected proof
        if proof_request.proof == expected_proof {
            return HttpResponse::Ok().body(format!(
                "User {} authenticated successfully",
                proof_request.username
            ));
        }
    }

    HttpResponse::Unauthorized().body("Authentication failed")
}

fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}

fn hash_with_challenge(hash: String, challenge: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(hash);
    hasher.update(challenge);
    format!("{:x}", hasher.finalize())
}
