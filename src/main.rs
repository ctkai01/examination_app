use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use todo_actix::logger::{get_subscriber, init_subscriber};
use tracing_actix_web::TracingLogger;

async fn test() -> impl Responder {
    HttpResponse::Ok().json("asas")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the server at 127.0.0.1:8000");

    let subscriber = get_subscriber("examination_app".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(test))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
