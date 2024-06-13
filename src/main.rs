use std::sync::Arc;

use dotenv::dotenv;
use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Http, Middleware, Provider},
};
use eyre::Result;
use sqlx::{mysql::MySqlPoolOptions, FromRow, MySql};

// #[derive(FromRow, Debug, Clone)]
#[derive(FromRow, Debug, PartialEq, Eq)]
struct Person {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    dotenv().ok();

    // connect_mysql().await?;

    // request_rpc().await?;

    request_rpc_bsc().await?;

    Ok(())
}

async fn connect_mysql() -> Result<(), sqlx::Error> {
    let database_url = std::env::var(format!("DATABASE_URL")).unwrap();
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

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    IUniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#,
);

async fn request_rpc() -> Result<(), Box<dyn std::error::Error>> {
    // 请求rpc

    let client = Provider::<Http>::try_from("https://eth.llamarpc.com")?;
    let client = Arc::new(client);

    // ETH/USDT pair on Uniswap V2
    let address = "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852".parse::<Address>()?;
    let pair = IUniswapV2Pair::new(address, Arc::clone(&client));

    // getReserves -> get_reserves
    let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await?;
    println!("Reserves (ETH, USDT): ({reserve0}, {reserve1})");

    let mid_price = f64::powi(10.0, 18 - 6) * reserve1 as f64 / reserve0 as f64;
    println!("ETH/USDT price: {mid_price:.2}");

    Ok(())
}

async fn request_rpc_bsc() -> Result<(), Box<dyn std::error::Error>> {
    // 请求rpc

    let rpc_url = std::env::var(format!("BSC_RPC_URL")).unwrap();
    let client = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(client);

    let balance = client
        .get_balance("0xE2bcF8373f6a6BD14189c7D4C5dBE7BE8838937e", None)
        .await?;

    println!("balance is {}", balance);
    Ok(())
}
