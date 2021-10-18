#[derive(Debug)]
pub struct Pass {
    pub from_id: u32,
    pub pearl_id: u32,
    pub to_id: u32
}

#[derive(Debug)]
pub struct Nom {
    pub nautiloid_id: u32,
    pub pearl_id: u32
}
#[derive(Debug)]
pub enum ActionType {
    Pass(Pass),
    Nom(Nom)
}