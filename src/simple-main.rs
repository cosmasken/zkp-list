use std::fmt::format;

use rand::Rng;

//simulated user and server
struct User {
    username: String,
    password: String,
}

struct Server {
    salt : String //randomly generated
}

impl Server {

    fn authenticate(&self, user : &User, response: &str) -> bool {
        //Simulate server challenge
        let challenge: &String = &self.salt;

        //simulate user's response
        let user_response  =  hash(user.password.clone(),challenge);

        //compare response
        response == user_response
    }

   
}

fn hash(input :String, salt: &str) -> String {
    //TODO: simulate hash function using SHA256
    format!("HASH({}+{})", input, salt)
}

fn main(){
    let user : User = User {
        username: "Lauren".to_string(),
        password: "Password123".to_string(),
    };

    let server :Server = Server {
        salt: rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect(),
    };

    //user authentication
    let response : String = hash(user.password.clone(), &server.salt);

    //implement server authentication
    if server.authenticate(&user, &response){
        println!("User {} authenticated successfully", user.username);
    } else {
        println!("User {} authentication failed", user.username);
    }
}