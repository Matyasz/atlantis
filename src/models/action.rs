pub struct Pass {
    pub from_id: u32,
    pub pearl_id: u32,
    pub to_id: u32
}

pub struct Nom {
    pub nautiloid_id: u32,
    pub pearl_id: u32
}

pub enum ActionType {
    Pass(Pass),
    Nom(Nom)
}


// {
//     "1": {
//         "Pass": {
//             "pearl_id":52,
//             "to_worker":0
//         }
//     },
//     "0": {
//         "Nom":1234
//     }
// }