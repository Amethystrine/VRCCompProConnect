extern crate postgres;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Member{
    id:i32,
    vrc_name:String,
    discord_name:String,
    atcoder_id:String,
    regist:String,
}

#[tokio::main]
async fn main()  -> anyhow::Result<()>{
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    //let conn = PgConnection::connect(&db_url).await?;
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;
     let query = sqlx::query("CREATE TABLE IF NOT EXISTS member (
         id              SERIAL PRIMARY KEY NOT NULL,
         vrc_name        VARCHAR NOT NULL,
         discord_name    VARCHAR NOT NULL,
         atcoder_id      VARCHAR NOT NULL,
         regist          VARCHAR NOT NULL
       )");
    //let query = sqlx::query("INSERT INTO member (vrc_name,discord_name,atcoder_id,regist) VALUES ('あめんばー','あめとりんばー','amentorimaru','2023-07-20')");
    //let query = sqlx::query("DROP TABLE member");
    query.execute(&pool).await?;
    //let users:Vec<Member> = sqlx::query_as(r"select * from member").fetch_all(&pool).await?;
    //println!("{:?}",users);    
    println!("done");
    Ok(())

}