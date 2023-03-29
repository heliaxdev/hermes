use core::str::FromStr;
use core::time::Duration;
use std::thread;
use std::time::Instant;

use borsh::BorshDeserialize;
use ibc_proto::google::protobuf::Any;
use namada::ledger::parameters::storage as parameter_storage;
use namada::proto::Tx;
use namada::tendermint_rpc::endpoint::broadcast::tx_sync::Response as AbciPlusRpcResponse;
use namada::tendermint_rpc::Client;
use namada::types::token::Amount;
use namada::types::transaction::{Fee, GasLimit, WrapperTx};
use namada_apps::wasm_loader;
use tendermint_rpc::endpoint::broadcast::tx_sync::Response;

use crate::chain::cosmos;
use crate::chain::cosmos::types::tx::{TxStatus, TxSyncResult};
use crate::chain::endpoint::ChainEndpoint;
use crate::chain::requests::{IncludeProof, QueryHeight, QueryTxHash, QueryTxRequest};
use crate::error::Error;

use super::{NamadaChain, WASM_DIR, WASM_FILE};

pub const FEE_TOKEN: &str = "NAM";
const DEFAULT_MAX_GAS: u64 = 100_000;
const WAIT_BACKOFF: Duration = Duration::from_millis(300);

impl NamadaChain {
    pub fn send_tx(&mut self, proto_msg: &Any) -> Result<Response, Error> {
        let tx_code = wasm_loader::read_wasm(WASM_DIR, WASM_FILE).expect("Loading IBC wasm failed");
        let mut tx_data = vec![];
        prost::Message::encode(proto_msg, &mut tx_data)
            .map_err(|e| Error::protobuf_encode(String::from("Message"), e))?;
        let tx = Tx::new(tx_code, Some(tx_data));

        // the wallet should exist because it's confirmed when the bootstrap
        let secret_key = self
            .wallet
            .find_key(&self.config.key_name)
            .map_err(Error::namada_key_pair_not_found)?;
        let signed_tx = tx.sign(&secret_key);

        let fee_token_addr = self
            .wallet
            .find_address(FEE_TOKEN)
            .ok_or_else(|| Error::namada_address_not_found(FEE_TOKEN.to_string()))?
            .clone();

        // fee
        let wrapper_tx_fees_key = parameter_storage::get_wrapper_tx_fees_key();
        let (value, _) = self.query(wrapper_tx_fees_key, QueryHeight::Latest, IncludeProof::No)?;
        let fee_amount = Amount::try_from_slice(&value[..]).map_err(Error::borsh_decode)?;

        let gas_limit = GasLimit::from(self.config.max_gas.unwrap_or(DEFAULT_MAX_GAS));

        let epoch = self.query_epoch()?;
        let wrapper_tx = WrapperTx::new(
            Fee {
                amount: fee_amount,
                token: fee_token_addr,
            },
            &secret_key,
            epoch,
            gas_limit,
            signed_tx,
            Default::default(),
            None,
        );

        let tx = wrapper_tx
            .sign(&secret_key)
            .expect("Signing of the wrapper transaction should not fail");
        let tx_bytes = tx.to_bytes();

        let mut response = self
            .rt
            .block_on(self.rpc_client.broadcast_tx_sync(tx_bytes.into()))
            .map_err(|e| Error::abci_plus_rpc(self.config.rpc_addr.clone(), e))?;
        // overwrite the tx decrypted hash for the tx query
        response.hash = wrapper_tx.tx_hash.into();
        Ok(into_response(response))
    }

    pub fn wait_for_block_commits(
        &self,
        tx_sync_results: &mut [TxSyncResult],
    ) -> Result<(), Error> {
        let start_time = Instant::now();
        loop {
            if cosmos::wait::all_tx_results_found(tx_sync_results) {
                return Ok(());
            }

            let elapsed = start_time.elapsed();
            if elapsed > self.config.rpc_timeout {
                return Err(Error::tx_no_confirmation());
            }

            thread::sleep(WAIT_BACKOFF);

            for TxSyncResult {
                response,
                events,
                status,
            } in tx_sync_results.iter_mut()
            {
                if let TxStatus::Pending { message_count: _ } = status {
                    // If the transaction failed, query_txs returns the IbcEvent::ChainError,
                    // so that we don't attempt to resolve the transaction later on.
                    if let Ok(events_per_tx) =
                        self.query_txs(QueryTxRequest::Transaction(QueryTxHash(response.hash)))
                    {
                        // If we get events back, progress was made, so we replace the events
                        // with the new ones. in both cases we will check in the next iteration
                        // whether or not the transaction was fully committed.
                        if !events_per_tx.is_empty() {
                            *events = events_per_tx;
                            *status = TxStatus::ReceivedResponse;
                        }
                    }
                }
            }
        }
    }
}

/// Convert a broadcast response to one of the base Tendermint
fn into_response(resp: AbciPlusRpcResponse) -> Response {
    Response {
        code: u32::from(resp.code).into(),
        data: Vec::<u8>::from(resp.data).into(),
        log: resp.log.to_string(),
        hash: tendermint::Hash::from_str(&resp.hash.to_string()).unwrap(),
    }
}
