use serde::Deserialize;

#[derive(Deserialize)]
pub struct Layer {
    pub color: String,
    pub thickness: u32,
}

#[derive(Deserialize)]
pub struct Pearl {
    pub id: u64,
    pub layers: Vec<Layer>,
}

#[derive(Deserialize)]
pub struct Worker {
    pub desk: Vec<Pearl>,
    pub flavor: String,
    pub id: u32,
}

pub type NeighborMap = Vec<Vec<u32>>;

#[derive(Deserialize)]
pub struct State {
    pub workers: Vec<Worker>,
    pub neighbor_map: NeighborMap,
    pub score: u32,
}
