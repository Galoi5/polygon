use crate::common::pool::PoolVariant;
use crate::common::token::Token;
use petgraph::graph::DiGraph;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub pool: PoolVariant,
    pub zero_for_one: bool, // Direction of the trade on this edge
}

impl GraphEdge {
    pub fn new(pool: PoolVariant, zero_for_one: bool) -> Self {
        Self { pool, zero_for_one }
    }

    /// Fast access to weight for SPFA
    pub fn weight(&self) -> f64 {
        self.pool.get_log_weight(self.zero_for_one)
    }
}

/// The concrete Graph type for our Arbitrage Bot
pub type ArbGraph = DiGraph<Token, GraphEdge>;

/// A lookup table to quickly find NodeIndices by Address
pub struct GraphManager {
    pub graph: ArbGraph,
    pub node_map: HashMap<Address, petgraph::graph::NodeIndex>,
}

impl GraphManager {
    pub fn new() -> Self {
        Self {
            graph: ArbGraph::new(),
            node_map: HashMap::new(),
        }
    }

    /// Adds a token if it doesn't exist, returns its index
    pub fn add_or_get_token(&mut self, token: Token) -> petgraph::graph::NodeIndex {
        if let Some(&index) = self.node_map.get(&token.address) {
            return index;
        }

        // Clone address before moving token into graph
        let addr = token.address;
        let index = self.graph.add_node(token);
        self.node_map.insert(addr, index);
        index
    }
}
