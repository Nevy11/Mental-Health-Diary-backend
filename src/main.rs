use crate::audio_text::memory_wav::{convert_to_wav_in_memory, transcribe_audio_from_memory};
use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{
    delete, get, http, patch, post, web::Json, App, HttpResponse, HttpServer, Responder,
};
use ai::{
    create_ai::create_ai,
    delete_ai::delete_ai,
    read_ai::read_one_ai,
    update_ai::{update_answer_ai, update_question_ai, update_username_ai},
};
use audio_text::{
    whisper_doc::whisper_transcribe_medium,
    whisper_mod_2::{save_audio, wav_convert_file, whisper_transcribe_medium2},
};
use day_of_week::{
    date::date, day_month::current_day, day_time::current_month, day_year::current_year,
};

use diary::{
    create_diary::create_diary, delete_diary::delete_diary, read_diary::read_one_diary_content,
    update_diary::update_diary,
};
use favourite_day::{
    create_favourite_day::create_favourite_day, delete_favourite_day::delete_favourite_day,
    read_all_favourite_day::read_all_favourite_day, read_one_favourite_day::read_one_favourite_day,
    update_favourite_day::update_favourite_day,
};
use futures_util::{StreamExt, TryStreamExt};
use goals::{
    create_goal::create_goal,
    delete_goal::{delete_all_goals, delete_goal},
    goals_done::{
        create_done_goal::create_done_goal,
        delete_done_goal::{delete_all_done_goals, delete_done_goal},
        read_done_goal::read_one_done_goal,
        update_done_goal::update_done_goal,
    },
    read_goal::read_one_goal,
    update_goal::update_goal,
};
use models::{
    Ai, AiReadOne, AiReturn, ChatUsers, CheckIfGoalExists, CurrentDay, CurrentMonth, CurrentYear,
    DeleteUserPassword, Diary, DiaryExists, DiaryReturn, ErrorReturn, FavouriteDay,
    FavouriteDayReadAllReturn, FavouriteDayReadOne, FavouriteDayReturn, FavouriteDayUpdate, Goal,
    GoalDone, GoalUpdateReturn, IsSuccessful, LoginChatUsers, MessageResponse, MyDate, QAUpdateAi,
    ReturnAi, ReturnAiReadOne, ReturnAudioData, ReturnChatUserReadOne, SearchGoal, SuccessReadOne,
    SuccessReturn, UpdateGoal, UpdateUserPassword, UpdateUsernameOrEmail, UsernameUpdateAi,
};
use users::{
    create_users::create_chat_user,
    delete_users::delete_chat_user,
    read_users::{check_for_users_password, read_one_chat_user},
    update_users::update_chat_user,
};
use validator::ValidateLength;

pub mod ai;
pub mod audio_text;
pub mod connection;
pub mod day_of_week;
pub mod diary;
pub mod favourite_day;
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

