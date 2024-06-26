use cosmwasm_std::{
    coin, ensure, ensure_eq, entry_point, to_json_binary, Decimal, Deps, DepsMut, Env, MessageInfo,
    QueryResponse, Reply, StdResult, Uint128,
};

use crate::{
    error::ContractError,
    execute::{
        execute_buy_locality_ticket, execute_buy_tickets, execute_cancel_raffle, execute_claim,
        execute_create_locality, execute_create_raffle, execute_modify_raffle, execute_receive,
        execute_receive_nois, execute_sudo_toggle_lock, execute_toggle_lock, execute_update_config,
        handle_cron,
    },
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, RaffleResponse, SudoMsg},
    query::{
        add_raffle_winners, query_all_localities_raw, query_all_raffles, query_all_tickets,
        query_config, query_discount, query_ticket_count,
    },
    state::{
        get_raffle_state, load_raffle, Config, LocalityConfig, CONFIG, LOCALITY_CONFIG,
        LOCALITY_INFO, MAX_TICKET_NUMBER, MINIMUM_RAFFLE_DURATION, OLD_CONFIG,
        STATIC_RAFFLE_CREATION_FEE,
    },
    utils::get_nois_randomness,
};
use cw_utils::parse_reply_instantiate_data;
use utils::{
    state::{is_valid_name, Locks, NATIVE_DENOM},
    types::Response,
};

