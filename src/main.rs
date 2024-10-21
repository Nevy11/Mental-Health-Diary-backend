use actix_cors::Cors;
use actix_web::{get, http, post, web::Json, App, HttpResponse, HttpServer, Responder};
use models::QuestionAnswer;

pub mod models;

#[get("get_test")]
pub async fn get_test() -> impl Responder {
    let data = QuestionAnswer {
        username: String::from("Nevy11"),
        question: String::from("Good morning?"),
        answer: String::from("Am doing fine, what do you want me to do for you today?"),
    };
    HttpResponse::Ok().json(data)
}

#[post("question_answer")]
pub async fn get_question_answer(data: Json<QuestionAnswer>) -> impl Responder {
    let username = data.username.clone();
    let question = data.question.clone();
    let answer = data.answer.clone();
    println!("Username: {username}:\nQuestion: {question}\nAnswer: {answer}");
    let returning_data = QuestionAnswer {
        username,
        question,
        answer,
    };
    HttpResponse::Ok().json(returning_data)
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, Steve!");
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:4200")
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(get_question_answer)
            .service(get_test)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
