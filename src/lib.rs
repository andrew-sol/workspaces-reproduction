use near_sdk::serde_json;
use near_units::parse_gas;
use near_units::parse_near;
use serde_json::json;
use workspaces::{network::Sandbox, result::ExecutionFinalResult, Account, Contract, Worker};

use crate::staking_farm::*;
use crate::types::*;
use crate::utils::*;
use crate::validator::*;

pub mod staking_farm;
pub mod types;
pub mod utils;
pub mod validator;

pub const STAKING_FARM_WASM_FILEPATH: &str = "./contracts/staking_farm.wasm";
pub const VALIDATOR_WASM_FILEPATH: &str = "./contracts/staking_pool.wasm";

pub const ONE_DAY_IN_NANOSECONDS: u64 = 86400000000000;

#[tokio::main]
#[allow(dead_code, unused_must_use)]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    // create accounts
    let owner = worker.root_account().unwrap();
    let alice = create_account(&worker, "alice", parse_near!("2000000 N")).await?;

    // deploy contracts
    let (validator_contract, staking_farm_contract) = deploy_contracts(&worker).await?;

    // initialize contracts
    init_contracts(&owner, &validator_contract, &staking_farm_contract).await?;

    // begin tests
    test_deposit_stake_unstake(&alice, &staking_farm_contract, &validator_contract).await?;
    test_withdraw(&worker, &alice, &staking_farm_contract).await?;

    Ok(())
}

async fn deploy_contracts(worker: &Worker<Sandbox>) -> anyhow::Result<(Contract, Contract)> {
    println!("Deploying contracts...");

    let validator_account = create_account(&worker, "validator", parse_near!("100000 N")).await?;
    let validator_wasm = std::fs::read(VALIDATOR_WASM_FILEPATH)?;
    let validator_contract = validator_account.deploy(&validator_wasm).await?.unwrap();

    let staking_farm_wasm = std::fs::read(STAKING_FARM_WASM_FILEPATH)?;
    let staking_farm_contract = worker.dev_deploy(&staking_farm_wasm).await?;

    Ok((validator_contract, staking_farm_contract))
}

async fn init_contracts(
    owner: &Account,
    validator_contract: &Contract,
    staking_farm_contract: &Contract,
) -> anyhow::Result<()> {
    println!("Initializing contracts...");

    let pk = owner.secret_key().public_key();
    let res = validator_contract
        .call("new")
        .args_json(json!({
            "owner_id": owner.id(),
            "stake_public_key": pk,
            "reward_fee_fraction": {
                "numerator": 1,
                "denominator": 100,
            },
        }))
        .gas(parse_gas!("50 T") as u64)
        .transact()
        .await?;
    check_res(&res, "validator_contract::new()");

    let res = staking_farm_contract
        .call("new")
        .args_json(json!({
            "owner_id": owner.id(),
            "validator_id": validator_contract.id(),
            "reward_fee_fraction": {
                "numerator": 1,
                "denominator": 2,
            },
        }))
        .transact()
        .await?;
    check_res(&res, "staking_farm_contract::new()");

    Ok(())
}

