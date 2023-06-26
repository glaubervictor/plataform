use actix_threadpool::run;
use actix_web::web::{Data, Json};

use crate::{
    database::PostgresPool,
    error::ApiError,
    models::user::{NewUser, User},
    repositories::user_repository::UserRepository,
    validate::validate,
};

pub async fn users_get_all(data: Data<PostgresPool>) -> Result<Json<Vec<User>>, ApiError> {
    let mut user_repository = UserRepository::new(data);
    let users = run(move || user_repository.get_all()).await?;
    Ok(Json(users))
}

pub async fn users_create(
    data: Data<PostgresPool>,
    params: Json<NewUser>,
) -> Result<Json<User>, ApiError> {
    validate(&params)?;

    let new_user = NewUser {
        email: params.email.to_string(),
        password: params.password.to_string(),
    };

    let mut user_repository = UserRepository::new(data);
    let user = run(move || user_repository.create(&new_user)).await?;

    Ok(Json(user))
}
