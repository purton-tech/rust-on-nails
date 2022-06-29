use crate::queries;
use axum::{extract::Extension, Json};
use deadpool_postgres::Pool;
use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};
use crate::errors::CustomError;


use serde::Serialize;

#[derive(Serialize)]
pub struct World {
    id: i32,
    random_number: i32
}

pub async fn db(
    Extension(pool): Extension<Pool>,
) -> Result<Json<World>, CustomError> {

    // Get a random id
    let mut rng = SmallRng::from_rng(&mut thread_rng()).unwrap();
    let random_id = (rng.gen::<u32>() % 10_000 + 1) as i32;

    // Get a client from the pool
    let client = pool.get().await?;

    let world = queries::worlds::world(&client, &random_id).await?;

    let world = World {
        id: world.id,
        random_number: world.message  
    };

    Ok(Json(world))
}
