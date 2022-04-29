////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    farming_sc
    (
        claimReward
        getBalance
        getBetweenClaimTimestamp
        getEarlyUnstakePenalty
        getEarlyUnstakePeriod
        getEarned
        getFarmingTokenId
        getLastClaimTimes
        getLastStakeTimes
        getLastUpdateTime
        getMinClaimAmount
        getMinStakeLimit
        getPaused
        getReward
        getRewardAmountPerBlock
        getRewardApr
        getRewardPerWei
        getRewardPerWeiStored
        getRewardTokenId
        getTotalSupply
        getUserRewardPerWeiPaid
        setBetweenClaimTimestamp
        setEarlyUnstakePenalty
        setEarlyUnstakePeriod
        setFarmingTokenId
        setMinClaimAmount
        setMinStakeLimit
        setPaused
        setRewardAmountPerBlock
        setRewardTokenId
        stakeFarm
        unstakeFarm
        withdraw
    )
}

elrond_wasm_node::wasm_empty_callback! {}
