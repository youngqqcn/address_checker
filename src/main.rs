use std::sync::Arc;

use dotenv::dotenv;
use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Http, Middleware, Provider},
    types::U256,
};
use eyre::Result;
use sqlx::{mysql::MySqlPoolOptions, FromRow, MySql};

// #[derive(FromRow, Debug, Clone)]
#[derive(FromRow, Debug, PartialEq, Eq, Clone)]
struct AddressBalance {
    id: i32,
    chain: String,
    token: String,
    addr: String,
    checked: bool,
    balance: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    dotenv().ok();

    connect_mysql().await?;

    // request_rpc().await?;

    // request_rpc_bsc().await?;

    // let ret = get_eth_balance(
    //     "0xE2bcF8373f6a6BD14189c7D4C5dBE7BE8838937e"
    //         .parse::<Address>()
    //         .unwrap(),
    // )
    // .await?;
    // println!("ret = {}", ret);

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

    // TODO: 处理空的情况
    let row = sqlx::query_as::<_, AddressBalance>(
        "SELECT * from  address_balance where checked = 0 LIMIT 1",
    )
    .bind(1)
    .fetch_optional(&pool)
    .await
    .unwrap();

    match row.clone() {
        Some(row) => {
            println!("Found row: {:?}", row);
            // 处理查询到的结果
        }
        None => {
            println!("No rows found.");
            // 处理查询结果为空的情况
            return Ok(());
        }
    };

    println!("{:?}", row);

    // let inserted_row =
    //     sqlx::query("INSERT INTO address_balance(chain, token, addr) VALUES(?, ?, ?)")
    //         .bind("BSC")
    //         .bind("BNB")
    //         .bind("0xE2bcF8373f6a6BD14189c7D4C5dBE7BE8838937e")
    //         .execute(&pool)
    //         .await
    //         .unwrap();
    // println!("inserted row: {:?}", inserted_row);


    let updated_row = sqlx::query("UPDATE address_balance SET balance=? , checked=1 WHERE id =?")
        .bind("123")
        .bind(row.unwrap().id.clone())
        .execute(&pool)
        .await
        .unwrap();

    println!("updated row: {:?}", updated_row);

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

async fn get_eth_balance(address: Address) -> Result<U256, Box<dyn std::error::Error>> {
    // 请求rpc

    let rpc_url = std::env::var(format!("BSC_RPC_URL")).unwrap();
    let client = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(client);

    let balance = client.get_balance(address, None).await?;

    println!("balance is {}", balance);
    Ok(balance)
}

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    IERC20,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function allowance(address owner, address spender) external view returns (uint256)
    ]"#,
);

async fn get_erc20_token_balance(
    account: Address,
    contract: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    // 请求rpc
    let rpc_url = std::env::var(format!("BSC_RPC_URL")).unwrap();
    let client = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(client);

    let erc20_token = IERC20::new(contract, Arc::clone(&client));

    // getReserves -> get_reserves
    let balance = erc20_token.balance_of(account).call().await?;
    println!("balance is {balance}");

    Ok(balance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    fn setup() {
        dotenv().ok();
    }

    #[test]
    async fn test_get_eth_balance() {
        setup();

        let account = "0xE2bcF8373f6a6BD14189c7D4C5dBE7BE8838937e"
            .parse::<Address>()
            .unwrap();

        let x = get_eth_balance(account).await.unwrap();
        assert_eq!(x.gt(&U256::from(0)), true);
    }

    #[test]
    async fn test_get_erc20_token_balance() {
        setup();

        let account = "0xE2bcF8373f6a6BD14189c7D4C5dBE7BE8838937e"
            .parse::<Address>()
            .unwrap();
        let contract = "0x55d398326f99059fF775485246999027B3197955"
            .parse::<Address>()
            .unwrap();
        let x = get_erc20_token_balance(account, contract).await.unwrap();
        assert_eq!(x.gt(&U256::from(0)), true);
    }
}
