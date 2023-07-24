
use diesel::sql_types::Text;
use jsonwebtoken::errors::Error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::{post, put};
use rocket::form::{Form,FromForm};
use crate::establish_connection;
use crate::lib::models::User;
use crate::lib::utils::hash_password;
use crate::schema::users;
use diesel::sql_query;
use diesel::prelude::*;
use rocket::serde::json::Json;
use jsonwebtoken::{encode, Header, EncodingKey, Validation, Algorithm, decode, DecodingKey};
use serde::{Serialize, Deserialize};


#[derive(FromForm)]
pub struct SigninForm {
    pub email: String,
    pub password: String,
}


#[derive(FromForm, Insertable)]
#[diesel(table_name = users)]
pub struct SignupForm {
    pub firstname: String,
    pub lastname: String,
    pub bio: String,
    pub password: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub bio: &'a str,
    pub password: &'a str,
    pub email: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    id: i32,
    email: String,
}

pub fn get_email_from_token(token: &str, secret_key: &str) -> Result<UserData, Error> {
    // Decode the JWT token using the secret key and the UserData struct as the claims type
    let validation = Validation::default();
    
    let decoded = decode::<UserData>(token, &DecodingKey::from_secret(secret_key.as_ref()), &validation)?;
    
    // Extract the UserData from the claims and return it
    Ok(decoded.claims)
}

#[post("/signin", data = "<signin_form>")]
pub fn signin(signin_form: Form<SigninForm>) -> Result<Json<String>, Custom<Json<String>>> {
    let connection = &mut establish_connection();

    // Clone the Arc for use inside the closure
    println!("{}", hash_password(signin_form.password.as_str()).as_str());

    let user = sql_query("SELECT * FROM users WHERE email = $1 AND password = crypt($2, 'password')")
    .bind::<Text, _>(signin_form.email.as_str())
    .bind::<Text, _>(signin_form.password.as_str())
    .get_result::<User>(connection);
    
    // Retrieve the user from the database
    match user {
        Ok(user) => {
            

            let user_data = UserData { id: user.id, email: user.email.to_string() };
            
            match encode(&Header::default(), &user_data, &EncodingKey::from_secret("secret".as_ref())) {
                Ok(token) => Ok(Json(token)), // Return the token as JSON in the response
                Err(e) => {
                    eprintln!("Error: {}", e);
                    Err(Custom(Status::InternalServerError, Json("Error creating JWT.".to_string())))
                }
            }
        }, // Return the user as JSON in the response
        Err(e) => {
            // Handle the error and return a JSON response with an appropriate HTTP status code
            eprintln!("Error: {}", e);
            Err(Custom(Status::InternalServerError, Json("Error retrieving user information.".to_string())))
        }
    }
}

// Route pour l'endpoint '/users' qui reçoit des données PUT (simulées ici avec un formulaire)
#[put("/signup", data = "<signup_form>")]
pub fn signup(signup_form: Form<SignupForm>) -> Result<Json<User>, Custom<Json<String>>> {
    use crate::schema::users::dsl::users;

    let connection = &mut establish_connection();

    let new_user = NewUser {
        firstname: &signup_form.firstname,
        lastname: &signup_form.lastname,
        bio: &signup_form.bio,
        email: &signup_form.email,
        password: &signup_form.password
    };

    let result = diesel::insert_into(users).values(new_user).returning(User::as_returning()).get_result(connection);

    match result {
        Ok(user) => Ok(Json(user)), // Return the user as JSON in the response
        Err(e) => {
            // Handle the error and return a JSON response with an appropriate HTTP status code
            eprintln!("Error: {}", e);
            Err(Custom(Status::InternalServerError, Json("Error trying add user.".to_string())))
        }
    }
}
