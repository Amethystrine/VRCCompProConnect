extern crate postgres;
use sqlx::postgres::{PgPoolOptions};
use std::env;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error>  {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    println!("111");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;
    println!("222");

     let row:(i64,)=sqlx::query_as("CREATE TABLE IF NOT EXISTS member (
                    id              SERIAL PRIMARY KEY,
                    vrc_name            VARCHAR NOT NULL,
                    discord_name         VARCHAR NOT NULL,
                    atcoder_id         VARCHAR NOT NULL,
                    regist             TIMESTAMP NOT NULL
                  )").bind(150_i64).fetch_one(&pool).await?;
    println!("done");
    Ok(())

}