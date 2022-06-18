use cosmwasm_std::{Addr, Deps};

use crate::error::ContractError;
use crate::state::CONFIG;

pub fn ensure_is_owner(deps: Deps, sender: &Addr) -> Result<(), ContractError> {
    let addr_canonical = deps.api.addr_canonicalize(sender.as_ref())?;
    let owner = CONFIG.load(deps.storage)?.owner;
    if addr_canonical != owner {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}
