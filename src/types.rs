use near_sdk::{
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
};
use workspaces::AccountId;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBalanceBoundsJson {
    pub min: String,
    pub max: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableAccount {
    pub account_id: AccountId,
    /// The unstaked balance that can be withdrawn or staked.
    pub unstaked_balance: U128,
    /// The amount balance staked at the current "stake" share price.
    pub staked_balance: U128,
    /// Whether the unstaked balance is available for withdrawal now.
    pub can_withdraw: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Ratio {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableFarm {
    pub farm_id: u64,
    pub name: String,
    pub token_id: AccountId,
    pub amount: U128,
    pub start_date: U64,
    pub end_date: U64,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolSummary {
    /// Pool owner.
    pub owner: AccountId,
    /// The total staked balance.
    pub total_staked_balance: U128,
    /// The fraction of the reward that goes to the owner of the staking pool for running the
    /// validator node.
    pub reward_fee_fraction: Ratio,
    /// If reward fee fraction is changing, this will be different from current.
    pub next_reward_fee_fraction: Ratio,
    /// Active farms that affect stakers.
    pub farms: Vec<HumanReadableFarm>,
}
