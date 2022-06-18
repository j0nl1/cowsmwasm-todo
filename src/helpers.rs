use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg, Deps};

use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{CONFIG};

pub fn ensure_is_owner(deps: Deps, sender: &Addr) -> Result<(), ContractError> {
    let addr_canonical = deps.api.addr_canonicalize(sender.as_ref())?;
    let owner = CONFIG.load(deps.storage)?.owner;
    if addr_canonical != owner {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}
