use std::collections::HashMap;

use serde::Deserialize;

// These types are helpful for making code cleaner and more readable,
// especially in other files that reference them
pub type Desk = Vec<Pearl>;
pub type Layers = Vec<Layer>;
pub type Workers = Vec<Worker>;
pub type NeighborMap = Vec<Vec<u32>>;
pub type NeighborGraph = HashMap<u32, Vec<u32>>;

#[derive(Debug)]
pub struct WorkerPearlIDs{
    pub worker_id: u32,
    pub pearl_id: u32
}

#[derive(Deserialize, Clone)]
pub struct Layer {
    pub color: String,
    pub thickness: u32,
}

#[derive(Deserialize, Clone)]
pub struct Pearl {
    pub id: u32,
    pub layers: Layers,
}

#[derive(Deserialize, Clone)]
pub struct Worker {
    pub desk: Desk,
    pub flavor: String,
    pub id: u32,
}

#[derive(Deserialize, Clone)]
pub struct State {
    pub workers: Workers,
    pub neighbor_map: NeighborMap,
    pub score: u32,
}
