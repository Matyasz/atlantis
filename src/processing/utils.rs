use crate::models;
use models::ability_map::AbilityMap;
use models::state::{NeighborMap, Workers};
use serde_json;
use std::collections::HashMap;

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
pub fn build_neighbor_map(neighbors: &NeighborMap, workers: &Workers) -> HashMap<u32, Vec<u32>> {
    let mut neighbor_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let worker_ids = get_worker_ids(workers);

    for id in worker_ids {
        let mut nb_list: Vec<u32> = Vec::new();

        for pair in neighbors {
            println!("{}", id);
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
