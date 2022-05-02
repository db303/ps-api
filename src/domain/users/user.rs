use {
    crate::domain::users::{user_email::UserEmail, user_name::UserName},
    uuid::Uuid,
};

#[derive(Debug)]
pub struct User {
    pub user_id: Uuid,
    pub username: UserName,
    pub email: UserEmail,
}
