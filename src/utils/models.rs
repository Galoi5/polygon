use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]

pub struct Token {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u64,
}

impl Token {
    pub fn new(id: String, symbol: String, name: String, decimals: u64) -> Self {
        Self {
            id,
            symbol,
            name,
            decimals,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pool {
    pub id: String,
    pub fee: u8, // in basis points
    pub token0: Token,
    pub token1: Token,
    pub sqrtPriceX96: u128,
}

impl Pool {
    pub fn new(id: String, fee: u8, token0: Token, token1: Token, sqrtPriceX96: u128) -> Self {
        Self {
            id,
            fee,
            token0,
            token1,
            sqrtPriceX96,
        }
    }
}