pub mod types {
            
} 
 pub mod queries { pub mod invitations {

use cornucopia_client::GenericClient;
use tokio_postgres::Error;


    pub async fn insert<T: GenericClient>(client:&T, organisation_id : &i32,email : &str,invitation_selector : &str,invitation_verifier_hash : &str) -> Result<(),Error> {let stmt = client.prepare("INSERT INTO
invitations (organisation_id, email, invitation_selector, invitation_verifier_hash)
VALUES($1, $2, $3, $4)
").await?;
let _ = client.execute(&stmt, &[&organisation_id,&email,&invitation_selector,&invitation_verifier_hash]).await?;

Ok(())}


    pub async fn get<T: GenericClient>(client:&T, invitation_selector : &str) -> Result<Option<(i32,i32,String,String,String,time::OffsetDateTime,time::OffsetDateTime)>,Error> {let stmt = client.prepare("SELECT
id,
organisation_id,
email,
invitation_selector,
invitation_verifier_hash,
created_at,
updated_at
FROM
invitations
WHERE
invitation_selector = $1
").await?;
let res = client.query_opt(&stmt, &[&invitation_selector]).await?;

let return_value = res.map(|res| { let return_value_0: i32 = res.get(0); let return_value_1: i32 = res.get(1); let return_value_2: String = res.get(2); let return_value_3: String = res.get(3); let return_value_4: String = res.get(4); let return_value_5: time::OffsetDateTime = res.get(5); let return_value_6: time::OffsetDateTime = res.get(6); (return_value_0,return_value_1,return_value_2,return_value_3,return_value_4,return_value_5,return_value_6) }); Ok(return_value)}


    pub async fn delete<T: GenericClient>(client:&T, email : &str,organisation_id : &i32) -> Result<(),Error> {let stmt = client.prepare("DELETE FROM
invitations
WHERE
email = $1
AND
organisation_id = $2
").await?;
let _ = client.execute(&stmt, &[&email,&organisation_id]).await?;

Ok(())}


    pub async fn get_all<T: GenericClient>(client:&T, organisation_id : &i32) -> Result<(i32,String,String,String,i32,time::OffsetDateTime,time::OffsetDateTime),Error> {let stmt = client.prepare("SELECT
id,
email,
invitation_selector,
invitation_verifier_hash,
organisation_id,
updated_at,
created_at
FROM
invitations
WHERE organisation_id = $1
").await?;
let res = client.query_one(&stmt, &[&organisation_id]).await?;

let return_value={ let return_value_0: i32 = res.get(0); let return_value_1: String = res.get(1); let return_value_2: String = res.get(2); let return_value_3: String = res.get(3); let return_value_4: i32 = res.get(4); let return_value_5: time::OffsetDateTime = res.get(5); let return_value_6: time::OffsetDateTime = res.get(6); (return_value_0,return_value_1,return_value_2,return_value_3,return_value_4,return_value_5,return_value_6) }; Ok(return_value)}
}

pub mod users {

use cornucopia_client::GenericClient;
use tokio_postgres::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct ExampleQuery {pub id : i32,pub email : String,pub reset_password_selector : Option<String>}
    pub async fn example_query<T: GenericClient>(client:&T, id : &i32) -> Result<Vec<super::super::queries::users::ExampleQuery>,Error> {let stmt = client.prepare("SELECT
id, email, reset_password_selector
FROM
users
WHERE
users.id < $1
").await?;
let res = client.query(&stmt, &[&id]).await?;

let return_value = res.iter().map(|res| { let return_value_0: i32 = res.get(0); let return_value_1: String = res.get(1); let return_value_2: Option<String> = res.get(2); super::super::queries::users::ExampleQuery { id : return_value_0,email : return_value_1,reset_password_selector : return_value_2 } }).collect::<Vec<super::super::queries::users::ExampleQuery>>(); Ok(return_value)}
} }