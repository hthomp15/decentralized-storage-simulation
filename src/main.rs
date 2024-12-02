use std::collections::HashMap;
use std::hash::{Hash, Hasher};

type CID = String;

struct Node {
    storage: HashMap<CID, String>,
}

impl Node {
    fn new() -> Self {
        Node {
            storage: HashMap::new(),
        }
    }

    fn store_data(&mut self, data: &str) -> CID {
        let cid = Self::generate_cid(data);
        if !self.storage.contains_key(&cid) {
            self.storage.insert(cid.clone(), data.to_string());
        }
        cid
    }

    fn retrieve_data(&self, cid: &CID) -> Option<&String> {
        self.storage.get(cid)
    }

    fn generate_cid(data: &str) -> CID {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

struct Network {
    nodes: HashMap<String, Node>,
}

impl Network {
    fn new() -> Self {
        Network {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, node_id: &str) {
        self.nodes.insert(node_id.to_string(), Node::new());
    }

    fn calculate_replication_factor(&self) -> usize {
        let total_nodes = self.nodes.len();

        // Ensure replication factor is at least 1 and at most the total number of nodes
        let replication_factor = std::cmp::max(1, total_nodes / 2);
        std::cmp::min(replication_factor, total_nodes)
    }

    fn store_data(&mut self, data: &str) -> CID {
        let replication_factor = self.calculate_replication_factor();
        println!(
            "Dynamic replication factor calculated: {} (based on {} nodes)",
            replication_factor, self.nodes.len()
        );

        let cid = Node::generate_cid(data);

        let mut replicated = 0;
        for node in self.nodes.values_mut() {
            if replicated >= replication_factor {
                break;
            }
            node.store_data(data);
            replicated += 1;
        }

        if replicated < replication_factor {
            println!(
                "Warning: Could not replicate to all nodes (replicated: {})",
                replicated
            );
        }
        cid
    }

    fn retrieve_data(&self, cid: &CID) -> Option<String> {
        for (node_id, node) in &self.nodes {
            if let Some(data) = node.retrieve_data(cid) {
                println!("Data found on node: {}", node_id);
                return Some(data.clone());
            }
        }
        println!("Data not found on any node");
        None
    }

    fn simulate_node_failure(&mut self, node_id: &str) {
        self.nodes.remove(node_id);
        println!("Node '{}' has been removed (simulated failure)", node_id);
    }
}

fn main() {
    let mut network = Network::new();

    // Add nodes
    network.add_node("node1");
    network.add_node("node2");
    network.add_node("node3");

    // Store data
    let data = "PLDG FTW!!!!";
    let cid = network.store_data(data);
    println!("Data stored with CID: {}", cid);

    // Retrieve data
    let retrieved_data = network.retrieve_data(&cid);
    match retrieved_data {
        Some(data) => println!("Retrieved data: {}", data),
        None => println!("Data not found in the network."),
    }

    // Simulate node failure
    network.simulate_node_failure("node1");

    // Retrieve data after failure
    let fallback_data = network.retrieve_data(&cid);
    match fallback_data {
        Some(data) => println!("Retrieved data after failure: {}", data),
        None => println!("Data unavailable across the network."),
    }
}
