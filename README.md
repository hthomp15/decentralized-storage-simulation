
# Distributed Storage Network Simulation

This project implements a basic simulation of a distributed storage network inspired by systems like IPFS (InterPlanetary File System). The goal is to demonstrate how data can be stored and retrieved across a network of nodes, with replication for fault tolerance and dynamic handling of node failures.

## Features

- **Content-Addressable Storage**: Data is stored using a unique Content Identifier (CID) derived from the data itself using a hash function.
- **Dynamic Replication**: Data is replicated across multiple nodes to ensure availability, with a replication factor dynamically determined based on the number of active nodes.
- **Fault Tolerance**: Simulates node failures and ensures data can still be retrieved from remaining nodes.
- **Automatic Fallback**: If a node does not contain the requested data, the network searches other nodes to locate it.

## How It Works

1. **Nodes**: Each node in the network has its own storage, represented as a `HashMap` of CIDs to data.
2. **Network**: A central controller manages the network of nodes, including adding nodes, storing data, and retrieving data.
3. **Replication**: When storing data, the network replicates it across a subset of nodes based on a calculated replication factor.
4. **Data Retrieval**:
   - If no node is specified, the network searches all nodes to retrieve the data.
   - If the specified node does not contain the data, the network searches other nodes as a fallback.
5. **Node Failure Simulation**: Nodes can be removed from the network to simulate failures, testing the fault tolerance of the system.

## Code Overview

### `Node`
A node represents an individual storage unit in the network.
- `store_data(data: &str) -> CID`: Stores data on the node and generates a unique CID.
- `retrieve_data(cid: &CID) -> Option<&String>`: Retrieves data based on a CID.

### `Network`
The network coordinates the nodes and manages data replication and retrieval.
- `add_node(node_id: &str)`: Adds a new node to the network.
- `store_data(data: &str) -> CID`: Stores data on multiple nodes with replication.
- `retrieve_data(cid: &CID) -> Option<String>`: Searches nodes for the data using the CID.
- `simulate_node_failure(node_id: &str)`: Removes a node from the network to simulate failure.
- `calculate_replication_factor() -> usize`: Dynamically determines the replication factor based on the number of nodes.

### `main`
The entry point demonstrates the system by:
1. Adding nodes to the network.
2. Storing data with replication.
3. Retrieving data before and after simulating a node failure.

## Usage

1. Clone the repository and navigate to the project directory:
   ```bash
   git clone https://github.com/hthomp15/decentralized-storage-simulation.git
   cd decentralized-storage-simulation
   ```

2. Build and run the project:
   ```bash
   cargo run
   ```
   Observe the output demonstrating storage, retrieval, and fault tolerance.

## Example Output

```vbnet
Dynamic replication factor calculated: 1 (based on 3 nodes)
Data stored with CID: c845422db41e5c8d
Data found on node: node1
Retrieved data: PLDG FTW!!!!
Node 'node1' has been removed (simulated failure)
Data found on node: node2
Retrieved data after failure: PLDG FTW!!!!
```

## Inspiration

This project is inspired by the architecture of decentralized storage networks like IPFS and Filecoin, which rely on distributed systems to ensure data durability, fault tolerance, and accessibility.
