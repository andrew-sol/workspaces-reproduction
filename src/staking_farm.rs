use near_sdk::{json_types::U128, Balance};
use workspaces::AccountId;

use crate::*;

const ONE_SEC_IN_NS: u64 = 1_000_000_000;

// STAKE METHODS ===========================
// ========================================

pub async fn deposit(
    staking_farm_contract: &Contract,
    user: &Account,
    amount: Balance,
) -> anyhow::Result<()> {
    let res = user
        .call(staking_farm_contract.id(), "deposit")
        .deposit(amount)
        .gas(parse_gas!("100 T") as u64)
        .transact()
        .await?;
    check_res(&res, "staking_farm_contract::deposit");

    Ok(())
}

pub async fn deposit_and_stake(
    staking_farm_contract: &Contract,
    user: &Account,
    amount: Balance,
) -> anyhow::Result<()> {
    let res = user
        .call(staking_farm_contract.id(), "deposit_and_stake")
        .deposit(amount)
        .gas(parse_gas!("130 T") as u64)
        .transact()
        .await?;
    check_res(&res, "staking_farm_contract::deposit_and_stake");

    Ok(())
}

pub async fn stake(
    staking_farm_contract: &Contract,
    user: &Account,
    amount: Balance,
) -> anyhow::Result<()> {
    let res = user
        .call(staking_farm_contract.id(), "stake")
        .args_json(json!({ "amount": U128(amount) }))
        .gas(parse_gas!("130 T") as u64)
        .transact()
        .await?;
    check_res(&res, "staking_farm_contract::stake");

    Ok(())
}

pub async fn unstake(
    staking_farm_contract: &Contract,
    user: &Account,
    amount: Balance,
) -> anyhow::Result<()> {
    let res = user
        .call(staking_farm_contract.id(), "unstake")
        .args_json(json!({ "amount": U128(amount) }))
        .gas(parse_gas!("200 T") as u64)
        .transact()
        .await?;
    check_res(&res, "staking_farm_contract::unstake");

    Ok(())
}

pub async fn stake_all(staking_farm_contract: &Contract, user: &Account) -> anyhow::Result<()> {
    let res = user
        .call(staking_farm_contract.id(), "stake_all")
        .gas(parse_gas!("130 T") as u64)
        .transact()
        .await?;
    check_res(&res, "staking_farm_contract::stake_all");

    Ok(())
}

pub async fn unstake_all(staking_farm_contract: &Contract, user: &Account) -> anyhow::Result<()> {
    let res = user
        .call(staking_farm_contract.id(), "unstake_all")
        .gas(parse_gas!("200 T") as u64)
        .transact()
        .await?;
    check_res(&res, "staking_farm_contract::unstake_all");

    Ok(())
}

pub async fn withdraw(
    staking_farm_contract: &Contract,
    user: &Account,
    amount: Balance,
) -> anyhow::Result<()> {
    let res = user
        .call(staking_farm_contract.id(), "withdraw")
        .args_json(json!({ "amount": U128(amount) }))
        .gas(parse_gas!("130 T") as u64)
        .transact()
        .await?;
    check_res(&res, "staking_farm_contract::withdraw");

    Ok(())
}

pub async fn withdraw_all(staking_farm_contract: &Contract, user: &Account) -> anyhow::Result<()> {
    let res = user
        .call(staking_farm_contract.id(), "withdraw_all")
        .gas(parse_gas!("130 T") as u64)
        .transact()
        .await?;
    check_res(&res, "staking_farm_contract::withdraw_all");

    Ok(())
}

pub async fn get_account_unstaked_balance(
    staking_farm_contract: &Contract,
    user: &Account,
) -> anyhow::Result<u128> {
    let res: U128 = view_call(
        user,
        staking_farm_contract,
        "get_account_unstaked_balance",
        json!({"account_id": user.id()}),
    )
    .await?
    .json()?;

    Ok(res.0)
}

pub async fn get_account_staked_balance(
    staking_farm_contract: &Contract,
    user: &Account,
) -> anyhow::Result<u128> {
    let res: U128 = view_call(
        user,
        staking_farm_contract,
        "get_account_staked_balance",
        json!({"account_id": user.id()}),
    )
    .await?
    .json()?;

    Ok(res.0)
}

