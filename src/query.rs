use crate::msg::{ContractQueryMsg, ContractQueryResponse};
use cosmwasm_std::{to_binary, Addr, Binary, Deps, Empty, QueryRequest, StdResult, WasmQuery};

pub fn encode_msg_query(msg: Binary, address: Addr) -> StdResult<QueryRequest<Empty>> {
    Ok(WasmQuery::Smart {
        contract_addr: address.to_string(),
        msg: msg,
    }
    .into())
}

pub fn try_query_contract(
    deps: Deps,
    contract: Addr,
    wallet: Addr,
) -> StdResult<ContractQueryResponse> {
    let msg = ContractQueryMsg::Tokens { owner: wallet };
    let wasm = encode_msg_query(to_binary(&msg).unwrap(), contract)?;
    let query_response: ContractQueryResponse = deps.querier.query(&wasm.into())?;
    Ok(query_response)
}
