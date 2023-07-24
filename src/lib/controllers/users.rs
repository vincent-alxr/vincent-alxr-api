use std::convert::Infallible;

use diesel::sql_types::Integer;
use rocket::request::{FromRequest, Outcome, self};
use rocket::{get, post, put, delete, Error, Request, async_trait};
use rocket::form::{Form,FromForm};
use rocket::http::{Header, Status};
use crate::establish_connection;
use crate::lib::models::{User, UserHasSkill, UserWithSkills, LevelEnum};
use diesel::{prelude::*, sql_query};
use rocket::serde::json::Json;

#[derive(FromForm)]
pub struct UserForm {
    pub name: String,
    pub email: String,
}

#[derive(FromForm)]
pub struct AddSkillForm {
    pub title: String,
    pub level: LevelEnum,
}

// Route principale pour l'endpoint '/users'
#[get("/")]
pub fn get_users() -> Json<Vec<User>> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let result = users.select(User::as_select()).load(connection).expect("Error loading users");
    Json(result)
}



#[get("/<user_id>")]
pub fn get_user_by_id(user_id: i32) -> Result<Json<UserWithSkills>, String> {
    use crate::schema::users::dsl::users;
    use crate::schema::users_has_skills::dsl::users_has_skills;
    
    let connection = &mut establish_connection();
    let user = users.find(user_id).select(User::as_select()).first(connection).optional();
    

    match user {
        Ok(Some(user)) => {
            let skills = sql_query("SELECT * FROM users_has_skills WHERE user_id = $1")
            .bind::<Integer,_>(user.id)
            .get_results::<UserHasSkill>(connection);
            
            match skills {
                Ok(skills) => {
                    let user_with_skills = UserWithSkills { user, skills };
                    Ok(Json(user_with_skills))
                },
                Err(err) => Err(err.to_string())
            }
        },
        Ok(None) => Err("Can not find the user".to_string()),
        Err(_) => Err(format!("An error occured while fetching user {}", user_id)),
    }
}


#[derive(Debug)]
pub enum TokenError {
    MissingToken,
    InvalidToken,
}

pub struct Token(String);

#[async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = TokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(bearer) => {
                let token_without_bearer: Vec<&str> = bearer.split(" ").collect();
                // check validity
                if token_without_bearer.len() >= 2 {
                    Outcome::Success(Token(token_without_bearer[1].to_string()))
                } else {
                    rocket::outcome::Outcome::Failure((Status::BadRequest, TokenError::InvalidToken))
                }
            },
            // token does not exist
            None => rocket::outcome::Outcome::Failure((Status::Unauthorized, TokenError::MissingToken)),
        }
    }
}

// Route pour l'endpoint '/users' qui reçoit des données PUT (simulées ici avec un formulaire)
#[put("/skills/add", data = "<add_skill_form>")]
pub fn update_user(add_skill_form: Form<AddSkillForm>, token: Token) -> String {
    
    format!(
        "Mise à jour de l'utilisateur avec l'ID {} : {}",
        token.0, add_skill_form.title
    )

}

// Route pour l'endpoint '/users' qui reçoit des données DELETE (simulées ici avec un formulaire)
#[delete("/<id>")]
pub fn delete_user(id: u32) -> String {
    format!("Suppression de l'utilisateur avec l'ID {}", id)
}