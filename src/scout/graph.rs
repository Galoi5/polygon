use alloy_primitives::Address;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use utils::models::{PoolEdge, TokenNode};

pub struct ArbitrageGraph {
    pub graph: DiGraph<TokenNode, PoolEdge>,
    pub token_to_node: HashMap<Address, NodeIndex>,
}

impl ArbitrageGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            token_to_node: HashMap::new(),
        }
    }

    /// Adds or retrieves a token node
    pub fn add_token(&mut self, token: TokenNode) -> NodeIndex {
        *self
            .token_to_node
            .entry(token.address)
            .or_insert_with(|| self.graph.add_node(token.clone()))
    }

    /// Adds a bidirectional edge for a Uniswap V3 pool
    pub fn add_pool(&mut self, pool: PoolEdge, token_in: Address, token_out: Address) {
        let u = self.token_to_node.get(&token_in).expect("Node in exists");
        let v = self.token_to_node.get(&token_out).expect("Node out exists");

        // Add the directed edge
        self.graph.add_edge(*u, *v, pool);
    }
}
