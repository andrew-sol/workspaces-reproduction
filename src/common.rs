use crate::*;

/// Registers the given account on the given contract using the standard "Storage Deposit" API
pub async fn register_user_on_contract(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    // get required deposit for registration
    let res = user
        .call(contract.id(), "storage_balance_bounds")
        .transact()
        .await?;
    check_res(
        &res,
        format!("{}::storage_balance_bounds", contract.id()).as_str(),
    );
    let storage_balance_bounds: StorageBalanceBoundsJson = res.json()?;

    // register user
    deposit_storage(
        user,
        &contract,
        storage_balance_bounds.min.parse::<u128>().unwrap(),
    )
    .await?;

    Ok(())
}

/// Deposit storage onto the given contract using the standard "storage_deposit" method
pub async fn deposit_storage(
    user: &Account,
    contract: &Contract,
    amount: u128,
) -> anyhow::Result<()> {
    // register user
    let res = user
        .call(contract.id(), "storage_deposit")
        .args_json(json!({}))
        .deposit(amount)
        .gas(parse_gas!("75 T") as u64)
        .transact()
        .await?;
    check_res(&res, format!("{}::storage_deposit", contract.id()).as_str());

    Ok(())
}
