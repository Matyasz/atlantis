use serde::Deserialize;
use std::ops::Index;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Color {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
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
    type Output = u32;
    fn index(&self, s: &str) -> &u32 {
        match s {
            "Red" => &self.red,
            "Green" => &self.green,
            "Blue" => &self.blue,
            _ => panic!("Type AbilityMap does not have field {}", s),
        }
    }
}
