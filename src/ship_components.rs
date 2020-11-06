use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShipObjective {
    Idle,
    TravelTo(Uuid),
    //DockedAt(Uuid),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ship {
    pub objective: ShipObjective,
    pub speed: f64,
}
