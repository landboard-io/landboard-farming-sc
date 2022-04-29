##### - configuration - #####
PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"

WALLET="./wallets/devnet.pem"

################################################
TOKEN_ID="SVEN-de92d2"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"
TOKEN_ID_ONLY_HEX="$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

MIN_STAKE_LIMIT=100000000000000000000                       # 100 SVEN
REWARD_AMOUNT_PER_BLOCK=500000000000000000                  # 0.5 SVEN per block
MIN_CLAIM_AMOUNT=10000000000000000000                       # 10 SVEN
BETWEEN_CLAIM_TIMESTAMP=1800                                # 30 minutes
EARLY_UNSTAKE_PERIOD=172800                                 # 2 days
EARLY_UNSTAKE_PENALTY=100                                   # 1%
PAUSED=1

STAKE="stakeFarm"
STAKE_ONLY_HEX="$(echo -n ${STAKE} | xxd -p -u | tr -d '\n')"

CALLER_ADDRESS="erd149axj8feledcw7zck5f3ecwrncgd0gemcr9q69yxqlk0zvnl5zvs065jqu"
CALLER_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"

################################################
ADDRESS=$(erdpy data load --key=address-devnet)
TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
################################################

deploy() {
    erdpy --verbose contract deploy \
    --project=${PROJECT} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=50000000 \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_ID_HEX} ${MIN_STAKE_LIMIT} ${REWARD_AMOUNT_PER_BLOCK} ${MIN_CLAIM_AMOUNT} ${BETWEEN_CLAIM_TIMESTAMP} ${EARLY_UNSTAKE_PERIOD} ${EARLY_UNSTAKE_PENALTY} \
    --send \
    --outfile="deploy-devnet.interaction.json" \
    --proxy=${PROXY} \
    --metadata-payable \
    --metadata-payable-by-sc \
    --chain=${CHAIN_ID} || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

stakeFarm() {
    erdpy --verbose tx new --receiver ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 \
    --data="ESDTTransfer@${TOKEN_ID_ONLY_HEX}@056bc75e2d63100000@${STAKE_ONLY_HEX}" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

unstakeFarm() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="unstakeFarm" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

claimReward() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="claimReward" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

withdraw() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="withdraw" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# setLive() {
#     erdpy --verbose contract call ${ADDRESS} \
#     --recall-nonce --pem=${WALLET} \
#     --gas-limit=6000000 \
#     --function="setPaused" \
#     --arguments 0 \
#     --send --proxy=${PROXY} --chain=${CHAIN_ID}
# }

# setPaused() {
#     erdpy --verbose contract call ${ADDRESS} \
#     --recall-nonce --pem=${WALLET} \
#     --gas-limit=6000000 \
#     --function="setPaused" \
#     --arguments 1 \
#     --send --proxy=${PROXY} --chain=${CHAIN_ID}
# }

# setEarlyUnstakePenalty() {
#     erdpy --verbose contract call ${ADDRESS} \
#     --recall-nonce --pem=${WALLET} \
#     --gas-limit=6000000 \
#     --function="setEarlyUnstakePenalty" \
#     --arguments 5000 \
#     --send --proxy=${PROXY} --chain=${CHAIN_ID}
# }

# setEarlyUnstakePeriod() {
#     erdpy --verbose contract call ${ADDRESS} \
#     --recall-nonce --pem=${WALLET} \
#     --gas-limit=6000000 \
#     --function="setEarlyUnstakePeriod" \
#     --arguments 600 \
#     --send --proxy=${PROXY} --chain=${CHAIN_ID}
# }

# view
# getRewardPerWei() {
#     erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getRewardPerWei"
# }

getEarned() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getEarned" --arguments ${CALLER_ADDRESS_HEX}
}

# getPaused() {
#     erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getPaused"
# }

getTotalSupply() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getTotalSupply"
}

getReward() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getReward" --arguments ${CALLER_ADDRESS_HEX}
}

getBalance() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getBalance" --arguments ${CALLER_ADDRESS_HEX}
}

getRewardApr() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getRewardApr" --arguments ${CALLER_ADDRESS_HEX}
}

# getLastStakeTimes() {
#     erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getLastStakeTimes" --arguments ${CALLER_ADDRESS_HEX}
# }

getRewardAmountPerBlock() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getRewardAmountPerBlock"
}

getEarlyUnstakePenalty() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getEarlyUnstakePenalty"
}

getEarlyUnstakePeriod() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getEarlyUnstakePeriod"
}

getUserRewardPerWeiPaid() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getUserRewardPerWeiPaid" --arguments ${CALLER_ADDRESS_HEX}
}