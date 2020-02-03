#![feature(proc_macro_hygiene)]

#[macro_use] extern crate rocket;
use serde::{Serialize, Deserialize};

use rocket::http::{Status};

use jsonwebtoken::{decode,  Validation, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   username: String
}


#[get("/hello?<token>")]
fn hello(token: String) -> Result<String, Status> {
    let _token_data = decode::<Claims>(&token, &DecodingKey::from_secret("kaoyjoAbPAlZdvXtenphsQnOd-rni5w3cvb3u_bWfvBoAMZLBeYs006gt5BPdQXZKBTiDQXwSIGeJSMEnaocPgpe9eSYOnDGR1cScC48E0Z-bpt6D9CbHRvf1rv4E9Z_d2lYlix3Yomaq198ap_dsICoqEffNH9aWl3sB_GLmS7yU7RehlTp1qye3Z-Y5FMnvlAUp0DqxcMDGoUAWYV_h-Rl_N8ZbB7YoH2MQ8N6N9S6rSUs6qnTqulmO7nT-Tbfg9-7FgZhahDuweEltC24lvkpTDEa5fQlmza4EqtCdi6tX39SNIOnj668w9_1fTJDJtsMx3OWzteine1yQc_KVQ".as_ref()), &Validation::default());
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