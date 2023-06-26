mod users_controller;

use actix_web::web;

use self::users_controller::{users_create, users_get_all};

pub fn app_configure(config: &mut web::ServiceConfig) {
    //config.service(resource("/users").route(get().to(get_users)));

    config.service(
        web::scope("/api/v1").service(
            web::scope("/users")
                //.route("/{id}", web::get().to(get_user))
                // .route("/{id}", web::put().to(update_user))
                // .route("/{id}", web::delete().to(delete_user))
                // .route("", web::get().to(get_users))
                .route("", web::get().to(users_get_all))
                .route("", web::post().to(users_create)),
        ),
    );
}
