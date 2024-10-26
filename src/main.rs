use actix_cors::Cors;
use actix_web::{delete, http, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use models::{
    ChatUsers, DeleteUserPassword, IsSuccessful, LoginChatUsers, MessageResponse,
    UpdateUserPassword, UpdateUsernameOrEmail,
};
use users::{
    create_users::create_chat_user,
    delete_users::delete_chat_user,
    read_users::{check_for_users_password, read_all_chat_user},
    update_users::update_chat_user,
};

pub mod connection;
pub mod goals;
pub mod models;
pub mod roberta_model;
pub mod schema;
pub mod users;

#[post("/sign_up_user")]
pub async fn sign_up_user(data: Json<ChatUsers>) -> impl Responder {
    let data_to_create = ChatUsers {
        username: data.username.clone().to_uppercase(),
        userpassword: data.userpassword.clone(),
        email: data.email.clone().to_uppercase(),
    };
    let created_result = create_chat_user(data_to_create);
    match created_result {
        Ok(created_data) => {
            println!("Created: {:?} successfully", created_data.username);
            let return_data = IsSuccessful { is_it: true };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            println!("{e:?}");
            let return_data = IsSuccessful { is_it: false };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/login_user")]
pub async fn login_user(data: Json<LoginChatUsers>) -> impl Responder {
    let login_result = check_for_users_password(data.username.clone(), data.userpassword.clone());
    let return_data = IsSuccessful {
        is_it: login_result,
    };
    HttpResponse::Ok().json(return_data)
}

#[patch("/update_user_password")]
pub async fn update_user_password(data: Json<UpdateUserPassword>) -> impl Responder {
    let field = String::from("userpassword");
    let islogged = check_for_users_password(
        data.username.to_uppercase().clone(),
        data.current_password.clone(),
    );
    if islogged {
        let udpated_result =
            update_chat_user(data.username.clone(), data.new_password.clone(), field);
        match udpated_result {
            Some(Ok(updated_data)) => {
                println!(
                    "user {} password is updated successfully",
                    updated_data.username
                );
                let return_data = IsSuccessful { is_it: true };
                HttpResponse::Ok().json(return_data)
            }
            Some(Err(e)) => {
                println!("Error: {:?}", e);
                let return_data = IsSuccessful { is_it: false };
                HttpResponse::Ok().json(return_data)
            }
            None => {
                let return_data = IsSuccessful { is_it: false };
                HttpResponse::Ok().json(return_data)
            }
        }
    } else {
        HttpResponse::Ok().body("Incorrect current password")
    }
}

#[patch("update_email_or_username")]
pub async fn update_email_or_username(data: Json<UpdateUsernameOrEmail>) -> impl Responder {
    let name_of_user = data.username.clone();
    let field_of_user = data.field.clone().to_uppercase();
    let new_value_of_user = data.new_value.clone();
    let field_of_user = field_of_user.as_str();
    match field_of_user {
        "USERNAME" => {
            let user_data_option =
                update_chat_user(name_of_user, new_value_of_user, "USERNAME".to_string());
            match user_data_option {
                Some(data) => match data {
                    Ok(_) => {
                        let return_data = IsSuccessful { is_it: true };
                        HttpResponse::Ok().json(return_data)
                    }
                    Err(e) => {
                        println!("Error emerged: {e:?}");

                        let return_data = IsSuccessful { is_it: false };
                        HttpResponse::Ok().json(return_data)
                    }
                },
                None => {
                    println!("Invalid field entered");
                    let return_data = IsSuccessful { is_it: false };
                    HttpResponse::Ok().json(return_data)
                }
            }
        }
        "EMAIL" => {
            let user_data_option =
                update_chat_user(name_of_user, new_value_of_user, "EMAIL".to_string());
            match user_data_option {
                Some(data) => match data {
                    Ok(_) => {
                        let return_data = IsSuccessful { is_it: true };
                        HttpResponse::Ok().json(return_data)
                    }
                    Err(e) => {
                        println!("Error emerged: {e:?}");

                        let return_data = IsSuccessful { is_it: false };
                        HttpResponse::Ok().json(return_data)
                    }
                },
                None => {
                    println!("Invalid field entered");
                    let return_data = IsSuccessful { is_it: false };
                    HttpResponse::Ok().json(return_data)
                }
            }
        }
        _ => {
            let return_data = format!("Nothing has matched to be updated");
            let return_data = IsSuccessful { is_it: false };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[delete("/delete_user")]
pub async fn delete_user(data: Json<DeleteUserPassword>) -> impl Responder {
    let is_logged = check_for_users_password(
        data.username.clone().to_uppercase(),
        data.current_password.clone(),
    );
    if is_logged {
        let deleted_result = delete_chat_user(data.username.to_uppercase().clone());
        match deleted_result {
            Ok(deleted_data) => {
                println!("Deleted: {:?} successfully", deleted_data.username);
                let return_data = IsSuccessful { is_it: true };
                HttpResponse::Ok().json(return_data)
            }

            Err(e) => {
                println!("Error: {e:?}");
                let return_data = IsSuccessful { is_it: false };
                HttpResponse::Ok().json(return_data)
            }
        }
    } else {
        let return_data = IsSuccessful { is_it: false };
        HttpResponse::Ok().json(return_data)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let all_result = read_all_chat_user();
    match all_result {
        Ok(result) => {
            println!("{result:?}");
        }
        Err(e) => {
            println!("{e:?}")
        }
    }
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:4200")
                    .allowed_origin("http://192.168.137.210:10000")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(sign_up_user)
            .service(login_user)
            .service(update_user_password)
            .service(update_email_or_username)
            .service(delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
