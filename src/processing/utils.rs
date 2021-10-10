use crate::models;
use models::ability_map::AbilityMap;
use models::state::NeighborMap;
use serde_json;

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
fn build_neighbor_map(neighbors: NeighborMap) -> () {}
