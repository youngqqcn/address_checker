use sqlx::{mysql::MySqlPoolOptions, FromRow, MySql};
use dotenv::dotenv;

// #[derive(FromRow, Debug, Clone)]
#[derive(FromRow, Debug, PartialEq, Eq)]
struct Person {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Hello, world!");
    dotenv().ok();
    let database_url = std::env::var(format!("DATABASE_URL")).unwrap();

    // let database_url = "mysql://root:ae633jmFLiAGqigSO41@localhost:3306/fansland_sol";

    // mysql 数据库
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    // let mut conn = pool.acquire().await.unwrap();

    let row = sqlx::query_as::<_, Person>("SELECT * from  persons where id = ?")
        .bind(1)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("{:?}", row);

    let inserted_row = sqlx::query("INSERT INTO persons (name) VALUES(?)")
        .bind("xxx1")
        .execute(&pool)
        .await
        .unwrap();

    println!("inserted row: {:?}", inserted_row);

    Ok(())
}
