use actix_web::web::Data;
use diesel::RunQueryDsl;

use crate::{database::PostgresPool, error::ApiError, models::user::{User, NewUser}};

pub struct UserRepository {
    pool: Data<PostgresPool>,
}

impl UserRepository {
    pub fn new(pool: Data<PostgresPool>) -> Self {
        Self { pool }
    }

    pub fn get_all(&mut self) -> Result<Vec<User>, ApiError> {
        use crate::schema::users::dsl::users;
        let mut conn = self.pool.get()?;
        let all_users = users.load::<User>(&mut conn)?;

        Ok(all_users.into())
    }

    pub fn create(&mut self, new_user: &NewUser) -> Result<User, ApiError> {
        use crate::schema::users::dsl::users;

        let user: User = new_user.clone().into();
        let mut conn = self.pool.get()?;

        diesel::insert_into(users).values(&user).execute(&mut conn)?;
        Ok(user.into())
    }
}
