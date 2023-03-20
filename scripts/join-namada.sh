#!/bin/bash

set -e

NAMADA_DIR=$1
CHAIN_ID_A=$2
CHAIN_ID_B=$3
if [ -z "${NAMADA_DIR}" ] || [ -z "${CHAIN_ID_A}" ] || [ -z "${CHAIN_ID_B}" ]
then
  echo "Usage: join-namada.sh NAMADA_DIR CHAIN_ID_A CHAIN_ID_B"
  exit 1
fi

cd $(dirname $0)
IBC_RS_DIR=${PWD%/scripts*}

NAMADAC=${NAMADA_DIR}/target/release/namadac
NAMADAN=${NAMADA_DIR}/target/release/namadan
NAMADAW=${NAMADA_DIR}/target/release/namadaw

BASE_DIR_A=${IBC_RS_DIR}/data/namada-a/.namada
BASE_DIR_B=${IBC_RS_DIR}/data/namada-b/.namada

HERMES_CONFIG_TEMPLATE="
[global]
log_level = 'info'

[mode]

[mode.clients]
enabled = true
refresh = true
misbehaviour = true

[mode.connections]
enabled = false

[mode.channels]
enabled = false

[mode.packets]
enabled = true
clear_interval = 10
clear_on_start = false
tx_confirmation = true

[telemetry]
enabled = false
host = '127.0.0.1'
port = 3001

[[chains]]
id = '_CHAIN_ID_A_'
type = 'namada'
rpc_addr = 'http://127.0.0.1:26657'
grpc_addr = 'http://127.0.0.1:9090'
websocket_addr = 'ws://127.0.0.1:26657/websocket'
account_prefix = ''
key_name = 'relayer'
store_prefix = 'ibc'
gas_price = { price = 0.001, denom = 'stake' }

[[chains]]
id = '_CHAIN_ID_B_'
type = 'namada'
rpc_addr = 'http://127.0.0.1:27657'
grpc_addr = 'http://127.0.0.1:9090'
websocket_addr = 'ws://127.0.0.1:27657/websocket'
account_prefix = 'cosmos'
key_name = 'relayer'
store_prefix = 'ibc'
gas_price = { price = 0.001, denom = 'nam' }
"

# Make the base directories
mkdir -p ${BASE_DIR_A}
mkdir -p ${BASE_DIR_B}

# Join the network
${NAMADAC} --base-dir ${BASE_DIR_A} utils join-network --chain-id ${CHAIN_ID_A}
${NAMADAC} --base-dir ${BASE_DIR_B} utils join-network --chain-id ${CHAIN_ID_B}

# Run ledger B temporarily for making tendermint config
${NAMADAN} --base-dir ${BASE_DIR_B} ledger run > /dev/null 2>&1 &
pid=$!
sleep 5
kill ${pid}

# Replace the default port number for instance B
cat ${BASE_DIR_B}/${CHAIN_ID_B}/config.toml \
  | sed \
  -e "s/127.0.0.1:26657/127.0.0.1:27657/g" \
  -e "s/127.0.0.1:26658/127.0.0.1:27658/g" \
  -e "s/0.0.0.0:26656/0.0.0.0:27656/g" \
  -e "s/127.0.0.1:26661/127.0.0.1:27661/g" \
  > tmp.toml
mv tmp.toml ${BASE_DIR_B}/${CHAIN_ID_B}/config.toml
cat ${BASE_DIR_B}/${CHAIN_ID_B}/tendermint/config/config.toml \
  | sed \
  -e "s/127.0.0.1:26658/127.0.0.1:27658/g" \
  -e "s/127.0.0.1:26657/127.0.0.1:27657/g" \
  -e "s/0.0.0.0:26656/0.0.0.0:27656/g" \
  -e "s/127.0.0.1:26661/127.0.0.1:27661/g" \
  > tmp.toml
mv tmp.toml ${BASE_DIR_B}/${CHAIN_ID_B}/tendermint/config/config.toml

# Run ledgers
${NAMADAN} --base-dir ${BASE_DIR_A} ledger run > ${BASE_DIR_A}/namada.log 2>&1 &
${NAMADAN} --base-dir ${BASE_DIR_B} ledger run > ${BASE_DIR_B}/namada.log 2>&1 &

# Make each relayer account
${NAMADAW} --base-dir ${BASE_DIR_A} key gen --alias relayer --unsafe-dont-encrypt
${NAMADAW} --base-dir ${BASE_DIR_B} key gen --alias relayer --unsafe-dont-encrypt

# Copy wasm
mkdir -p ${IBC_RS_DIR}/namada_wasm
cp -f ${NAMADA_DIR}/wasm/checksums.json ${IBC_RS_DIR}/namada_wasm
cp -f ${NAMADA_DIR}/wasm/tx_ibc.*.wasm ${IBC_RS_DIR}/namada_wasm

# Copy wallets
mkdir -p ${IBC_RS_DIR}/namada_wallet/${CHAIN_ID_A}
mkdir -p ${IBC_RS_DIR}/namada_wallet/${CHAIN_ID_B}
cp ${BASE_DIR_A}/${CHAIN_ID_A}/wallet.toml ${IBC_RS_DIR}/namada_wallet/${CHAIN_ID_A}
cp ${BASE_DIR_B}/${CHAIN_ID_B}/wallet.toml ${IBC_RS_DIR}/namada_wallet/${CHAIN_ID_B}

# Make Hermes config
echo "${HERMES_CONFIG_TEMPLATE}" \
  | sed -e "s/_CHAIN_ID_A_/${CHAIN_ID_A}/g" -e "s/_CHAIN_ID_B_/${CHAIN_ID_B}/g" \
  > ${IBC_RS_DIR}/config_for_namada.toml

echo "
Namada data and logs are under ${IBC_RS_DIR}/data/namada-*/.namada"
echo "After the sync, you can create a channel and start Hermes process
"
echo "Command to create a channel:
hermes -c ${IBC_RS_DIR}/config_for_namada.toml create channel ${CHAIN_ID_A} --chain-b ${CHAIN_ID_B} --port-a transfer --port-b transfer --new-client-connection
"
echo "Command to start Hermes to relay packets:
hermes -c ${IBC_RS_DIR}/config_for_namada.toml start
"
