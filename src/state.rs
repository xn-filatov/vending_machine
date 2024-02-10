use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

pub const ADMIN: Item<Addr> = Item::new("admin");

#[cw_serde]
pub struct Chocolate {
    pub count: Uint128,
}
#[cw_serde]
pub struct Water {
    pub count: Uint128,
}
#[cw_serde]
pub struct Chips {
    pub count: Uint128,
}

#[cw_serde]
pub struct Storage {
    pub chocolate: Chocolate,
    pub water: Water,
    pub chips: Chips,
}
pub const STORAGE: Item<Storage> = Item::new("storage");
