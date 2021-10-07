use serde::Deserialize;

#[derive(Deserialize)]
pub struct Layer {
    pub color: String,
    pub thickness: i32
}

#[derive(Deserialize)]
pub struct Pearl {
    pub id: i64,
    pub layers: Vec<Layer>
}

#[derive(Deserialize)]
pub struct Worker {
    pub desk: Vec<Pearl>,
    pub flavor: String,
    pub id: i32
}

#[derive(Deserialize)]
pub struct State {
    pub workers: Vec<Worker>,
    pub neighbor_map: Vec<Vec<i32>>,
    pub score: i32
}