use actix_web::{get,post,web, App, HttpServer, Responder,HttpResponse};
use serde::{Serialize, Deserialize};
extern crate postgres;
use sqlx::{postgres::PgPoolOptions, Postgres,Pool};
use std::env;
use dotenv::dotenv;
use serde_json::Value;
use tokio;


#[derive(Debug, Clone, PartialEq, sqlx::FromRow, Serialize, Deserialize)]
pub struct Member{
    id:Option<i32>,
    vrc_name:String,
    discord_name:String,
    atcoder_id:String,
    regist:String,
}


#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[post("/add_member")]
async fn add_member(data: web::Json<Member>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let _result = sqlx::query(r#"INSERT INTO member (vrc_name, discord_name, atcoder_id, regist) VALUES ($1, $2, $3, $4)"#)
        .bind(&data.vrc_name)
        .bind(&data.discord_name)
        .bind(&data.atcoder_id)
        .bind(&data.regist)
        .execute(pool.get_ref())
        .await;
    match _result{
        Ok(_pg)=>HttpResponse::Ok().body("Successfully added member"),
        Err(e)=>HttpResponse::InternalServerError().body(e.to_string()),
    }
    
}

#[get("/get_member")]
async fn get_member(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    match sqlx::query_as::<_, Member>("select * from member").fetch_all(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching members"),
    }
}


#[post("/create_transcript/{contest}")]
//async fn create_transcript(req: HttpRequest, contest: web::Path<String>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
async fn create_transcript(contest: web::Path<String>) -> impl Responder {
    // コンテストIDとコンテストタイムを取得する
    let url="https://kenkoooo.com/atcoder/resources/problems.json";
    //let response = reqwest::get(url).await?;
    ///let json: Value = response.json().await?;
    //println!("{:#?}",json);
    // 存在しないコンテストの場合は終了
    // コンテストにデータベースがあるか確認し、無い場合は作成する(最大八問分の)
    // 部員ごとにループ
    // コンテストタイム以降の提出を取得する（WriterTesterは知らん）
    // ACがある場合は全てのAC提出を放り込む
    // ACが無い場合は各種判定の最後の提出を
    
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await{
            Ok(pool) => pool,
            Err(_) => {
                return HttpResponse::Ok().json(users);
            }
        };

    match sqlx::query_as::<_, Member>("select * from member").fetch_all(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching members"),
    }
}

#[get("/init_member")]
async fn init_member(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    
    let res1 = sqlx::query_as::<_, Member>("truncate table member").fetch_all(pool.get_ref()).await;
    let res2 = sqlx::query_as::<_, Member>("INSERT INTO member (vrc_name,discord_name,atcoder_id,regist) VALUES ('あめんばー','あめとりんばー','amentorimaru','2023-07-20')").fetch_all(pool.get_ref()).await;
    let res3 = sqlx::query_as::<_, Member>("INSERT INTO member (vrc_name,discord_name,atcoder_id,regist) VALUES ('cleantted','cleantted(テッド)','cleantted','2023-07-21')").fetch_all(pool.get_ref()).await;
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await{
            Ok(pool) => pool,
            Err(_) => {
                return Ok(());
            }
        };


    HttpServer::new(move|| {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(index)
        .service(get_member)
        .service(add_member)
        .service(create_transcript)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}