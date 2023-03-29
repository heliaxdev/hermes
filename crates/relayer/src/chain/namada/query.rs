use core::str::FromStr;

use ibc_relayer_types::core::ics23_commitment::merkle::convert_tm_to_ics_merkle_proof;
use ibc_relayer_types::core::ics23_commitment::merkle::MerkleProof;
use ibc_relayer_types::events::{IbcEvent, IbcEventType};
use ibc_relayer_types::Height as ICSHeight;
use namada::ledger::queries::RPC;
use namada::tendermint_rpc::query::Query as AbciPlusQuery;
use namada::tendermint_rpc::{Client, Order};
use namada::types::storage::{BlockHeight, Epoch, Key, PrefixValue};
use tendermint::abci::{Event, EventAttribute};
use tendermint::merkle::proof::{ProofOp, ProofOps};
use tendermint_rpc::query::Query;

use crate::chain::requests::QueryHeight;
use crate::error::Error;
use crate::event::{ibc_event_try_from_abci_event, IbcEventWithHeight};

use super::NamadaChain;
use crate::chain::endpoint::ChainEndpoint;
use crate::chain::requests::IncludeProof;

impl NamadaChain {
    pub fn query(
        &self,
        key: Key,
        height: QueryHeight,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        let height = match height {
            QueryHeight::Latest => None,
            QueryHeight::Specific(h) => Some(BlockHeight(h.revision_height())),
        };
        let is_proven = matches!(include_proof, IncludeProof::Yes);
        let response = self
            .rt
            .block_on(
                RPC.shell()
                    .storage_value(&self.rpc_client, None, height, is_proven, &key),
            )
            .map_err(Error::namada_query)?;

        let proof = if is_proven {
            let proof_ops = response.proof.ok_or_else(Error::empty_response_proof)?;
            // convert MerkleProof to one of the base tendermint
            let ops: Vec<ProofOp> = proof_ops
                .ops
                .iter()
                .map(|proof_op| ProofOp {
                    field_type: proof_op.field_type.clone(),
                    key: proof_op.key.clone(),
                    data: proof_op.data.clone(),
                })
                .collect();
            let tm_proof_ops = ProofOps { ops };
            let proof = convert_tm_to_ics_merkle_proof(&tm_proof_ops).map_err(Error::ics23)?;
            Some(proof)
        } else {
            None
        };

        Ok((response.data, proof))
    }

    pub fn query_prefix(&self, prefix: Key) -> Result<Vec<PrefixValue>, Error> {
        let response = self
            .rt
            .block_on(
                RPC.shell()
                    .storage_prefix(&self.rpc_client, None, None, false, &prefix),
            )
            .map_err(Error::namada_query)?;
        Ok(response.data)
    }

    pub fn query_epoch(&self) -> Result<Epoch, Error> {
        self.rt
            .block_on(RPC.shell().epoch(&self.rpc_client))
            .map_err(Error::namada_query)
    }

    pub fn query_events(&self, query: Query) -> Result<Vec<IbcEventWithHeight>, Error> {
        crate::time!("query_block: query block packet events");
        crate::telemetry!(query, self.id(), "query_block");

        let query = AbciPlusQuery::from_str(&query.to_string()).unwrap();
        let blocks = &self
            .rt
            .block_on(self.rpc_client.block_search(query, 1, 1, Order::Ascending))
            .map_err(|e| Error::abci_plus_rpc(self.config.rpc_addr.clone(), e))?
            .blocks;
        let block = match blocks.get(0) {
            Some(b) => &b.block,
            // transaction is not committed yet
            None => return Ok(vec![]),
        };
        let response = self
            .rt
            .block_on(self.rpc_client.block_results(block.header.height))
            .map_err(|e| Error::abci_plus_rpc(self.config.rpc_addr.clone(), e))?;

        let events = response
            .end_block_events
            .ok_or_else(|| Error::query("No transaction result was found".to_string()))?;
        let mut ibc_events = vec![];
        for event in &events {
            let height = ICSHeight::new(self.id().version(), u64::from(response.height)).unwrap();
            let event = into_abci_event(event);
            match ibc_event_try_from_abci_event(&event) {
                Ok(e) => ibc_events.push(IbcEventWithHeight::new(e, height)),
                Err(err) => {
                    // skip AppModule, ReceivePacket, and Message event
                    if event.kind == "app_module"
                        || event.kind == IbcEventType::ReceivePacket.as_str()
                        || event.kind == "message"
                    {
                        continue;
                    }
                    let success_code_tag = EventAttribute {
                        key: "code".to_string(),
                        value: "0".to_string(),
                        index: true,
                    };
                    if !event.attributes.contains(&success_code_tag) {
                        let ibc_event = IbcEventWithHeight::new(
                            IbcEvent::ChainError(format!(
                                "The transaction was invalid: event {:?} error {}",
                                event, err
                            )),
                            height,
                        );
                        ibc_events.push(ibc_event);
                    }
                }
            }
        }
        Ok(ibc_events)
    }
}

fn into_abci_event(event: &namada::tendermint::abci::Event) -> Event {
    let attributes = event
        .attributes
        .iter()
        .map(|tag| EventAttribute {
            key: tag.key.to_string(),
            value: tag.value.to_string(),
            index: true,
        })
        .collect();

    Event {
        kind: event.type_str.clone(),
        attributes,
    }
}
