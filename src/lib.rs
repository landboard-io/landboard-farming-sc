#![no_std]

elrond_wasm::imports!();

mod state;

pub const BLOCK_TIMES: u64 = 6; // 1 block = 6 seconds 
pub const MAX_PERCENT: u64 = 10_000; // 100%

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::derive::contract]
pub trait Farming:
    state::StateModule 
{

    #[init]
    fn init(&self, 
        farming_token_id: TokenIdentifier,
        reward_token_id: TokenIdentifier,
        min_stake_limit: BigUint,
        reward_amount_per_block: BigUint,
        min_claim_amount: BigUint,
        between_claim_timestamp: u64,
        early_unstake_period: u64,
        early_unstake_penalty: u64,
    ) {
        require!(
            farming_token_id.is_valid_esdt_identifier(),
            "invalid farming_token_id"
        );

        require!(
            reward_token_id.is_valid_esdt_identifier(),
            "invalid reward_token_id"
        );

        self.farming_token_id().set(&farming_token_id);
        self.reward_token_id().set(&reward_token_id);
        self.min_stake_limit().set(&min_stake_limit);
        self.reward_amount_per_block().set(&reward_amount_per_block);
        self.min_claim_amount().set(&min_claim_amount);
        self.between_claim_timestamp().set(between_claim_timestamp);
        self.early_unstake_period().set(early_unstake_period);
        self.early_unstake_penalty().set(early_unstake_penalty);

        self.paused().set(0u32);    // live
    }
    
    // owner
    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self,
        #[var_args] opt_token_id: OptionalValue<TokenIdentifier>,
        #[var_args] opt_token_amount: OptionalValue<BigUint>) {
        // if token_id is not given, set it to eGLD
        let token_id = match opt_token_id {
            OptionalValue::Some(v) => v,
            OptionalValue::None => TokenIdentifier::egld()
        };
        // if token_amount is not given, set it to balance of SC - max value to withdraw
        let token_amount = match opt_token_amount {
            OptionalValue::Some(v) => v,
            OptionalValue::None => self.blockchain().get_sc_balance(&token_id, 0)
        };

        self.send().direct(&self.blockchain().get_caller(), &token_id, 0, &token_amount, &[]);
    }

    // endpoint
    #[payable("*")]
    #[endpoint(stakeFarm)]
    fn stake_farm(&self, 
        #[payment_token] farming_token_id: TokenIdentifier, 
        #[payment_amount] stake_amount: BigUint
    ) {
        self.require_activation();

        require!(
            farming_token_id == self.farming_token_id().get(),
            "invalid farming_token_id"
        );
        require!(
            stake_amount >= self.min_stake_limit().get(),
            "cannot stake less than min_stake_limit"
        );

        let caller = self.blockchain().get_caller();

        // update all factors
        self.update_reward(&caller);

        self.total_supply().update(|v| *v += &stake_amount);
        self.balances(&caller).update(|v| *v += &stake_amount);
        self.last_stake_times(&caller).set(self.blockchain().get_block_timestamp());

    }

    #[endpoint(unstakeFarm)]
    fn unstake_farm(&self, 
        #[var_args] opt_unstake_amount: OptionalValue<BigUint>
    ) {
        self.require_activation();

        let caller = self.blockchain().get_caller();

        require!(
            self.balances(&caller).get() > BigUint::zero(),
            "zero balance"
        );

        // update all factors
        self.update_reward(&caller);

        // if unstake_amount is not given, unstake all staked balance
        let mut unstake_amount = match opt_unstake_amount {
            OptionalValue::Some(value) => {
                require!(
                    self.balances(&caller).get() >= value,
                    "unstake_amount cannot be greater than balance"
                );

                value
            },
            OptionalValue::None => self.balances(&caller).get()
        };

        self.total_supply().update(|v| *v -= & unstake_amount
        );
        self.balances(&caller).update(|v| *v -= & unstake_amount
        );

        // check penalty
        if self.last_stake_times(&caller).get() + self.early_unstake_period().get() > self.blockchain().get_block_timestamp() {
            // set penalty
            let penalty_amount = unstake_amount.clone() * BigUint::from(self.early_unstake_penalty().get()) / BigUint::from(MAX_PERCENT);
            unstake_amount = unstake_amount - penalty_amount;
        }

        require!(
            self.blockchain().get_sc_balance(&self.farming_token_id().get(), 0) >= unstake_amount,
            "not enough staking tokens in smart contract"
        );
        
        self.send().direct(&caller, &self.farming_token_id().get(), 0, &unstake_amount, &[]);
    }

    #[endpoint(claimReward)]
    fn claim_reward(&self) {
        self.require_activation();

        let caller = self.blockchain().get_caller();

        // update all factors
        self.update_reward(&caller);

        let reward_amount = self.rewards(&caller).get();

        self.require_claim_activation(&caller);

        self.rewards(&caller).update(|v| *v -= &reward_amount
        );

        self.last_claim_times(&caller).set(self.blockchain().get_block_timestamp());

        require!(
            self.blockchain().get_sc_balance(&self.reward_token_id().get(), 0) >= reward_amount,
            "not enough rewarding tokens in smart contract"
        );
        
        self.send().direct(&caller, &self.reward_token_id().get(), 0, &reward_amount, &[]);
    }

    // private
    #[inline]
    fn update_reward(&self, user_address: &ManagedAddress) {
        self.reward_per_wei_stored().set(&self.get_reward_per_wei());
        self.last_update_time().set(self.blockchain().get_block_timestamp());
        
        self.rewards(user_address).set(self.get_earned(user_address));
        self.user_reward_per_wei_paid(user_address).set(&self.reward_per_wei_stored().get());
    }

    #[inline]
    fn require_activation(&self) {
        require!(
            self.paused().get() == 0u32,
            "staking is not live"
        );
    }

    #[inline]
    fn require_claim_activation(&self, user_address: &ManagedAddress) {

        require!(
            self.rewards(&user_address).get() > self.min_claim_amount().get(),
            "can not claim less than min_claim_amount"
        );

        require!(
            self.last_claim_times(&user_address).get() + self.between_claim_timestamp().get() < self.blockchain().get_block_timestamp(),
            "can claim after between_claim_timestamp"
        )
    }

    // view

    #[view(getRewardPerWei)]
    fn get_reward_per_wei(&self) -> BigUint {
        let reward_per_wei_stored = self.reward_per_wei_stored().get();
        return if self.total_supply().get() == BigUint::zero() {
            reward_per_wei_stored
        } else {
            let block_delta = BigUint::from((self.blockchain().get_block_timestamp() - self.last_update_time().get()) / BLOCK_TIMES);

            reward_per_wei_stored + &block_delta * &self.reward_amount_per_block().get()
        }
    }

    #[view(getEarned)]
    fn get_earned(&self, user_address: &ManagedAddress) -> BigUint {
        let reward_pert_token_delta = &self.get_reward_per_wei() - &self.user_reward_per_wei_paid(user_address).get();
        
        return if self.total_supply().get() == BigUint::zero() {
            BigUint::zero()
        } else {
            reward_pert_token_delta * &self.balances(user_address).get() / &self.total_supply().get() + &self.rewards(user_address).get()
        }
    }

    #[view(getRewardApr)]
    fn get_reward_apr(&self, user_address: &ManagedAddress) -> BigUint {
        let apr = BigUint::from(MAX_PERCENT) * &self.balances(user_address).get() / &self.total_supply().get();

        apr
    }

}
