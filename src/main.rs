use actix_cors::Cors;
use actix_web::{
    delete, get, http, patch, post, web::Json, App, HttpResponse, HttpServer, Responder,
};
use day_of_week::{
    date::date, day_month::current_day, day_time::current_month, day_year::current_year,
};
use diary::{
    create_diary::create_diary,
    delete_diary::delete_diary,
    read_diary::{read_all_diary_content, read_one_diary_content},
    update_diary::update_diary,
};
use favourite_day::{
    create_favourite_day::create_favourite_day, delete_favourite_day::delete_favourite_day,
    read_all_favourite_day::read_all_favourite_day, read_one_favourite_day::read_one_favourite_day,
    update_favourite_day::update_favourite_day,
};
use goals::{
    create_goal::create_goal,
    delete_goal::{delete_all_goals, delete_goal},
    goals_done::{
        create_done_goal::create_done_goal,
        delete_done_goal::{delete_all_done_goals, delete_done_goal},
        read_done_goal::{read_all_done_goal, read_one_done_goal},
        update_done_goal::update_done_goal,
    },
    read_goal::{read_all_goal, read_one_goal},
    update_goal::update_goal,
};
use models::{
    ChatUsers, CheckIfGoalExists, CurrentDay, CurrentMonth, CurrentYear, DeleteUserPassword, Diary,
    DiaryExists, DiaryReturn, ErrorReturn, FavouriteDay, FavouriteDayReadAllReturn,
    FavouriteDayReadOne, FavouriteDayReturn, FavouriteDayUpdate, Goal, GoalDone, GoalUpdateReturn,
    IsSuccessful, LoginChatUsers, MessageResponse, MyDate, SearchGoal, SuccessReadOne,
    SuccessReturn, UpdateGoal, UpdateUserPassword, UpdateUsernameOrEmail,
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

    println!("\n\n Done goals: \n");
    let done_goals = read_all_done_goal();
    match done_goals {
        Ok(done_goals_vec) => println!("{:?}", done_goals_vec),
        Err(e) => println!("{e:?}"),
    }
    let all_fav_days = read_all_favourite_day();
    match all_fav_days {
        Ok(all_data) => {
            println!("All Favourite days: \n");
            println!("\n{all_data:?}")
        }
        Err(e) => println!("{e:?}"),
    }
    let all_diary = read_all_diary_content();
    match all_diary {
        Ok(all_detail) => {
            println!("\n\nAll data in diary");
            for x in all_detail {
                println!("{:?}\n", x);
            }
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
                    .allowed_origin("http://192.168.137.68:4200")
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
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
