use cosmwasm_std::{Deps, Env};
use utils::state::AssetInfo;

use crate::{
    msg::QueryFilters,
    state::{get_locality_state, get_raffle_state, LocalityInfo, RaffleInfo},
    utils::buyer_can_buy_ticket,
};

pub fn state_filter(env: &Env, raffle_info: &RaffleInfo, filters: &QueryFilters) -> bool {
    match &filters.states {
        Some(state) => state.contains(&get_raffle_state(env, raffle_info).to_string()),
        None => true,
    }
}

pub fn owner_filter(raffle_info: &RaffleInfo, filters: &QueryFilters) -> bool {
    match &filters.owner {
        Some(owner) => raffle_info.owner == owner.clone(),
        None => true,
    }
}

pub fn contains_token_filter(raffle_info: &RaffleInfo, filters: &QueryFilters) -> bool {
    match &filters.contains_token {
        Some(token) => raffle_info.assets.iter().any(|asset| match asset {
            AssetInfo::Coin(x) => x.denom == token.as_ref(),
            AssetInfo::Cw721Coin(x) => x.address == token.as_ref(),
            AssetInfo::Sg721Token(x) => x.address == token.as_ref(),
        }),
        None => true,
    }
}

pub fn has_gated_rights_filter(
    deps: Deps,
    raffle_info: &RaffleInfo,
    filters: &QueryFilters,
) -> bool {
    match &filters.gated_rights_ticket_buyer {
        Some(buyer) => buyer_can_buy_ticket(deps, raffle_info, buyer.to_string()).is_ok(),
        None => true,
    }
}

// locality
pub fn locality_state_filter(
    env: &Env,
    locality_info: &LocalityInfo,
    filters: &QueryFilters,
) -> bool {
    match &filters.states {
        Some(state) => state.contains(&get_locality_state(env, locality_info).to_string()),
        None => true,
    }
}
