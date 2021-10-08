
// use std::env::current_dir;
use std::fs::{File, canonicalize};
use std::io::BufReader;
use std::ops::Index;
use std::path::{Path, PathBuf};
// use std::ops::IndexMut;

use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Color {
    pub red: i32,
    pub green: i32,
    pub blue: i32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AbilityMap {
    pub general: Color,
    pub vector: Color,
    pub matrix: Color,
}

impl Index<&'_ str> for AbilityMap {
    type Output = Color;
    fn index(&self, s: &str) -> &Color {
        match s {
            "General" => &self.general,
            "Vector" => &self.vector,
            "Matrix" => &self.matrix,
            _ => panic!("Type AbilityMap does not have field {}", s),
        }
    }
}

impl Index<&'_ str> for Color {
    type Output = i32;
    fn index(&self, s: &str) -> &i32 {
        match s {
            "Red" => &self.red,
            "Green" => &self.green,
            "Blue" => &self.blue,
            _ => panic!("Type AbilityMap does not have field {}", s),
        }
    }
}

// impl IndexMut<&'_ str> for AbilityMap {
//     fn index_mut(&mut self, s: &str) -> &mut Color {
//         match s {
//             "General" => &mut self.General,
//             "Vector" => &mut self.Vector,
//             "Matrix" => &mut self.Matrix,
//             _ => panic!("Type AbilityMap does not have field {}", s),
//         }
//     }
// }

/// Reads the data from the JSON file containing information about how the
/// different flavors of nautiloid are able to dissolve different layers
/// of pearls.
pub fn get_ability_map() -> AbilityMap{
    let src_dir = canonicalize(PathBuf::from("./src")).unwrap();
    let file = src_dir.join(Path::new("static_files/ability_map.json"));
    
    let file = match File::open(&file) {
        Ok(f) => {
            f
        },
        Err(e) => {
            let p = file.to_str().unwrap();
            panic!("File open error: \nFile: {}\nError: {}", p, e);
        }
    };

    let reader = BufReader::new(&file);

    let map: AbilityMap = match serde_json::from_reader(reader) {
        Ok(m) => {
            m
        },
        Err(e) => {
            panic!("File buffer read error: {}", e)
        }
    };

    return map;
}