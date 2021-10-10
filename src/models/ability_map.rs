use serde::Deserialize;
use std::ops::Index;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Color {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
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
