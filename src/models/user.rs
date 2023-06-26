use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct NewUser {
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        User {
            id: Uuid::new_v4(),
            email: user.email,
            password: user.password,
        }
    }
}
