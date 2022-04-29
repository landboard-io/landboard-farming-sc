elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StateModule {
    // owner
    #[only_owner]
    #[endpoint(setFarmingTokenId)]
    fn set_farming_token_id(&self, farming_token_id: TokenIdentifier) {
        require!(
            farming_token_id.is_valid_esdt_identifier(),
            "invalid farming_token_id"
        );

        self.farming_token_id().set(&farming_token_id);
    }

    #[only_owner]
    #[endpoint(setRewardTokenId)]
    fn set_reward_token_id(&self, reward_token_id: TokenIdentifier) {
        require!(
            reward_token_id.is_valid_esdt_identifier(),
            "invalid reward_token_id"
        );

        self.reward_token_id().set(&reward_token_id);
    }

    #[only_owner]
    #[endpoint(setMinStakeLimit)]
    fn set_min_stake_limit(&self, min_stake_limit: BigUint) {
        self.min_stake_limit().set(&min_stake_limit);
    }

    #[only_owner]
    #[endpoint(setRewardAmountPerBlock)]
    fn set_reward_amount_per_block(&self, reward_amount_per_block: BigUint) {
        self.reward_amount_per_block().set(&reward_amount_per_block);
    }

    #[endpoint(setMinClaimAmount)]
    fn set_min_claim_amount(&self, min_claim_amount: BigUint) {
        self.min_claim_amount().set(&min_claim_amount);
    }

    #[only_owner]
    #[endpoint(setBetweenClaimTimestamp)]
    fn set_between_claim_timestamp(&self, between_claim_timestamp: u64) {
        self.between_claim_timestamp().set(&between_claim_timestamp);
    }

    #[only_owner]
    #[endpoint(setEarlyUnstakePeriod)]
    fn set_early_unstake_period(&self, early_unstake_period: u64) {
        self.early_unstake_period().set(&early_unstake_period);
    }

    #[only_owner]
    #[endpoint(setEarlyUnstakePenalty)]
    fn set_early_unstake_penalty(&self, early_unstake_penalty: u64) {
        self.early_unstake_penalty().set(&early_unstake_penalty);
    }

    #[only_owner]
    #[endpoint(setPaused)]
    fn set_paused(&self, paused: u32) {
        self.paused().set(paused);
    }

    // storage
    #[view(getFarmingTokenId)]
    #[storage_mapper("farming_token_id")]
    fn farming_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getRewardTokenId)]
    #[storage_mapper("reward_token_id")]
    fn reward_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getMinStakeLimit)]
    #[storage_mapper("min_stake_limit")]
    fn min_stake_limit(&self) -> SingleValueMapper<BigUint>;

    #[view(getMinClaimAmount)]
    #[storage_mapper("min_claim_amount")]
    fn min_claim_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getBetweenClaimTimestamp)]
    #[storage_mapper("between_claim_timestamp")]
    fn between_claim_timestamp(&self) -> SingleValueMapper<u64>;

    #[view(getEarlyUnstakePeriod)]
    #[storage_mapper("early_unstake_period")]
    fn early_unstake_period(&self) -> SingleValueMapper<u64>;

    #[view(getEarlyUnstakePenalty)]
    #[storage_mapper("early_unstake_penalty")]
    fn early_unstake_penalty(&self) -> SingleValueMapper<u64>;

    #[view(getPaused)]
    #[storage_mapper("paused")]
    fn paused(&self) -> SingleValueMapper<u32>;

    // reward
    #[view(getRewardAmountPerBlock)]
    #[storage_mapper("reward_amount_per_block")]
    fn reward_amount_per_block(&self) -> SingleValueMapper<BigUint>;

    #[view(getLastUpdateTime)]
    #[storage_mapper("last_update_time")]
    fn last_update_time(&self) -> SingleValueMapper<u64>;

    #[view(getRewardPerWeiStored)]
    #[storage_mapper("reward_per_wei_stored")]
    fn reward_per_wei_stored(&self) -> SingleValueMapper<BigUint>;

    // state
    #[view(getTotalSupply)]
    #[storage_mapper("total_supply")]
    fn total_supply(&self) -> SingleValueMapper<BigUint>;

    #[view(getUserRewardPerWeiPaid)]
    #[storage_mapper("user_reward_per_wei_paid")]
    fn user_reward_per_wei_paid(&self, user_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getReward)]
    #[storage_mapper("rewards")]
    fn rewards(&self, user_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getBalance)]
    #[storage_mapper("balances")]
    fn balances(&self, user_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getLastStakeTimes)]
    #[storage_mapper("last_stake_times")]
    fn last_stake_times(&self, user_address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getLastClaimTimes)]
    #[storage_mapper("last_claim_times")]
    fn last_claim_times(&self, user_address: &ManagedAddress) -> SingleValueMapper<u64>;
}