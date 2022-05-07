use sqlx::{Error, PgPool};

pub struct User {
    pub id: i32,
    pub email: String,
}

impl User {
    pub async fn get_users(pool: &PgPool, user_id: u32) -> Result<Vec<User>, Error> {
        sqlx::query_as!(
            User,
            "
                SELECT 
                    id, email
                FROM 
                    users
                WHERE
                    id < $1
            ",
            user_id as i32
        )
        .fetch_all(pool)
        .await
    }
}
