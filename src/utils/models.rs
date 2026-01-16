use alloy_primitives::{Address, U256};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

/// Metadata for a Token (Node)
#[derive(Debug, Clone)]
pub struct TokenNode {
    pub address: Address,
    pub symbol: String,
    pub decimals: u8, // Decimals are described as a BigInt! in documentation but are almost always very small
}

/// Metadata for a Uniswap V3 Pool (Edge)
#[derive(Debug, Clone)]
pub struct PoolEdge {
    pub address: Address,
    pub fee: u32, // e.g., 3000 for 0.3%

    // --- SPFA Data ---
    // Log weight: -log(price_after_fee).
    // We use f64 for graph search; negative cycles = profit.
    pub weight: f64,

    // --- Newton-Raphson State ---
    pub liquidity: u128,      // Current active liquidity (L)
    pub sqrt_price_x96: U256, // current sqrtPriceX96
    pub tick: i32,            // current tick
    pub tick_spacing: i32,

    // Directionality relative to the graph edge
    // true if token0 -> token1, false if token1 -> token0
    pub zero_for_one: bool,
}
