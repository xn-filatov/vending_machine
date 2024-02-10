use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::Storage;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub storage: Storage,
}
#[cw_serde]
pub enum ItemType {
    Chocolate,
    Water,
    Chips,
}

#[cw_serde]
pub struct ItemsCountResp {
    pub storage: Storage,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ItemsCountResp)]
    ItemCount {},
}

#[cw_serde]
pub enum ExecuteMsg {
    GetItem { item_type: ItemType },
    Refill { new_storage: Storage },
}