pub async fn get_account_total_balance(
    staking_farm_contract: &Contract,
    user: &Account,
) -> anyhow::Result<u128> {
    let res: U128 = view_call(
        user,
        staking_farm_contract,
        "get_account_total_balance",
        json!({"account_id": user.id()}),
    )
    .await?
    .json()?;

    Ok(res.0)
}

pub async fn is_account_unstaked_balance_available(
    staking_farm_contract: &Contract,
    user: &Account,
) -> anyhow::Result<bool> {
    let res: bool = view_call(
        user,
        staking_farm_contract,
        "is_account_unstaked_balance_available",
        json!({"account_id": user.id()}),
    )
    .await?
    .json()?;

    Ok(res)
}

pub async fn get_account(
    staking_farm_contract: &Contract,
    user: &Account,
) -> anyhow::Result<HumanReadableAccount> {
    let res: HumanReadableAccount = view_call(
        user,
        staking_farm_contract,
        "get_account",
        json!({"account_id": user.id()}),
    )
    .await?
    .json()?;

    Ok(res)
}

pub async fn get_pool_summary(
    staking_farm_contract: &Contract,
    user: &Account,
) -> anyhow::Result<PoolSummary> {
    let res: PoolSummary = view_call(user, staking_farm_contract, "get_pool_summary", json!({}))
        .await?
        .json()?;

    Ok(res)
}

pub async fn is_contract_can_withdraw(
    staking_farm_contract: &Contract,
    user: &Account,
) -> anyhow::Result<bool> {
    let res: bool = view_call(
        user,
        staking_farm_contract,
        "is_contract_can_withdraw",
        json!({}),
    )
    .await?
    .json()?;

    Ok(res)
}

// FARM METHODS ===========================
// ========================================

/// Transfer the given amount of ft_contract token to the staking_farm_contract.
pub async fn transfer_farm_token(
    worker: &Worker<Sandbox>,
    ft_contract: &Contract,
    staking_farm_contract: &Contract,
    user: &Account,
    amount: Balance,
) -> anyhow::Result<()> {
    let block = worker.view_block().await?;
    let start_date = block.timestamp() + ONE_SEC_IN_NS * 3;
    let end_date = start_date + ONE_SEC_IN_NS * 100;
    let msg =
        serde_json::to_string(&json!({ "name": "Test", "start_date": format!("{}", start_date), "end_date": format!("{}", end_date) }))
            .unwrap();
    let res = user
        .call(ft_contract.id(), "ft_transfer_call")
        .deposit(1)
        .args_json(json!({
            "receiver_id": staking_farm_contract.id(),
            "amount": amount,
            "msg": msg,
        }))
        .transact()
        .await?;
    check_res(&res, "transfer_farm_token - ft_contract::ft_transfer_call");

    Ok(())
}

pub async fn get_active_farms(
    staking_farm_contract: &Contract,
    user: &Account,
) -> anyhow::Result<Vec<HumanReadableFarm>> {
    let res: Vec<HumanReadableFarm> =
        view_call(user, staking_farm_contract, "get_active_farms", json!({}))
            .await?
            .json()?;

    Ok(res)
}

pub async fn get_unclaimed_reward(
    staking_farm_contract: &Contract,
    user: &Account,
    farm_id: u64,
) -> anyhow::Result<u128> {
    let res: U128 = view_call(
        user,
        staking_farm_contract,
        "get_unclaimed_reward",
        json!({"account_id": user.id(), "farm_id": farm_id}),
    )
    .await?
    .json()?;

    Ok(res.0)
}

pub async fn claim(
    staking_farm_contract: &Contract,
    user: &Account,
    token_id: AccountId,
    delegator_id: Option<AccountId>,
) -> anyhow::Result<()> {
    let res = user
        .call(staking_farm_contract.id(), "claim")
        .deposit(1)
        .gas(parse_gas!("100 T") as u64)
        .args_json(json!({
            "token_id": token_id,
            "delegator_id": delegator_id,
        }))
        .transact()
        .await?;
    check_res(&res, "staking_farm_contract::claim");

    Ok(())
}

pub async fn stop_farm(
    staking_farm_contract: &Contract,
    user: &Account,
    farm_id: u64,
) -> anyhow::Result<()> {
    let res = user
        .call(staking_farm_contract.id(), "stop_farm")
        .deposit(1)
        .gas(parse_gas!("100 T") as u64)
        .args_json(json!({
            "farm_id": farm_id,
        }))
        .transact()
        .await?;
    check_res(&res, "staking_farm_contract::stop_farm");

    Ok(())
}
