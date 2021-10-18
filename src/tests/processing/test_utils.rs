#[test]
fn ability_map_values() {
    use crate::{models::ability_map::AbilityMap, processing::utils::get_ability_map};

    let map: AbilityMap = get_ability_map();
    assert_eq!(map["Vector"]["Green"], 5);
}