use cw2::set_contract_version;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let nois_proxy_addr = deps
        .api
        .addr_validate(&msg.nois_proxy_addr)
        .map_err(|_| ContractError::InvalidProxyAddress)?;

    // define the accepted fee coins
    let creation_coins = match msg.creation_coins {
        Some(cc_msg) => cc_msg,
        None => vec![coin(STATIC_RAFFLE_CREATION_FEE, NATIVE_DENOM)], // TODO: update to handle ibc contract support native denoms
    };

    // fee decimal range
    ensure!(
        msg.raffle_fee >= Decimal::zero() && msg.raffle_fee <= Decimal::one(),
        ContractError::InvalidFeeRate {}
    );
    ensure!(
        msg.nois_proxy_coin.amount >= Uint128::zero(),
        ContractError::InvalidProxyCoin
    );
    // valid name
    if !is_valid_name(&msg.name) {
        return Err(ContractError::InvalidName {});
    }
    // define internal contract
    let config = Config {
        name: msg.name,
        owner: deps
            .api
            .addr_validate(&msg.owner.unwrap_or_else(|| info.sender.to_string()))?,
        fee_addr: deps
            .api
            .addr_validate(&msg.fee_addr.unwrap_or_else(|| info.sender.to_string()))?,
        last_raffle_id: None,
        last_locality_id: None,
        minimum_raffle_duration: msg
            .minimum_raffle_duration
            .unwrap_or(MINIMUM_RAFFLE_DURATION)
            .max(MINIMUM_RAFFLE_DURATION),
        raffle_fee: msg.raffle_fee,
        locks: Locks {
            lock: false,
            sudo_lock: false,
        },
        nois_proxy_addr,
        nois_proxy_coin: msg.nois_proxy_coin,
        creation_coins,
        max_tickets_per_raffle: Some(msg.max_ticket_number.unwrap_or(MAX_TICKET_NUMBER)),
        fee_discounts: msg
            .fee_discounts
            .into_iter()
            .map(|d| d.check(deps.api))
            .collect::<Result<_, _>>()?,
    };
    CONFIG.save(deps.storage, &config)?;
    if let Some(params) = msg.locality_config {
        LOCALITY_CONFIG.save(
            deps.storage,
            &LocalityConfig {
                locality_mint_portion: Decimal::zero(),
            },
        )?;
    }
    set_contract_version(
        deps.storage,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), ::cosmwasm_std::entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response> {
    let old_config = OLD_CONFIG.load(deps.storage)?;

    let config = Config {
        name: old_config.name,
        owner: old_config.owner,
        fee_addr: old_config.fee_addr,
        last_raffle_id: old_config.last_raffle_id,
        last_locality_id: None,
        minimum_raffle_duration: old_config.minimum_raffle_duration,
        max_tickets_per_raffle: old_config.max_tickets_per_raffle,
        raffle_fee: old_config.raffle_fee,
        locks: old_config.locks,
        nois_proxy_addr: old_config.nois_proxy_addr,
        nois_proxy_coin: old_config.nois_proxy_coin,
        creation_coins: old_config.creation_coins,
        fee_discounts: msg
            .fee_discounts
            .into_iter()
            .map(|d| d.check(deps.api))
            .collect::<Result<_, _>>()?,
    };
    set_contract_version(
        deps.storage,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )?;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateRaffle {
            owner,
            assets,
            raffle_options,
            raffle_ticket_price,
        } => execute_create_raffle(
            deps,
            env,
            info,
            owner,
            assets,
            raffle_ticket_price,
            raffle_options,
        ),
        ExecuteMsg::CancelRaffle { raffle_id } => execute_cancel_raffle(deps, env, info, raffle_id),
        ExecuteMsg::ModifyRaffle {
            raffle_id,
            raffle_ticket_price,
            raffle_options,
        } => execute_modify_raffle(
            deps,
            env,
            info,
            raffle_id,
            raffle_ticket_price,
            raffle_options,
        ),
        ExecuteMsg::BuyTicket {
            raffle_id,
            ticket_count,
            sent_assets,
            on_behalf_of,
        } => execute_buy_tickets(
            deps,
            env,
            info,
            raffle_id,
            ticket_count,
            sent_assets,
            on_behalf_of,
        ),
        ExecuteMsg::Receive(msg) => execute_receive(deps, env, info, msg),
        ExecuteMsg::NoisReceive { callback } => execute_receive_nois(deps, env, info, callback),
        ExecuteMsg::ClaimRaffle { raffle_id } => execute_claim(deps, env, raffle_id),
        ExecuteMsg::ToggleLock { lock } => execute_toggle_lock(deps, env, info, lock),
        ExecuteMsg::UpdateConfig {
            name,
            owner,
            fee_addr,
            minimum_raffle_duration,
            max_tickets_per_raffle,
            raffle_fee,
            nois_proxy_addr,
            nois_proxy_coin,
            creation_coins,
            fee_discounts,
        } => execute_update_config(
            deps,
            env,
            info,
            name,
            owner,
            fee_addr,
            minimum_raffle_duration,
            max_tickets_per_raffle,
            raffle_fee,
            nois_proxy_addr,
            nois_proxy_coin,
            creation_coins,
            fee_discounts,
        ),
        ExecuteMsg::UpdateRandomness { raffle_id } => {
            let config = CONFIG.load(deps.storage)?;
            ensure_eq!(info.sender, config.owner, ContractError::Unauthorized);
            let msg = get_nois_randomness(deps.as_ref(), raffle_id)?;
            Ok(Response::new().add_message(msg))
        }
        ExecuteMsg::CreateLocality { locality_params } => {
            execute_create_locality(deps, info, env, locality_params)
        }
        ExecuteMsg::PurchaseLocalityTicket {
            id,
            ticket_count,
            assets,
        } => execute_buy_locality_ticket(deps, env, info, id, ticket_count, assets),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    let response = match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?)?,
        QueryMsg::RaffleInfo { raffle_id } => {
            let mut raffle_info = load_raffle(deps.storage, raffle_id)?;
            let raffle_state = get_raffle_state(&env, &raffle_info);
            add_raffle_winners(deps, &env, raffle_id, &mut raffle_info)?;
            to_json_binary(&RaffleResponse {
                raffle_id,
                raffle_state,
                raffle_info: Some(raffle_info),
            })?
        }
        QueryMsg::AllRaffles {
            start_after,
            limit,
            filters,
        } => to_json_binary(&query_all_raffles(deps, env, start_after, limit, filters)?)?,
        QueryMsg::AllTickets {
            raffle_id,
            start_after,
            limit,
        } => to_json_binary(&query_all_tickets(
            deps,
            env,
            raffle_id,
            start_after,
            limit,
        )?)?,
        QueryMsg::TicketCount { owner, raffle_id } => {
            to_json_binary(&query_ticket_count(deps, env, raffle_id, owner)?)?
        }
        QueryMsg::FeeDiscount { user } => to_json_binary(&query_discount(deps, user)?)?,
        QueryMsg::AllLocalities {
            start_after,
            limit,
            filters,
        } => to_json_binary(&query_all_localities_raw(
            deps,
            env,
            start_after,
            limit,
            filters,
        )?)?,
    };
    Ok(response)
}

// sudo entry point for governance override
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::ToggleLock { lock } => {
            execute_sudo_toggle_lock(deps, env, lock).map_err(|_| ContractError::ContractBug {})
        }
        SudoMsg::BeginBlock {} => handle_cron(deps, env).map_err(|err| err),
        SudoMsg::EndBlock {} => unimplemented!(),
    }
}

// Reply callback triggered from cw721 contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id.clone() > CONFIG.load(deps.storage)?.last_locality_id.expect("msg") {
        return Err(ContractError::InvalidReplyID {});
    }
    let mut info = LOCALITY_INFO.load(deps.storage, msg.id.clone())?;

    let reply = parse_reply_instantiate_data(msg.clone());

    match reply {
        Ok(res) => {
            let sg721_address = res.contract_address;
            info.collection = Some(deps.api.addr_validate(&sg721_address)?);
            LOCALITY_INFO.save(deps.storage, msg.id, &info)?;
            Ok(Response::default()
                .add_attribute("action", "instantiate_sg721_reply")
                .add_attribute("sg721_address", sg721_address))
        }
        Err(_) => Err(ContractError::InstantiateSg721Error {}),
    }
}
