use alloy_primitives::{Address, address};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Token {
    /// The contract address (The unique ID)
    pub address: Address,

    /// Human readable symbol (e.g., "WETH") - Optional, used for logging
    pub symbol: String,

    /// Decimals (e.g., 18) - Critical for normalizing amounts for display
    pub decimals: u8,

    /// Optimization: Cached flags to avoid repeated address checks
    pub is_weth: bool,
    pub is_native: bool, // For V4 which might support native ETH
}

/// For an ultra-optimized version, use a fixed-size byte array for the symbol
#[derive(Clone, Copy)]
pub struct TinyToken {
    pub address: Address,
    pub symbol: [u8; 8], // Store "WETH" as bytes, truncate if longer
    pub decimals: u8,
}

impl Token {
    /// Create a new token and auto-detect common flags
    pub fn new(address: Address, symbol: String, decimals: u8) -> Self {
        let is_weth = address == address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"); // Mainnet WETH
        let is_native = address == Address::ZERO; // V4 Convention for Native ETH

        Self {
            address,
            symbol,
            decimals,
            is_weth,
            is_native,
        }
    }

    /// Helper to create a dummy token for testing or unitialized nodes
    pub fn empty() -> Self {
        Self::new(Address::ZERO, "UNK".to_string(), 18)
    }
}

// --- Trait Implementations for HashMaps & Graph uniqueness ---

// Two tokens are equal if their addresses are equal.
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Eq for Token {}

// Hashing should only depend on the address.
impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

// Pretty printing for logs: "WETH(0xC02...)"
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.symbol, self.address)
    }
}

// Display just the symbol for cleaner graphviz exports
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
