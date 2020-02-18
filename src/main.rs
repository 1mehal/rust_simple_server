#![feature(proc_macro_hygiene)]

#[macro_use] extern crate rocket;
use serde::{Serialize, Deserialize};

use rocket::http::{Status};

use jsonwebtoken::{decode,  Validation, DecodingKey, Algorithm};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   username: String
}


#[get("/hello?<token>")]
fn hello(token: String) -> Result<String, Status> {
    let pubkey_pem = include_bytes!("asdemo_jwt256.key.pub");

    let _token_data = decode::<Claims>(&token, &DecodingKey::from_rsa_pem(pubkey_pem).unwrap(), &Validation::new(Algorithm::RS256));
    match _token_data {
        Ok(c) => {
            return Ok(format!("Dear, {}, greetings from far far away rust server", c.claims.username));
        },
        Err(_err) => {
           return Err(Status::Unauthorized)         
        }
    };
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}