#[allow(unused_must_use)]
pub async fn test_deposit_stake_unstake(
    user: &Account,
    staking_farm_contract: &Contract,
    validator_contract: &Contract,
) -> anyhow::Result<()> {
    println!("Start: test_deposit_stake_unstake");

    let pool_summary = get_pool_summary(staking_farm_contract, user).await?;
    println!("pool_summary {:#?}", pool_summary);

    let farm_account = staking_farm_contract.as_account();

    // DEPOSIT #################
    deposit(staking_farm_contract, user, parse_near!("1000 N")).await?;

    let account = get_account(staking_farm_contract, user).await?;
    let total_balance = get_account_total_balance(staking_farm_contract, user).await?;
    let staked_balance = get_account_staked_balance(staking_farm_contract, user).await?;
    let unstaked_balance = get_account_unstaked_balance(staking_farm_contract, user).await?;
    let validator_total_balance =
        validator_get_account_total_balance(&validator_contract, farm_account).await?;
    let validator_staked_balance =
        validator_get_account_staked_balance(&validator_contract, farm_account).await?;
    let validator_unstaked_balance =
        validator_get_account_unstaked_balance(&validator_contract, farm_account).await?;

    assert_eq!(total_balance, parse_near!("1000 N"));
    assert_eq!(account.staked_balance.0, parse_near!("0 N"));
    assert_eq!(account.unstaked_balance.0, parse_near!("1000 N"));
    assert_eq!(account.can_withdraw, true);
    assert_eq!(account.staked_balance.0, staked_balance);
    assert_eq!(account.unstaked_balance.0, unstaked_balance);
    assert_eq!(validator_total_balance, parse_near!("1000 N"));
    assert_eq!(validator_staked_balance, parse_near!("0 N"));
    assert_eq!(validator_unstaked_balance, parse_near!("1000 N"));

    // STAKE #################
    stake(staking_farm_contract, user, parse_near!("200 N")).await?;

    let account = get_account(staking_farm_contract, user).await?;
    let staked_balance = get_account_staked_balance(staking_farm_contract, user).await?;
    let unstaked_balance = get_account_unstaked_balance(staking_farm_contract, user).await?;
    let validator_staked_balance =
        validator_get_account_staked_balance(&validator_contract, farm_account).await?;
    let validator_unstaked_balance =
        validator_get_account_unstaked_balance(&validator_contract, farm_account).await?;

    assert_eq!(account.staked_balance.0, parse_near!("200 N"));
    assert_eq!(account.unstaked_balance.0, parse_near!("800 N"));
    assert_eq!(account.can_withdraw, true);
    assert_eq!(account.staked_balance.0, staked_balance);
    assert_eq!(account.unstaked_balance.0, unstaked_balance);
    assert_eq!(validator_staked_balance, parse_near!("200 N"));
    assert_eq!(validator_unstaked_balance, parse_near!("800 N"));

    // STAKE_ALL #################
    stake_all(staking_farm_contract, user).await?;

    let account = get_account(staking_farm_contract, user).await?;
    let staked_balance = get_account_staked_balance(staking_farm_contract, user).await?;
    let unstaked_balance = get_account_unstaked_balance(staking_farm_contract, user).await?;
    let validator_staked_balance =
        validator_get_account_staked_balance(&validator_contract, farm_account).await?;
    let validator_unstaked_balance =
        validator_get_account_unstaked_balance(&validator_contract, farm_account).await?;

    assert_eq!(account.staked_balance.0, parse_near!("1000 N"));
    assert_eq!(account.unstaked_balance.0, parse_near!("0 N"));
    assert_eq!(account.can_withdraw, true);
    assert_eq!(account.staked_balance.0, staked_balance);
    assert_eq!(account.unstaked_balance.0, unstaked_balance);
    assert_eq!(validator_staked_balance, parse_near!("1000 N"));
    assert_eq!(validator_unstaked_balance, parse_near!("0 N"));

    // UNSTAKE #################
    unstake(staking_farm_contract, user, parse_near!("100 N")).await?;

    let account = get_account(staking_farm_contract, user).await?;
    let staked_balance = get_account_staked_balance(staking_farm_contract, user).await?;
    let unstaked_balance = get_account_unstaked_balance(staking_farm_contract, user).await?;
    let validator_staked_balance =
        validator_get_account_staked_balance(&validator_contract, farm_account).await?;
    let validator_unstaked_balance =
        validator_get_account_unstaked_balance(&validator_contract, farm_account).await?;

    assert_eq!(account.staked_balance.0, parse_near!("900 N"));
    assert_eq!(account.unstaked_balance.0, parse_near!("100 N"));
    assert_eq!(account.can_withdraw, false);
    assert_eq!(account.staked_balance.0, staked_balance);
    assert_eq!(account.unstaked_balance.0, unstaked_balance);
    assert_eq!(validator_staked_balance, parse_near!("900 N"));
    assert_eq!(validator_unstaked_balance, parse_near!("100 N"));

    // UNSTAKE_ALL #################
    unstake_all(staking_farm_contract, user).await?;

    let account = get_account(staking_farm_contract, user).await?;
    let staked_balance = get_account_staked_balance(staking_farm_contract, user).await?;
    let unstaked_balance = get_account_unstaked_balance(staking_farm_contract, user).await?;
    let validator_staked_balance =
        validator_get_account_staked_balance(&validator_contract, farm_account).await?;
    let validator_unstaked_balance =
        validator_get_account_unstaked_balance(&validator_contract, farm_account).await?;

    assert_eq!(account.staked_balance.0, parse_near!("0 N"));
    assert_eq!(account.unstaked_balance.0, parse_near!("1000 N"));
    assert_eq!(account.can_withdraw, false);
    assert_eq!(account.staked_balance.0, staked_balance);
    assert_eq!(account.unstaked_balance.0, unstaked_balance);
    assert_eq!(validator_staked_balance, parse_near!("0 N"));
    assert_eq!(validator_unstaked_balance, parse_near!("1000 N"));

    // DEPOSIT_AND_STAKE #################
    deposit_and_stake(staking_farm_contract, user, parse_near!("1000 N")).await?;

    let staked_balance = get_account_staked_balance(staking_farm_contract, user).await?;
    let unstaked_balance = get_account_unstaked_balance(staking_farm_contract, user).await?;

    assert_eq!(staked_balance, parse_near!("1000 N"));
    assert_eq!(unstaked_balance, parse_near!("1000 N"));

    println!("Passed ✅ test_deposit_stake_unstake");
    Ok(())
}

#[allow(unused_must_use)]
pub async fn test_withdraw(
    worker: &Worker<Sandbox>,
    user: &Account,
    staking_farm_contract: &Contract,
) -> anyhow::Result<()> {
    println!("Start: test_withdraw");

    wait_epochs(worker, 5).await?;

    let pool_summary = get_pool_summary(staking_farm_contract, user).await?;
    println!("pool_summary {:#?}", pool_summary);

    let can_withdraw = is_contract_can_withdraw(staking_farm_contract, user).await?;
    println!("Can withdraw {}", can_withdraw);

    let account = get_account(staking_farm_contract, user).await?;
    println!("Account state: {:?}", account);

    assert_eq!(account.can_withdraw, true);
    assert_eq!(account.unstaked_balance.0, parse_near!("1000 N"));

    // WITHDRAW 200 NEAR ##############
    println!("Withdrawing 200 NEAR...");
    withdraw(staking_farm_contract, user, parse_near!("200 N")).await?;

    let account = get_account(staking_farm_contract, user).await?;

    assert_eq!(account.can_withdraw, true);
    assert_eq!(account.unstaked_balance.0, parse_near!("800 N"));

    // WITHDRAW_ALL ##############
    println!("Withdrawing the rest of NEAR...");
    let prev_total_balance = get_account_total_balance(staking_farm_contract, user).await?;
    withdraw_all(staking_farm_contract, user).await?;

    let account = get_account(staking_farm_contract, user).await?;
    let total_balance = get_account_total_balance(staking_farm_contract, user).await?;

    assert_eq!(account.can_withdraw, true);
    assert_eq!(account.unstaked_balance.0, 0);
    assert_eq!(prev_total_balance - total_balance, parse_near!("800 N"));

    println!("Passed ✅ test_withdraw");
    Ok(())
}
