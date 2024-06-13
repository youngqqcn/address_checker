use std::sync::Arc;

use dotenv::dotenv;
use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Http, Middleware, Provider},
    types::U256,
};
use eyre::Result;
use sqlx::{mysql::MySqlPoolOptions, FromRow};

// #[derive(FromRow, Debug, Clone)]
#[derive(FromRow, Debug, PartialEq, Eq, Clone)]
struct AddressBalance {
    id: i32,
    chain: String,
    token: String,
    addr: String,
    checked: bool,
    base_balance: String,
    usdt_balance: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    dotenv().ok();

    connect_mysql().await?;

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

    let result = sqlx::query_as::<_, AddressBalance>(
        "SELECT * from  address_balance where checked = 0 LIMIT 1",
    )
    .bind(1)
    .fetch_optional(&pool)
    .await
    .unwrap();

    let row = match result {
        Some(row) => row,
        None => {
            // 处理查询结果为空的情况
            println!("No rows found.");
            return Ok(());
        }
    };

    println!("{:?}", row);

    let rpc_url = std::env::var(format!("BSC_RPC_URL")).unwrap();
    let account = row.addr.parse::<Address>().unwrap();
    let base_balance = get_eth_balance(rpc_url.clone(), account).await.unwrap();
    let usdt_contract = "0x55d398326f99059fF775485246999027B3197955"
        .parse::<Address>()
        .unwrap();
    let usdt_balance = get_erc20_token_balance(rpc_url.clone(), account, usdt_contract)
        .await
        .unwrap();

    let updated_row = sqlx::query(
        "UPDATE address_balance SET base_balance=?, usdt_balance=?, checked=1 WHERE id =?",
    )
    .bind(base_balance.to_string())
    .bind(usdt_balance.to_string())
    .bind(row.id.clone())
    .execute(&pool)
    .await
    .unwrap();

    println!("updated row: {:?}", updated_row);

    Ok(())
}

async fn get_eth_balance(
    rpc_url: String,
    address: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    // 请求rpc

    // let rpc_url = std::env::var(format!("BSC_RPC_URL")).unwrap();
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
    rpc_url: String,
    account: Address,
    contract: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    // 请求rpc

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

        let rpc_url = std::env::var(format!("BSC_RPC_URL")).unwrap();
        let x = get_eth_balance(rpc_url, account).await.unwrap();
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
        let rpc_url = std::env::var(format!("BSC_RPC_URL")).unwrap();
        let x = get_erc20_token_balance(rpc_url, account, contract)
            .await
            .unwrap();
        assert_eq!(x.gt(&U256::from(0)), true);
    }
}
