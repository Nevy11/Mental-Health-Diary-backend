use actix_cors::Cors;
use actix_web::{delete, http, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use goals::{
    create_goal::create_goal,
    delete_goal::delete_all_goals,
    read_goal::{read_all_goal, read_one_goal},
    update_goal::update_goal,
};
use models::{
    ChatUsers, DeleteUserPassword, Goal, IsSuccessful, LoginChatUsers, MessageResponse, SearchGoal,
    UpdateGoal, UpdateUserPassword, UpdateUsernameOrEmail,
};
use token_generation::generate_token::generate_token;
use users::{
    create_users::create_chat_user,
    delete_users::delete_chat_user,
    read_users::{check_for_users_password, read_all_chat_user},
    update_users::update_chat_user,
};
use validator::ValidateLength;

pub mod connection;
pub mod goals;
pub mod models;
pub mod roberta_model;
pub mod schema;
pub mod token_generation;
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

#[post("/goal_create")]
pub async fn goal_create(data: Json<Goal>) -> impl Responder {
    let user_option = data.username.clone().to_uppercase().length();
    match user_option {
        Some(user_length) => {
            if user_length <= 1 {
                let message = MessageResponse {
                    message: "Enter a valid username".to_string(),
                };
                HttpResponse::Ok().json(message)
            } else {
                let goal_to_be_created = Goal {
                    username: data.username.clone().to_uppercase(),
                    goal_name: data.goal_name.clone(),
                };
                let created_result = create_goal(goal_to_be_created);
                match created_result {
                    Ok(created_data) => HttpResponse::Ok().json(created_data),
                    Err(e) => {
                        let message = MessageResponse {
                            message: e.to_string(),
                        };
                        HttpResponse::Ok().json(message)
                    }
                }
            }
        }
        None => {
            let return_message = MessageResponse {
                message: String::from("Failed to find the length of the username"),
            };
            HttpResponse::Ok().json(return_message)
        }
    }
}

#[post("/goal_read_one")]
pub async fn goal_read_one(data: Json<SearchGoal>) -> impl Responder {
    let user_data = SearchGoal {
        username: data.username.clone().to_uppercase(),
    };
    let goal_data = read_one_goal(user_data);
    match goal_data {
        Ok(created_data) => HttpResponse::Ok().json(created_data),
        Err(e) => {
            let message = MessageResponse {
                message: e.to_string(),
            };
            HttpResponse::Ok().json(message)
        }
    }
}

#[post("/clear_goals")]
pub async fn clear_goals(data: Json<SearchGoal>) -> impl Responder {
    let user_data: SearchGoal = SearchGoal {
        username: data.username.clone().to_uppercase(),
    };
    let created_result = delete_all_goals(user_data.username);
    match created_result {
        Ok(created_data) => {
            println!("Deleted: {created_data:?}");
            let message = MessageResponse {
                message: format!("Data is cleared"),
            };
            HttpResponse::Ok().json(message)
        }
        Err(e) => {
            let message = MessageResponse {
                message: e.to_string(),
            };
            HttpResponse::Ok().json(message)
        }
    }
}

#[patch("/goal_update")]
pub async fn goal_update(data: Json<UpdateGoal>) -> impl Responder {
    // let user_data = UpdateUsernameOrEmail {
    //     username: data.username.clone().to_uppercase(),

    // };
    let created_result = update_goal(
        data.username.to_uppercase().clone(),
        data.old_value.clone(),
        data.new_value.clone(),
    );
    match created_result {
        Some(Ok(created_data)) => HttpResponse::Ok().json(created_data),
        Some(Err(e)) => {
            let message = MessageResponse {
                message: e.to_string(),
            };
            HttpResponse::Ok().json(message)
        }
        None => {
            let message = MessageResponse {
                message: String::from("Enter a valid field"),
            };
            HttpResponse::Ok().json(message)
        }
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
    println!("Separator: \n\n");
    let all_result_goals = read_all_goal();
    match all_result_goals {
        Ok(result_data) => println!("{result_data:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match generate_token("user123") {
        Ok(token) => println!("Generated Token: {}", token),
        Err(e) => println!("Error: {e:?}"),
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
            .service(goal_update)
            .service(clear_goals)
            .service(goal_read_one)
            .service(goal_create)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
