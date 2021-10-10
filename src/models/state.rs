use serde::Deserialize;

// These types are helpful for making code cleaner and more readable,
// especially in other files that reference them
pub type Desk = Vec<Pearl>;
pub type Layers = Vec<Layer>;
pub type NeighborMap = Vec<Vec<u32>>;
pub type Workers = Vec<Worker>;

#[derive(Deserialize)]
pub struct Layer {
    pub color: String,
    pub thickness: u32,
}

#[derive(Deserialize)]
pub struct Pearl {
    pub id: u64,
    pub layers: Layers,
}

#[derive(Deserialize)]
pub struct Worker {
    pub desk: Desk,
    pub flavor: String,
    pub id: u32,
}

#[derive(Deserialize)]
pub struct State {
    pub workers: Workers,
    pub neighbor_map: NeighborMap,
    pub score: u32,
}
