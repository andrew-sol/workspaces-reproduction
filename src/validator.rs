use near_sdk::json_types::U128;

use crate::*;

pub async fn validator_get_account_unstaked_balance(
    validator_contract: &Contract,
    user: &Account,
) -> anyhow::Result<u128> {
    let res: U128 = view_call(
        user,
        validator_contract,
        "get_account_unstaked_balance",
        json!({"account_id": user.id()}),
    )
    .await?
    .json()?;

    Ok(res.0)
}

pub async fn validator_get_account_staked_balance(
    validator_contract: &Contract,
    user: &Account,
) -> anyhow::Result<u128> {
    let res: U128 = view_call(
        user,
        validator_contract,
        "get_account_staked_balance",
        json!({"account_id": user.id()}),
    )
    .await?
    .json()?;

    Ok(res.0)
}

pub async fn validator_get_account_total_balance(
    validator_contract: &Contract,
    user: &Account,
) -> anyhow::Result<u128> {
    let res: U128 = view_call(
        user,
        validator_contract,
        "get_account_total_balance",
        json!({"account_id": user.id()}),
    )
    .await?
    .json()?;

    Ok(res.0)
}

pub async fn validator_is_account_unstaked_balance_available(
    validator_contract: &Contract,
    user: &Account,
) -> anyhow::Result<bool> {
    let res: bool = view_call(
        user,
        validator_contract,
        "is_account_unstaked_balance_available",
        json!({"account_id": user.id()}),
    )
    .await?
    .json()?;

    Ok(res)
}

pub async fn validator_get_account(
    validator_contract: &Contract,
    user: &Account,
) -> anyhow::Result<HumanReadableAccount> {
    let res: HumanReadableAccount = view_call(
        user,
        validator_contract,
        "get_account",
        json!({"account_id": user.id()}),
    )
    .await?
    .json()?;

    Ok(res)
}
