use workspaces::{result::ViewResultDetails, types::Balance};

use crate::*;

/// Check the result of a contract method call
pub fn check_res(res: &ExecutionFinalResult, msg: &str) {
    if res.is_failure() {
        println!("{} | FAIL", msg);
        println!("{:#?}", res);
        panic!("FAIL: {}", msg);
    } else if res.receipt_failures().len() > 0 {
        println!("{} | FAIL", msg);
        println!("RECEIPTS:");
        res.receipt_outcomes()
            .into_iter()
            .for_each(|row| println!("{:?}", row));

        println!("FAILURES:");
        println!("{:?}", res.receipt_failures());

        panic!("FAIL: {}", msg);
    } else {
        res.logs().into_iter().for_each(|row| println!("{:?}", row));
        println!(
            "{} | OK ({} TGas)",
            msg,
            res.total_gas_burnt / 1_000_000_000_000
        );
    }
}

/// Checks that two amount are within epsilon
pub fn assert_almost_eq(left: Balance, right: Balance, epsilon: Balance) {
    println!("{} ~= {}", left, right);

    if left > right {
        assert!((left - right) < epsilon);
    } else {
        assert!((right - left) < epsilon);
    }
}

pub async fn create_account(
    worker: &Worker<Sandbox>,
    account_name: &str,
    near_amount: u128,
) -> anyhow::Result<Account> {
    println!("Creating account \"{}\"", account_name);

    let owner = worker.root_account().unwrap();

    let account = owner
        .create_subaccount(account_name)
        .initial_balance(near_amount)
        .transact()
        .await?
        .into_result()?;

    Ok(account)
}

pub async fn view_call(
    user: &Account,
    contract: &Contract,
    method: &str,
    args_json: serde_json::Value,
) -> anyhow::Result<ViewResultDetails> {
    println!("view {}@{} {:?}", contract.id(), method, args_json);

    let res = user
        .call(contract.id(), method.clone())
        .args_json(args_json)
        .view()
        .await?;

    Ok(res)
}

/// Fast-forward for a given number of epochs.
pub async fn wait_epochs(worker: &Worker<Sandbox>, epochs_num: u64) -> anyhow::Result<()> {
    println!("Fast-forwarding {} epochs...", epochs_num);

    let mut i = 0;

    while i < epochs_num {
        wait_epoch(worker).await?;
        i += 1;
    }

    Ok(())
}

/// Fast-forward for an epoch (approximate).
pub async fn wait_epoch(worker: &Worker<Sandbox>) -> anyhow::Result<()> {
    let start_block = worker.view_block().await?;
    let blocks_per_iteration = 100;
    let mut skipped_blocks = 0;

    loop {
        worker.fast_forward(blocks_per_iteration).await?;
        skipped_blocks += blocks_per_iteration;

        let block = worker.view_block().await?;

        if block.epoch_id() != start_block.epoch_id() {
            break;
        }
    }

    println!("Fast-forwarded {} blocks.", skipped_blocks);

    Ok(())
}
