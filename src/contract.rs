// Import all the libraries that we will use
use crate::errors::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::try_query_contract;
use crate::state::{Config, CONFIG};
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::{get_contract_version, set_contract_version};

// version info for migration info
const CONTRACT_NAME: &str = "query-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Here we define the entry point to the contract. We use the instantiate function to start it.
// This is a function that can only run once and it serves as a constructor for the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let config = Config {};
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

// Here we define the entry point to perform actions on the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        _ => Err(ContractError::UnexpectedMessage {}),
    }
}

// Here we define the entry point to the contract for querying it.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryContract { contract, wallet } => {
            to_binary(&try_query_contract(deps, contract, wallet)?)
        }
        _ => Err(StdError::generic_err("Unexpected query message")),
    }
}

// Entry point for migration functions
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    msg: MigrateMsg<Config>,
) -> Result<Response, ContractError> {
    match msg {
        MigrateMsg { version, config } => try_migrate(deps, version, config),
    }
}

fn try_migrate(
    deps: DepsMut,
    version: String,
    config: Option<Config>,
) -> Result<Response, ContractError> {
    let contract_version = get_contract_version(deps.storage)?;
    set_contract_version(deps.storage, contract_version.contract, version)?;

    if config.is_some() {
        CONFIG.save(deps.storage, &config.unwrap())?
    }

    Ok(Response::new()
        .add_attribute("method", "try_migrate")
        .add_attribute("version", contract_version.version))
}
