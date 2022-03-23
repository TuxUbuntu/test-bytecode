
mod tests;

use nsm::plugins::Memory;
use nsm::plugins::Stack;
use nsm::plugins::Arithmetic;
use nsm::StateMachine;
use futures::StreamExt;
use actix_files::Files;
use env_logger::Env;
use actix_web::middleware::Logger;
use actix_web::{post, web, App, HttpServer, HttpResponse, Result};


#[post("/eval")]
async fn eval(mut payload: web::Payload) -> Result<HttpResponse> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        body.extend_from_slice(&chunk?);
    }
    let text = String::from_utf8(body.freeze().to_vec()).unwrap();
    let tape = text.parse().expect("Parse tape error");
    let mut main = StateMachine::default();
    main.register(Memory::default())?;
    main.register(Stack::default())?;
    main.register(Arithmetic::default())?;
    let res = main.read(&tape)?;
    let res = format!("{:?}", res);
    Ok(HttpResponse::Ok().body(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let files = Files::new("/", "web-service/static")
            .prefer_utf8(true)
            .index_file("index.html");
        App::new()
            .wrap(Logger::default())
            .service(eval)
            .service(files)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

