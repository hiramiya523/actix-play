use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use env_logger::Env;

// #[actix_web::main] は、エントリポイント。Actix Webランタイムを初期化している
#[actix_web::main]
async fn main() -> std::io::Result<()>{
  // Logging設定
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  log::info!("starting HTTP server at http://localhost:8080");
  HttpServer::new(|| {
      App::new()
      .service(hello)
      .wrap(Logger::default())
          // .service(echo) // ここでメソッドと、ルーティングのバインドみたいなことをしているのか
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}

// -> Responderトレイトを実装する任意の型 を戻り値に指定
#[get("/")]
async fn hello() -> impl Responder {
  // rustでは、最後の式を戻り値としてみなす。セミコロンを入れると文になるので注意
  HttpResponse::Ok()
  .body("Hello kaito!!")
}

