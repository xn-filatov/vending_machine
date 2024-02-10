use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ADMIN, STORAGE};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    ADMIN.save(deps.storage, &msg.admin)?;
    STORAGE.save(deps.storage, &msg.storage)?;

    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        ItemCount {} => to_json_binary(&query::items_count(deps)?),
    }
}

mod query {
    use super::*;
    use crate::msg::ItemsCountResp;

    pub fn items_count(deps: Deps) -> StdResult<ItemsCountResp> {
        let storage = STORAGE.load(deps.storage)?;

        Ok(ItemsCountResp { storage })
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        GetItem { item_type } => exec::get_item(deps, item_type),
        Refill { new_storage } => exec::refill(deps, _info, new_storage),
    }
}

mod exec {
    use cosmwasm_std::Uint128;

    use crate::{
        msg::ItemType,
        state::{Chips, Chocolate, Storage, Water},
    };

    use super::*;

    pub fn get_item(deps: DepsMut, item_type: ItemType) -> Result<Response, ContractError> {
        let _ = STORAGE.update(deps.storage, |mut state| -> StdResult<_> {
            match item_type {
                ItemType::Chocolate => state.chocolate.count -= Uint128::new(1),
                ItemType::Water => state.water.count -= Uint128::new(1),
                ItemType::Chips => state.chips.count -= Uint128::new(1),
            };

            Ok(state)
        });
        Ok(Response::new())
    }

    pub fn refill(
        deps: DepsMut,
        info: MessageInfo,
        new_storage: Storage,
    ) -> Result<Response, ContractError> {
        let admin = ADMIN.load(deps.storage)?;
        if info.sender != admin {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }
        let storage = STORAGE.load(deps.storage)?;

        let _ = STORAGE.save(
            deps.storage,
            &Storage {
                chocolate: Chocolate {
                    count: storage.chocolate.count + new_storage.chocolate.count,
                },
                water: Water {
                    count: storage.water.count + new_storage.water.count,
                },
                chips: Chips {
                    count: storage.chips.count + new_storage.chips.count,
                },
            },
        );

        Ok(Response::new())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Uint128};
    use cw_multi_test::{App, ContractWrapper, Executor};

    use crate::{
        msg::ItemsCountResp,
        state::{Chips, Chocolate, Storage, Water},
    };

    use super::*;

    const MOCK_STORAGE: Storage = Storage {
        chocolate: Chocolate {
            count: Uint128::new(100),
        },
        water: Water {
            count: Uint128::new(10),
        },
        chips: Chips {
            count: Uint128::new(10),
        },
    };

    fn create_instance() -> (Addr, App) {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admin: Addr::unchecked("owner"),
                    storage: MOCK_STORAGE,
                },
                &[],
                "VendingMachine",
                None,
            )
            .unwrap();

        (addr, app)
    }

    #[test]
    fn items_count() {
        let (addr, app) = create_instance();

        let resp: ItemsCountResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::ItemCount {})
            .unwrap();

        assert_eq!(resp.storage, MOCK_STORAGE);
    }

    #[test]
    fn get_item() {
        let (addr, mut app) = create_instance();

        app.execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &ExecuteMsg::GetItem {
                item_type: crate::msg::ItemType::Chocolate,
            },
            &[],
        )
        .unwrap();

        let resp: ItemsCountResp = app
            .wrap()
            .query_wasm_smart(&addr, &QueryMsg::ItemCount {})
            .unwrap();

        assert_eq!(
            resp.storage,
            Storage {
                chocolate: Chocolate {
                    count: Uint128::new(99)
                },
                ..MOCK_STORAGE
            }
        );
    }

    #[test]
    fn refill() {
        let (addr, mut app) = create_instance();

        app.execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &ExecuteMsg::Refill {
                new_storage: Storage {
                    chocolate: Chocolate {
                        count: Uint128::new(5),
                    },
                    water: Water {
                        count: Uint128::new(15),
                    },
                    chips: Chips {
                        count: Uint128::new(25),
                    },
                },
            },
            &[],
        )
        .unwrap();

        let resp: ItemsCountResp = app
            .wrap()
            .query_wasm_smart(&addr, &QueryMsg::ItemCount {})
            .unwrap();

        assert_eq!(
            resp.storage,
            Storage {
                chocolate: Chocolate {
                    count: Uint128::new(105)
                },
                water: Water {
                    count: Uint128::new(25)
                },
                chips: Chips {
                    count: Uint128::new(35)
                },
                ..MOCK_STORAGE
            }
        );
    }

    #[test]
    fn unauthorized_refill() {
        let (addr, mut app) = create_instance();

        let err = app
            .execute_contract(
                Addr::unchecked("user"),
                addr,
                &ExecuteMsg::Refill {
                    new_storage: MOCK_STORAGE,
                },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::Unauthorized {
                sender: Addr::unchecked("user")
            },
            err.downcast().unwrap()
        );
    }
}
