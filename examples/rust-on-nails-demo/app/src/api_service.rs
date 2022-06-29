use crate::api::*;
use crate::errors::CustomError;
use crate::queries;
use deadpool_postgres::Pool;
use tonic::{Request, Response, Status};

pub struct FortunesService {
    pub pool: Pool,
}

#[tonic::async_trait]
impl crate::api::fortunes_server::Fortunes for FortunesService {
    async fn get_fortunes(
        &self,
        _request: Request<GetFortunesRequest>,
    ) -> Result<Response<GetFortunesResponse>, Status> {
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| CustomError::Database(e.to_string()))?;

        let fortunes = queries::fortunes::fortunes(&client)
            .await
            .map_err(|e| CustomError::Database(e.to_string()))?;

        // Map the structs we get from cornucopia to the structs
        // we need for our gRPC reply.
        let fortunes = fortunes
            .into_iter()
            .map(|fortune| Fortune {
                id: fortune.id as u32,
                message: fortune.message,
            })
            .collect();

        let response = GetFortunesResponse {
            fortunes,
        };

        return Ok(Response::new(response));
    }
}
