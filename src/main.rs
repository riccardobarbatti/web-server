#![allow(dead_code)]
use std::env;
use http::Request;
use http::Method;
use server::Server;
use website_handler::WebsiteHandler;

//import server
mod http;
mod server;
mod website_handler;

// const PORT: &str = "8000";
// const IP: &str = "127.0.0.1:";


fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));

    //OLD
    // let url= [IP, PORT].join("");
    // let server = Server::new(url);
    // //dbg!(server);
    // server.run(website_handler);

}

// mod http {
//     pub mod Request {}
//     pub mod method {}
//
// }