#[post("/user_read_one")]
pub async fn user_read_one(data: Json<AiReadOne>) -> impl Responder {
    let user_result = read_one_chat_user(data.username.clone());
    match user_result {
        Ok(user_data) => {
            let return_data = ReturnChatUserReadOne {
                success: true,
                username: user_data.username,
                email: user_data.email,
                message: "Successfully read one".to_string(),
                add_message: "The data has been fetched successfully".to_string(),
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = ReturnChatUserReadOne {
                success: false,
                username: format!("Err"),
                email: format!("Error"),
                message: format!("{e:?}"),
                add_message: format!("Failed to read the user data"),
            };
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
    let created_result = update_goal(
        data.username.to_uppercase().clone(),
        data.old_value.clone(),
        data.new_value.clone(),
    );
    match created_result {
        Some(Ok(created_data)) => {
            let return_data = GoalUpdateReturn {
                id: created_data.id,
                username: created_data.username,
                goal_name: created_data.goal_name,
                message: format!("Success"),
            };
            HttpResponse::Ok().json(return_data)
        }
        Some(Err(e)) => {
            let return_data = GoalUpdateReturn {
                id: 0,
                username: String::from(""),
                goal_name: String::from(""),
                message: e.to_string(),
            };
            HttpResponse::Ok().json(return_data)
        }
        None => {
            let return_data = GoalUpdateReturn {
                id: 0,
                username: String::from(""),
                goal_name: String::from(""),
                message: format!("The data doesn't match the available one to be updated"),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/delete_one_goal")]
pub async fn delete_one_goal(data: Json<Goal>) -> impl Responder {
    let user_data = Goal {
        username: data.username.clone().to_uppercase(),
        goal_name: data.goal_name.clone(),
    };
    let created_result = delete_goal(user_data);
    match created_result {
        Ok(created_data) => {
            println!("Deleted: {created_data:?}");
            let message = MessageResponse {
                message: format!(
                    "{:?} is deleted from the todo table in database",
                    created_data.goal_name
                ),
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

#[post("/check_todo_exists")]
pub async fn check_todo_exists(data: Json<Goal>) -> impl Responder {
    let search_data = SearchGoal {
        username: data.username.clone(),
    };
    let user_result = read_one_goal(search_data);
    let mut is_it = false;
    match user_result {
        Ok(user_data) => {
            for x in user_data {
                if data.goal_name == x.goal_name {
                    is_it = true;
                }
            }
            if is_it {
                let return_data = CheckIfGoalExists {
                    exists: true,
                    message: format!(""),
                    success: true,
                };
                HttpResponse::Ok().json(return_data)
            } else {
                let return_data = CheckIfGoalExists {
                    exists: false,
                    message: format!(""),
                    success: true,
                };
                HttpResponse::Ok().json(return_data)
            }
        }
        Err(e) => {
            let return_data = CheckIfGoalExists {
                exists: false,
                message: format!("{e:?}"),
                success: false,
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/check_done_exists")]
pub async fn check_done_exists(data: Json<Goal>) -> impl Responder {
    let search_data = SearchGoal {
        username: data.username.clone(),
    };
    let user_result = read_one_done_goal(search_data);
    let mut is_it = false;
    match user_result {
        Ok(user_data) => {
            for x in user_data {
                if data.goal_name == x.goal_name {
                    is_it = true;
                }
            }
            if is_it {
                let return_data = CheckIfGoalExists {
                    exists: true,
                    message: format!("Goal matches the goal stored in the database"),
                    success: true,
                };
                HttpResponse::Ok().json(return_data)
            } else {
                let return_data = CheckIfGoalExists {
                    exists: false,
                    message: format!("Data updated successfully"),
                    success: true,
                };
                HttpResponse::Ok().json(return_data)
            }
        }
        Err(e) => {
            let return_data = CheckIfGoalExists {
                exists: false,
                message: format!("{e:?}"),
                success: false,
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/done_goal_create")]
pub async fn done_goal_create(data: Json<GoalDone>) -> impl Responder {
    let data_filtered = GoalDone {
        username: data.username.to_uppercase().clone(),
        goal_name: data.goal_name.clone(),
    };
    let created_done_goal = create_done_goal(data_filtered);
    match created_done_goal {
        Ok(created_data) => {
            let return_data = SuccessReturn {
                success: true,
                username: created_data.username,
                goal_name: created_data.goal_name,
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = ErrorReturn {
                success: false,
                message: e.to_string(),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/done_goal_read_one")]
pub async fn done_goal_read_one(data: Json<SearchGoal>) -> impl Responder {
    let one_done_goal = SearchGoal {
        username: data.username.clone(),
    };
    let created_done_goal = read_one_done_goal(one_done_goal);
    match created_done_goal {
        Ok(created_data) => HttpResponse::Ok().json(&created_data),
        Err(e) => {
            let return_data = ErrorReturn {
                success: false,
                message: e.to_string(),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[patch("/done_goal_update")]
pub async fn done_goal_update(data: Json<UpdateGoal>) -> impl Responder {
    let created_done_goal = update_done_goal(
        data.username.clone(),
        data.old_value.clone(),
        data.new_value.clone(),
    );
    match created_done_goal {
        Some(Ok(created_data)) => {
            let return_data = SuccessReturn {
                success: true,
                username: created_data.username,
                goal_name: created_data.goal_name,
            };
            HttpResponse::Ok().json(return_data)
        }
        Some(Err(e)) => {
            let return_data = ErrorReturn {
                success: false,
                message: e.to_string(),
            };
            HttpResponse::Ok().json(return_data)
        }
        None => {
            let return_data = ErrorReturn {
                success: false,
                message: format!("The goal entered doesn't match the available one"),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/done_goal_delete")]
pub async fn done_goal_delete(data: Json<GoalDone>) -> impl Responder {
    let one_done_goal = GoalDone {
        username: data.username.clone().to_uppercase(),
        goal_name: data.goal_name.clone(),
    };

    let created_done_goal = delete_done_goal(one_done_goal);
    match created_done_goal {
        Ok(created_data) => {
            let return_data = SuccessReturn {
                success: true,
                username: created_data.username,
                goal_name: created_data.goal_name,
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = ErrorReturn {
                success: false,
                message: e.to_string(),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/done_goal_delete_all")]
pub async fn done_goal_delete_all(data: Json<SearchGoal>) -> impl Responder {
    let one_done_goal = SearchGoal {
        username: data.username.clone().to_uppercase(),
    };

    let created_done_goal = delete_all_done_goals(one_done_goal.username);
    match created_done_goal {
        Ok(created_data) => {
            let return_data = SuccessReadOne {
                success: true,
                data: created_data,
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = ErrorReturn {
                success: false,
                message: e.to_string(),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[get("day_current")]
pub async fn day_current() -> impl Responder {
    let day = current_day();
    let day_return = CurrentDay { day: day };
    HttpResponse::Ok().json(day_return)
}

#[get("month_current")]
pub async fn month_current() -> impl Responder {
    let month = current_month();
    let day_return = CurrentMonth { month: month };
    HttpResponse::Ok().json(day_return)
}

#[get("year_current")]
pub async fn year_current() -> impl Responder {
    let year: i32 = current_year();
    let day_return = CurrentYear { year: year };
    HttpResponse::Ok().json(day_return)
}

#[post("fav_day_create")]
pub async fn fav_day_create(data: Json<FavouriteDay>) -> impl Responder {
    let data_to_create = FavouriteDay {
        username: data.username.clone(),
        day_favourite: data.day_favourite.clone(),
    };
    let length_of_username = data_to_create.day_favourite.length();
    if length_of_username > Some(0) {
        let created_result = create_favourite_day(data_to_create);
        match created_result {
            Ok(created_data) => {
                let data_return = FavouriteDayReturn {
                    username: created_data.username,
                    day_favourite: created_data.day_favourite,
                    message: String::from(""),
                    success: true,
                };
                HttpResponse::Ok().json(data_return)
            }
            Err(e) => {
                println!("Error: {e:?}");
                let update_result = update_favourite_day(
                    data.username.clone(),
                    String::from("DAY_FAVOURITE"),
                    data.day_favourite.clone(),
                );
                match update_result {
                    Some(Ok(updated_data)) => {
                        let data_return = FavouriteDayReturn {
                            username: updated_data.username,
                            day_favourite: updated_data.day_favourite,
                            message: format!(""),
                            success: true,
                        };
                        HttpResponse::Ok().json(data_return)
                    }
                    Some(Err(e)) => {
                        let data_return = FavouriteDayReturn {
                            username: String::from(""),
                            day_favourite: String::from(""),
                            message: format!("{e:?}"),
                            success: false,
                        };
                        HttpResponse::Ok().json(data_return)
                    }
                    None => {
                        let data_return = FavouriteDayReturn {
                            username: String::from(""),
                            day_favourite: String::from(""),
                            message: format!("A valid field for upgrading is required"),
                            success: false,
                        };
                        HttpResponse::Ok().json(data_return)
                    }
                }
            }
        }
    } else {
        let data_return = FavouriteDayReturn {
            username: String::from(""),
            day_favourite: String::from(""),
            message: format!("Enter a valid username"),
            success: false,
        };
        HttpResponse::Ok().json(data_return)
    }
}
#[post("fav_day_read_one")]
pub async fn fav_day_read_one(data: Json<FavouriteDayReadOne>) -> impl Responder {
    let created_result = read_one_favourite_day(data.username.clone());
    match created_result {
        Ok(created_data) => {
            let data_return = FavouriteDayReturn {
                username: created_data.username,
                day_favourite: created_data.day_favourite,
                message: String::from(""),
                success: true,
            };
            HttpResponse::Ok().json(data_return)
        }
        Err(e) => {
            let data_return = FavouriteDayReturn {
                username: String::from(""),
                day_favourite: String::from(""),
                message: format!("{e:?}"),
                success: false,
            };
            HttpResponse::Ok().json(data_return)
        }
    }
}

#[post("fav_day_read_all")]
pub async fn fav_day_read_all() -> impl Responder {
    let created_result = read_all_favourite_day();
    match created_result {
        Ok(created_data) => {
            let data_return = FavouriteDayReadAllReturn {
                success: true,
                data: created_data,
                message: String::from(""),
            };

            HttpResponse::Ok().json(data_return)
        }
        Err(e) => {
            let data = FavouriteDay {
                username: String::from(""),
                day_favourite: String::from(""),
            };
            let data_return = FavouriteDayReadAllReturn {
                data: vec![data],
                success: false,
                message: format!("{e:?}"),
            };
            HttpResponse::Ok().json(data_return)
        }
    }
}
#[patch("fav_day_update")]
pub async fn fav_day_update(data: Json<FavouriteDayUpdate>) -> impl Responder {
    let created_result = update_favourite_day(
        data.username.clone(),
        data.field.clone(),
        data.new_value.clone(),
    );
    match created_result {
        Some(Ok(created_data)) => {
            let data_return = FavouriteDayReturn {
                username: created_data.username,
                day_favourite: created_data.day_favourite,
                message: String::from(""),
                success: true,
            };
            HttpResponse::Ok().json(data_return)
        }
        Some(Err(e)) => {
            let data_return = FavouriteDayReturn {
                username: String::from(""),
                day_favourite: String::from(""),
                message: format!("{e:?}"),
                success: false,
            };
            HttpResponse::Ok().json(data_return)
        }
        None => {
            let data_return = FavouriteDayReturn {
                username: String::from(""),
                day_favourite: String::from(""),
                message: String::from("Enter a valid field"),
                success: false,
            };
            HttpResponse::Ok().json(data_return)
        }
    }
}
#[post("fav_day_delete")]
pub async fn fav_day_delete(data: Json<FavouriteDayReadOne>) -> impl Responder {
    let created_result = delete_favourite_day(data.username.clone());
    match created_result {
        Ok(created_data) => {
            let data_return = FavouriteDayReturn {
                username: created_data.username,
                day_favourite: created_data.day_favourite,
                message: String::from(""),
                success: true,
            };
            HttpResponse::Ok().json(data_return)
        }
        Err(e) => {
            let data_return = FavouriteDayReturn {
                username: String::from(""),
                day_favourite: String::from(""),
                message: format!("{e:?}"),
                success: false,
            };
            HttpResponse::Ok().json(data_return)
        }
    }
}

#[post("/diary_create")]
pub async fn diary_create(data: Json<Diary>) -> impl Responder {
    let created_data = Diary {
        username: data.username.clone(),
        content: data.content.clone(),
    };
    let created_result = create_diary(created_data);
    match created_result {
        Ok(created_data) => {
            let return_data = DiaryReturn {
                username: created_data.username,
                content: created_data.content,
                message: format!("Saved successfully"),
                success: true,
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = DiaryReturn {
                username: format!(""),
                content: format!(""),
                message: format!("{e:?}"),
                success: false,
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/diary_read_one")]
pub async fn diary_read_one(data: Json<FavouriteDayReadOne>) -> impl Responder {
    let created_result = read_one_diary_content(data.username.clone());
    match created_result {
        Ok(created_data) => {
            let return_data = DiaryReturn {
                username: created_data.username,
                content: created_data.content,
                message: format!(""),
                success: true,
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = DiaryReturn {
                username: format!(""),
                content: format!(""),
                message: format!("{e:?}"),
                success: false,
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/diary_delete")]
pub async fn diary_delete(data: Json<FavouriteDayReadOne>) -> impl Responder {
    let created_result = delete_diary(data.username.clone());
    match created_result {
        Ok(created_data) => {
            let return_data = DiaryReturn {
                username: created_data.username.clone(),
                content: created_data.content,
                message: format!("Deleted diary user:  {}", created_data.username.clone()),
                success: true,
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = DiaryReturn {
                username: format!(""),
                content: format!(""),
                message: format!("{e:?}"),
                success: false,
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[patch("/diary_udpate")]
pub async fn diary_udpate(data: Json<FavouriteDayUpdate>) -> impl Responder {
    let updated_data = FavouriteDayUpdate {
        username: data.username.clone().to_uppercase(),
        field: data.field.clone(),
        new_value: data.new_value.clone(),
    };
    let created_result = update_diary(
        updated_data.username,
        updated_data.field,
        updated_data.new_value,
    );
    match created_result {
        Some(Ok(created_data)) => {
            let return_data = DiaryReturn {
                username: created_data.username.clone(),
                content: created_data.content,
                message: format!(
                    "Diary of username: {} has been updated successfully",
                    created_data.username.clone()
                ),
                success: true,
            };
            HttpResponse::Ok().json(return_data)
        }
        Some(Err(e)) => {
            let return_data = DiaryReturn {
                username: format!(""),
                content: format!(""),
                message: format!("{e:?}"),
                success: false,
            };
            HttpResponse::Ok().json(return_data)
        }
        None => {
            let return_data = DiaryReturn {
                username: format!(""),
                content: format!(""),
                message: format!("Enter a Valid field."),
                success: false,
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/check_if_user_exists")]
pub async fn check_if_user_exists(data: Json<FavouriteDayReadOne>) -> impl Responder {
    let created_result = delete_diary(data.username.clone());
    match created_result {
        Ok(_) => {
            let return_data = DiaryExists {
                exists: true,
                message: format!("User exists in the diary table"),
                success: true,
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = DiaryExists {
                exists: false,
                message: format!("{e:?}"),
                success: false,
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[get("/date_actual")]
pub async fn date_actual() -> impl Responder {
    let date_returned = MyDate {
        date: date().to_string(),
    };
    HttpResponse::Ok().json(date_returned)
}

#[post("/ai_create")]
pub async fn ai_create(data: Json<Ai>) -> impl Responder {
    let data = Ai {
        username: data.username.clone(),
        question: data.question.clone(),
        answer: data.answer.clone(),
    };
    let username = data.username.clone();
    let question = data.question.clone();
    let answer = data.answer.clone();
    let created_result = create_ai(data);
    match created_result {
        Ok(created_data) => {
            let return_data = ReturnAi {
                username: created_data.username,
                question: created_data.question,
                answer: created_data.answer,
                success: true,
                message: format!(""),
                add_message: format!("Data is created successfully"),
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = ReturnAi {
                username: format! {"{}", username.clone()},
                question: format!("{}", question.clone()),
                answer: format!("{}", answer.clone()),
                success: false,
                message: format!("{e:?}"),
                add_message: format!("Failed to create username: {}", username.clone()),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/ai_read_one")]
pub async fn ai_read_one(data: Json<AiReadOne>) -> impl Responder {
    let username = data.username.clone();

    let one_result = read_one_ai(username);
    match one_result {
        Ok(one_data) => {
            if one_data.length() > Some(0) {
                let return_data = ReturnAiReadOne {
                    data: one_data,
                    success: true,
                    message: format!("Successfully"),
                    add_message: format!("Data is read successfully"),
                };
                HttpResponse::Ok().json(return_data)
            } else {
                let return_data = ReturnAiReadOne {
                    data: one_data,
                    success: false,
                    message: format!("The data is not in the database."),
                    add_message: format!("The data is not stored."),
                };
                HttpResponse::Ok().json(return_data)
            }
        }
        Err(e) => {
            let mock_data = AiReturn {
                id: 0,
                username: format!(""),
                question: format!(""),
                answer: format!(""),
            };
            let mock_vector = vec![mock_data];
            let return_data = ReturnAiReadOne {
                data: mock_vector,
                success: false,
                message: format!("{e:?}"),
                add_message: format!("Failed to read the data"),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/ai_delete")]
pub async fn ai_delete(data: Json<AiReadOne>) -> impl Responder {
    let username = data.username.clone();
    if username.length() > Some(0) {
        let one_result = delete_ai(username);
        match one_result {
            Ok(one_data) => {
                let return_data = ReturnAiReadOne {
                    data: one_data,
                    success: true,
                    message: format!("Successfully"),
                    add_message: format!("Data is deleted successfully successfully"),
                };
                HttpResponse::Ok().json(return_data)
            }
            Err(e) => {
                let mock_data = AiReturn {
                    id: 0,
                    username: format!(""),
                    question: format!(""),
                    answer: format!(""),
                };
                let mock_vector = vec![mock_data];
                let return_data = ReturnAiReadOne {
                    data: mock_vector,
                    success: false,
                    message: format!("{e:?}"),
                    add_message: format!("Failed to deleted the data"),
                };
                HttpResponse::Ok().json(return_data)
            }
        }
    } else {
        let mock_data = AiReturn {
            id: 0,
            username: format!(""),
            question: format!(""),
            answer: format!(""),
        };
        let mock_vector = vec![mock_data];

        let return_data = ReturnAiReadOne {
            data: mock_vector,
            success: false,
            message: format!("Username length less than the required length"),
            add_message: format!("Enter a valid username."),
        };
        HttpResponse::Ok().json(return_data)
    }
}

#[patch("/ai_username_update")]
pub async fn ai_username_update(data: Json<UsernameUpdateAi>) -> impl Responder {
    let username = data.username.clone();
    let data = UsernameUpdateAi {
        username: username.clone(),
        new_value: data.new_value.clone(),
    };

    let one_result = update_username_ai(data.username, data.new_value);
    match one_result {
        Ok(created_data) => {
            let return_data = ReturnAi {
                username: created_data.username,
                question: created_data.question,
                answer: created_data.answer,
                success: true,
                message: format!(""),
                add_message: format!("Data is created successfully"),
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = ReturnAi {
                username: format!(""),
                question: format!("",),
                answer: format!("",),
                success: false,
                message: format!("{e:?}"),
                add_message: format!("Failed to update username: {}", username.clone()),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[patch("/ai_question_update")]
pub async fn ai_question_update(data: Json<QAUpdateAi>) -> impl Responder {
    let one_result = update_question_ai(
        data.username.clone(),
        data.old_value.clone(),
        data.new_value.clone(),
    );
    match one_result {
        Ok(created_data) => {
            let return_data = ReturnAi {
                username: created_data.username,
                question: created_data.question,
                answer: created_data.answer,
                success: true,
                message: format!(""),
                add_message: format!("Question is updated successfully"),
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = ReturnAi {
                username: format!(""),
                question: format!("",),
                answer: format!("",),
                success: false,
                message: format!("{e:?}"),
                add_message: format!("Error while saving"),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[patch("/ai_answer_update")]
pub async fn ai_answer_update(data: Json<QAUpdateAi>) -> impl Responder {
    let one_result = update_answer_ai(
        data.username.clone(),
        data.old_value.clone(),
        data.new_value.clone(),
    );
    match one_result {
        Ok(created_data) => {
            let return_data = ReturnAi {
                username: created_data.username,
                question: created_data.question,
                answer: created_data.answer,
                success: true,
                message: format!(""),
                add_message: format!("Answer is updated successfully"),
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(e) => {
            let return_data = ReturnAi {
                username: format!(""),
                question: format!("",),
                answer: format!("",),
                success: false,
                message: format!("{e:?}"),
                add_message: format!("Failed to update the answer."),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/upload_audio")]
async fn upload_audio(mut payload: Multipart) -> impl Responder {
    let mut audio_data: Vec<u8> = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Some(chunk) = field.next().await {
            match chunk {
                Ok(data) => audio_data.extend_from_slice(&data),
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ReturnAudioData {
                        success: false,
                        message: format!("Error reading chunk: {}", e),
                    });
                }
            }
        }
    }

    // Convert directly to WAV format in memory
    let mut wav_data = Vec::new();
    if let Err(e) = convert_to_wav_in_memory(audio_data, &mut wav_data) {
        return HttpResponse::InternalServerError().json(ReturnAudioData {
            success: false,
            message: format!("Failed to convert audio: {}", e),
        });
    }

    // Transcribe the audio data in memory
    match transcribe_audio_from_memory(&wav_data) {
        Ok(transcription) => HttpResponse::Ok().json(ReturnAudioData {
            success: true,
            message: transcription,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ReturnAudioData {
            success: false,
            message: format!("Transcription failed: {}", e),
        }),
    }
}

#[post("/transcribe")]
async fn transcribe_audio(payload: Multipart) -> impl Responder {
    match save_audio(payload).await {
        Ok(file_path) => {
            println!("File is saved at: {}", file_path);
            let file_result = wav_convert_file();
            match file_result {
                Ok(path_of_file) => {
                    println!("File is converted at: {}", path_of_file);
                    match whisper_transcribe_medium2() {
                        Ok(transcription) => {
                            let data_returned: ReturnAudioData = ReturnAudioData {
                                success: true,
                                message: transcription,
                            };
                            HttpResponse::Ok().json(data_returned)
                        }
                        Err(e) => {
                            let data_returned = ReturnAudioData {
                                success: false,
                                message: format!("Transcription failed: {}", e),
                            };
                            HttpResponse::InternalServerError().json(data_returned)
                        }
                    }
                }
                Err(e) => {
                    let data_returned = ReturnAudioData {
                        success: false,
                        message: format!("Convertion failed: {}", e),
                    };
                    HttpResponse::InternalServerError().json(data_returned)
                }
            }
        }
        Err(e) => {
            let data_returned = ReturnAudioData {
                success: false,
                message: format!("File upload failed: {}", e),
            };
            HttpResponse::InternalServerError().json(data_returned)
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    match whisper_transcribe_medium() {
        Ok(text) => {
            println!("Transcription: {}", text)
        }
        Err(e) => {
            eprintln!("Failed to transcribe audio, error code: {}", e)
        }
    }
    println!("Done Transcribing");
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:4200")
                    .allowed_origin("http://192.168.137.68:4200")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(sign_up_user)
            .service(user_read_one)
            .service(login_user)
            .service(update_user_password)
            .service(update_email_or_username)
            .service(delete_user)
            .service(goal_update)
            .service(clear_goals)
            .service(goal_read_one)
            .service(goal_create)
            .service(delete_one_goal)
            .service(done_goal_create)
            .service(done_goal_read_one)
            .service(done_goal_update)
            .service(done_goal_delete)
            .service(done_goal_delete_all)
            .service(day_current)
            .service(month_current)
            .service(year_current)
            .service(fav_day_create)
            .service(fav_day_read_one)
            .service(fav_day_read_all)
            .service(fav_day_update)
            .service(fav_day_delete)
            .service(check_done_exists)
            .service(check_todo_exists)
            .service(diary_create)
            .service(diary_read_one)
            .service(diary_delete)
            .service(diary_udpate)
            .service(check_if_user_exists)
            .service(date_actual)
            .service(ai_answer_update)
            .service(ai_question_update)
            .service(ai_username_update)
            .service(ai_delete)
            .service(ai_read_one)
            .service(ai_create)
            .service(upload_audio)
            .service(transcribe_audio)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
