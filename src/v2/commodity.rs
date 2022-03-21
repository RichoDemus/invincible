use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumIter, Display)]
pub enum Commodity {
    Food,
    HydrogenTanks,
    Fuel,
}
