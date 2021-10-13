use crate::models;
use crate::models::action::ActionType;
use crate::models::state::{NeighborGraph, Pearl, State, Worker, WorkerPearlIDs};
use models::ability_map::AbilityMap;
use models::state::{NeighborMap, Workers};
use serde_json;
use std::collections::HashMap;
use models::action::{Nom, Pass};

/// Reads the data from the JSON file containing information about how the
/// different flavors of nautiloid are able to dissolve different layers
/// of pearls.
///
/// Also, this method ensures that the JSON data will be included in the
/// final binary created at compile time.
///
/// # Arguments
///
/// * None
///
/// # Returns
///
/// * `AbilityMap` - A data structure detailing how quickly each flavor
///                  of nautiloid can process each color of pearl
pub fn get_ability_map() -> AbilityMap {
    // This line will include the .json file in the compiled binary, allowing
    // it to run anywhere after compilation
    let json_data = include_str!("../static_files/ability_map.json");

    let map: AbilityMap = match serde_json::from_str(json_data) {
        Ok(m) => m,
        Err(e) => {
            panic!("File read error: {}", e)
        }
    };

    return map;
}

///
///
///
///
fn get_worker_ids(workers: &Workers) -> Vec<u32> {
    let mut worker_ids: Vec<u32> = Vec::new();

    for w in workers {
        worker_ids.push(w.id);
    }

    // Sort the IDs so that 0 (gate keeper's ID) will come first
    worker_ids.sort();
    return worker_ids;
}

/// This method creates a HashMap where the keys are teh worker nautiloid IDs
/// and the values are the current number of pearls on that worker's desk.
///
/// This method could be rewritten to use a vector instead, however that would
/// either assume that the worker IDs all come in sequential order, starting
/// at 0 and incrementing without skipping any natural numbers, or it would
/// have to use unnecessary space to accommodate unused IDs. For the sake of
/// robustness and minimizing space, a HashMap is used. We don't want our code
/// failing when a nautiloid quits mid shift...
pub fn get_worker_pearl_counts(workers: &Workers) -> HashMap<u32, u32> {
    let mut counts: HashMap<u32, u32> = HashMap::new();

    for w in workers {
        counts.insert(w.id, w.desk.len() as u32);
    }

    return counts;
}

/// This method will build a HashMap where the keys are the nautiloid IDs,
/// and the values are a Vec<u32> of the IDs of all the nautiloids to which
/// they are adjacent. This is intended to make looking up neighbors easier
/// when deciding to pass a pearl or not.
///
/// With this method, we can construct the map only one time, instead of
/// having to iterate over the entire neighbor map each time we want to
/// find adjacent nautiloids.
///
/// Runtime here is O(w * n), where w is the number of workers, and n is the
/// number of neighbor pairings, which could be quite large is teh nautiloids
/// are all able to reach one another. In that worse case scenerio, the runtime
/// becomes O(w * w!) which is a real nightmare. This whole strategy would be
/// more efficient as some sort of graph method.
///
/// # Arguments
///
/// * `neighbors` - This is the NeighborMap that is read directly from the
///               raw JSON file
///
/// # Returns
///
/// * `HashMap<u32, Vec<u32>>` - A HashMap where the keys are nautiloid IDs,
///                              and the values are vectors of the IDs of
///                              the neighbor nautiloids.
pub fn build_neighbor_graph(neighbors: &NeighborMap, workers: &Workers) -> NeighborGraph {
    let mut neighbor_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let worker_ids = get_worker_ids(workers);

    for id in worker_ids {
        let mut nb_list: Vec<u32> = Vec::new();

        for pair in neighbors {
            if id == pair[0] {
                nb_list.push(pair[1]);
            } else if id == pair[1] {
                nb_list.push(pair[0]);
            }
        }
        neighbor_map.insert(id, nb_list);
    }

    return neighbor_map;
}

///
pub fn make_pass(from_id: u32, pearl_id: u32, to_id: u32) -> ActionType {
    return ActionType::Pass(
        Pass{
            from_id: from_id,
            pearl_id: pearl_id,
            to_id: to_id
        }
    );
}

///
pub fn make_nom(nautiloid_id: u32, pearl_id: u32) -> ActionType {
    return ActionType::Nom(
        Nom{
            nautiloid_id: nautiloid_id,
            pearl_id: pearl_id
        }
    );
}

/// Determines how long it will take a given nautiloid to process a given pearl,
/// based on the information in a given ability map.
pub fn get_time_to_process(pearl: &Pearl, worker: &Worker, ability_map: &AbilityMap) -> u32 {
    let mut total_time: u32 = 0;

    for layer in &pearl.layers {
        let layer_time = (
            (layer.thickness as f32) / (ability_map[&worker.flavor][&layer.color] as f32)
        ).ceil() as u32;

        total_time += layer_time;
    }

    return total_time;
}

/// Given a worker, finds which of its neighbors have an empty desk.
fn get_empty_neighbors(
    worker: &Worker,
    pearl_counts: &HashMap<u32, u32>,
    neighbor_graph: &NeighborGraph
) -> Vec<u32> {
    let mut empty_nbrs = neighbor_graph.get(&worker.id).unwrap().clone();
    empty_nbrs.retain(|&i| pearl_counts[&i] == 0);

    return empty_nbrs;
}

// Given the state, and a particular worker, determine the best option
// for passing a pearl to a neighbor.
pub fn get_best_neighbor(
    state: &State,
    worker: &Worker,
    pearl_counts: &HashMap<u32, u32>,
    ability_map: &AbilityMap,
    neighbor_graph: &NeighborGraph
) -> WorkerPearlIDs {
    let empty_neighbors = get_empty_neighbors(worker, pearl_counts, neighbor_graph);

    let best_time = u32::MAX;
    let mut best_pair = WorkerPearlIDs{worker_id: 0, pearl_id: 0};

    for e_nbr_id in &empty_neighbors {
        for p in &worker.desk {
            let mut nbr = state.workers.clone();
            nbr.retain(|w| &w.id == e_nbr_id);

            let time = get_time_to_process(p, &nbr[0], ability_map);

            if time < best_time {
                best_pair.worker_id = *e_nbr_id;
                best_pair.pearl_id = p.id;
            }
        }
    }

    return best_pair;
}

///
///
pub fn get_best_pearl_to_nom(worker: &Worker, ability_map: &AbilityMap) -> u32 {
    let best_time = u32::MAX;
    let mut best_pearl_id = worker.desk[0].id;

    for pearl in &worker.desk {
        let time = get_time_to_process(pearl, worker, ability_map);

        if time < best_time {
            best_pearl_id = pearl.id;
        }
    }

    return best_pearl_id;
}