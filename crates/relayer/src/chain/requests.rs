use core::fmt::{self, Display};

use crate::error::Error;

use ibc_proto::{
    cosmos::base::query::v1beta1::PageRequest as RawPageRequest,
    ibc::core::{
        channel::v1::{
            QueryChannelClientStateRequest as RawQueryChannelClientStateRequest,
            QueryChannelRequest as RawQueryChannelRequest,
            QueryChannelsRequest as RawQueryChannelsRequest,
            QueryConnectionChannelsRequest as RawQueryConnectionChannelsRequest,
            QueryNextSequenceReceiveRequest as RawQueryNextSequenceReceiveRequest,
            QueryPacketAcknowledgementRequest as RawQueryPacketAcknowledgementRequest,
            QueryPacketAcknowledgementsRequest as RawQueryPacketAcknowledgementsRequest,
            QueryPacketCommitmentRequest as RawQueryPacketCommitmentRequest,
            QueryPacketCommitmentsRequest as RawQueryPacketCommitmentsRequest,
            QueryPacketReceiptRequest as RawQueryPacketReceiptRequest,
            QueryUnreceivedAcksRequest as RawQueryUnreceivedAcksRequest,
            QueryUnreceivedPacketsRequest as RawQueryUnreceivedPacketsRequest,
        },
        client::v1::{
            QueryClientStateRequest as RawQueryClientStateRequest,
            QueryClientStatesRequest as RawQueryClientStatesRequest,
            QueryConsensusStateHeightsRequest as RawQueryConsensusStateHeightsRequest,
            QueryConsensusStateRequest as RawQueryConsensusStateRequest,
            QueryConsensusStatesRequest as RawQueryConsensusStatesRequest,
        },
        connection::v1::{
            QueryClientConnectionsRequest as RawQueryClientConnectionsRequest,
            QueryConnectionRequest as RawQueryConnectionRequest,
            QueryConnectionsRequest as RawQueryConnectionsRequest,
        },
    },
};
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId,
};
use ibc_relayer_types::events::WithBlockDataType;
use ibc_relayer_types::Height;

use serde::{Deserialize, Serialize};
use tendermint::block::Height as TMBlockHeight;
use tendermint::Hash as TxHash;
use tonic::metadata::AsciiMetadataValue;

/// Type to specify a height in a query. Specifically, this caters to the use
/// case where the user wants to query at whatever the latest height is, as
/// opposed to specifying a specific height.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum QueryHeight {
    Latest,
    Specific(Height),
}

impl From<QueryHeight> for TMBlockHeight {
    fn from(height_query: QueryHeight) -> Self {
        match height_query {
            QueryHeight::Latest => Self::from(0_u32),
            QueryHeight::Specific(height) => Self::from(height),
        }
    }
}

impl TryFrom<QueryHeight> for AsciiMetadataValue {
    type Error = Error;

    fn try_from(height_query: QueryHeight) -> Result<Self, Self::Error> {
        let height = match height_query {
            QueryHeight::Latest => 0u64,
            QueryHeight::Specific(height) => height.revision_height(),
        };

        str::parse(&height.to_string()).map_err(Error::invalid_metadata)
    }
}

impl Display for QueryHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryHeight::Latest => write!(f, "latest height"),
            QueryHeight::Specific(height) => write!(f, "{height}"),
        }
    }
}

/// Defines a type to be used in select requests to specify whether or not a proof should be
/// returned along with the response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncludeProof {
    Yes,
    No,
}

impl IncludeProof {
    pub fn to_bool(&self) -> bool {
        *self == IncludeProof::Yes
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PageRequest {
    /// key is a value returned in PageResponse.next_key to begin
    /// querying the next page most efficiently. Only one of offset or key
    /// should be set.
    pub key: Vec<u8>,
    /// offset is a numeric offset that can be used when key is unavailable.
    /// It is less efficient than using key. Only one of offset or key should
    /// be set.
    pub offset: u64,
    /// limit is the total number of results to be returned in the result page.
    /// If left empty it will default to a value to be set by each app.
    pub limit: u64,
    /// count_total is set to true  to indicate that the result set should include
    /// a count of the total number of items available for pagination in UIs.
    /// count_total is only respected when offset is used. It is ignored when key
    /// is set.
    pub count_total: bool,
    /// reverse is set to true if results are to be returned in the descending order.
    pub reverse: bool,
}

impl PageRequest {
    pub fn all() -> Self {
        // Note: do not use u64::MAX as the limit, as it may have unintended consequences
        // See https://github.com/informalsystems/hermes/pull/2950#issuecomment-1373733744

        Self::per_page(u32::MAX as u64)
    }

    pub fn per_page(limit: u64) -> Self {
        PageRequest {
            limit,
            ..Default::default()
        }
    }
}

impl From<PageRequest> for RawPageRequest {
    fn from(request: PageRequest) -> Self {
        RawPageRequest {
            key: request.key,
            offset: request.offset,
            limit: request.limit,
            count_total: request.count_total,
            reverse: request.reverse,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Paginate {
    #[default]
    All,

    PerPage {
        per_page: u64,
        total: u64,
    },
}

impl Paginate {
    pub fn is_enabled(&self) -> bool {
        !matches!(self, Self::All)
    }

    pub fn get_values(&self) -> (u64, u64) {
        match self {
            Paginate::PerPage { total, per_page } => (*per_page, *total),
            _ => (0, 0),
        }
    }
}

impl From<Paginate> for PageRequest {
    fn from(value: Paginate) -> Self {
        match value {
            Paginate::All => PageRequest::all(),
            Paginate::PerPage { per_page, .. } => PageRequest::per_page(per_page),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryClientStateRequest {
    pub client_id: ClientId,
    pub height: QueryHeight,
}

impl From<QueryClientStateRequest> for RawQueryClientStateRequest {
    fn from(request: QueryClientStateRequest) -> Self {
        Self {
            client_id: request.client_id.to_string(),
        }
    }
}

/// gRPC query to fetch all client states associated with the chain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryClientStatesRequest {
    pub pagination: Option<PageRequest>,
}

impl From<QueryClientStatesRequest> for RawQueryClientStatesRequest {
    fn from(request: QueryClientStatesRequest) -> Self {
        RawQueryClientStatesRequest {
            pagination: request.pagination.map(|pagination| pagination.into()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryConsensusStateRequest {
    pub client_id: ClientId,
    pub consensus_height: Height,
    pub query_height: QueryHeight,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryUpgradedClientStateRequest {
    /// Height at which the chain is scheduled to halt for upgrade
    pub upgrade_height: Height,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryUpgradedConsensusStateRequest {
    /// Height at which the chain is scheduled to halt for upgrade.
    pub upgrade_height: Height,
}

/// gRPC query to fetch all consensus states associated with the specified client.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryConsensusStatesRequest {
    pub client_id: ClientId,
    pub pagination: Option<PageRequest>,
}

impl From<QueryConsensusStatesRequest> for RawQueryConsensusStatesRequest {
    fn from(request: QueryConsensusStatesRequest) -> Self {
        RawQueryConsensusStatesRequest {
            client_id: request.client_id.to_string(),
            pagination: request.pagination.map(|pagination| pagination.into()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryConsensusStateHeightsRequest {
    pub client_id: ClientId,
    pub pagination: Option<PageRequest>,
}

impl From<QueryConsensusStateHeightsRequest> for RawQueryConsensusStateHeightsRequest {
    fn from(request: QueryConsensusStateHeightsRequest) -> Self {
        RawQueryConsensusStateHeightsRequest {
            client_id: request.client_id.to_string(),
            pagination: request.pagination.map(|pagination| pagination.into()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryConnectionsRequest {
    pub pagination: Option<PageRequest>,
}

impl From<QueryConnectionsRequest> for RawQueryConnectionsRequest {
    fn from(request: QueryConnectionsRequest) -> Self {
        RawQueryConnectionsRequest {
            pagination: request.pagination.map(|pagination| pagination.into()),
        }
    }
}

/// gRPC query to fetch all the connections associated with the specified client.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryClientConnectionsRequest {
    pub client_id: ClientId,
}

impl From<QueryClientConnectionsRequest> for RawQueryClientConnectionsRequest {
    fn from(request: QueryClientConnectionsRequest) -> Self {
        RawQueryClientConnectionsRequest {
            client_id: request.client_id.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryConnectionRequest {
    pub connection_id: ConnectionId,
    pub height: QueryHeight,
}

impl From<QueryConnectionRequest> for RawQueryConnectionRequest {
    fn from(request: QueryConnectionRequest) -> Self {
        Self {
            connection_id: request.connection_id.to_string(),
        }
    }
}

/// gRPC query to fetch all channels associated with the specified connection.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryConnectionChannelsRequest {
    pub connection_id: ConnectionId,
    pub pagination: Option<PageRequest>,
}

impl From<QueryConnectionChannelsRequest> for RawQueryConnectionChannelsRequest {
    fn from(request: QueryConnectionChannelsRequest) -> Self {
        RawQueryConnectionChannelsRequest {
            connection: request.connection_id.to_string(),
            pagination: request.pagination.map(|pagination| pagination.into()),
        }
    }
}

/// gRPC query to fetch all channels of the chain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryChannelsRequest {
    pub pagination: Option<PageRequest>,
}

impl From<QueryChannelsRequest> for RawQueryChannelsRequest {
    fn from(request: QueryChannelsRequest) -> Self {
        RawQueryChannelsRequest {
            pagination: request.pagination.map(|pagination| pagination.into()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryChannelRequest {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub height: QueryHeight,
}

impl From<QueryChannelRequest> for RawQueryChannelRequest {
    fn from(request: QueryChannelRequest) -> Self {
        Self {
            port_id: request.port_id.to_string(),
            channel_id: request.channel_id.to_string(),
        }
    }
}

/// gRPC request to fetch the client state associated with a specified channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryChannelClientStateRequest {
    pub port_id: PortId,
    pub channel_id: ChannelId,
}

impl From<QueryChannelClientStateRequest> for RawQueryChannelClientStateRequest {
    fn from(request: QueryChannelClientStateRequest) -> Self {
        RawQueryChannelClientStateRequest {
            port_id: request.port_id.to_string(),
            channel_id: request.channel_id.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryPacketCommitmentRequest {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub sequence: Sequence,
    pub height: QueryHeight,
}

impl From<QueryPacketCommitmentRequest> for RawQueryPacketCommitmentRequest {
    fn from(request: QueryPacketCommitmentRequest) -> Self {
        RawQueryPacketCommitmentRequest {
            port_id: request.port_id.to_string(),
            channel_id: request.channel_id.to_string(),
            sequence: request.sequence.into(),
        }
    }
}

/// gRPC query to fetch the packet commitment hashes associated with the specified channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryPacketCommitmentsRequest {
    pub query_height: QueryHeight,
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub pagination: Paginate,
}

impl From<QueryPacketCommitmentsRequest> for RawQueryPacketCommitmentsRequest {
    fn from(request: QueryPacketCommitmentsRequest) -> Self {
        RawQueryPacketCommitmentsRequest {
            port_id: request.port_id.to_string(),
            channel_id: request.channel_id.to_string(),
            pagination: Some(PageRequest::from(request.pagination).into()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryPacketReceiptRequest {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub sequence: Sequence,
    pub height: QueryHeight,
}

impl From<QueryPacketReceiptRequest> for RawQueryPacketReceiptRequest {
    fn from(request: QueryPacketReceiptRequest) -> Self {
        Self {
            port_id: request.port_id.to_string(),
            channel_id: request.channel_id.to_string(),
            sequence: request.sequence.as_u64(),
        }
    }
}

/// gRPC query to fetch all unreceived packet sequences associated with the specified channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryUnreceivedPacketsRequest {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub packet_commitment_sequences: Vec<Sequence>,
}

impl From<QueryUnreceivedPacketsRequest> for RawQueryUnreceivedPacketsRequest {
    fn from(request: QueryUnreceivedPacketsRequest) -> Self {
        RawQueryUnreceivedPacketsRequest {
            port_id: request.port_id.to_string(),
            channel_id: request.channel_id.to_string(),
            packet_commitment_sequences: request
                .packet_commitment_sequences
                .into_iter()
                .map(|seq| seq.into())
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryPacketAcknowledgementRequest {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub sequence: Sequence,
    pub height: QueryHeight,
}

impl From<QueryPacketAcknowledgementRequest> for RawQueryPacketAcknowledgementRequest {
    fn from(request: QueryPacketAcknowledgementRequest) -> Self {
        RawQueryPacketAcknowledgementRequest {
            port_id: request.port_id.to_string(),
            channel_id: request.channel_id.to_string(),
            sequence: request.sequence.as_u64(),
        }
    }
}

/// gRPC query to fetch all packet acknowledgements associated with the specified channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryPacketAcknowledgementsRequest {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub pagination: Paginate,
    pub packet_commitment_sequences: Vec<Sequence>,
}

impl From<QueryPacketAcknowledgementsRequest> for RawQueryPacketAcknowledgementsRequest {
    fn from(request: QueryPacketAcknowledgementsRequest) -> Self {
        RawQueryPacketAcknowledgementsRequest {
            port_id: request.port_id.to_string(),
            channel_id: request.channel_id.to_string(),
            pagination: Some(PageRequest::from(request.pagination).into()),
            packet_commitment_sequences: request
                .packet_commitment_sequences
                .into_iter()
                .map(|seq| seq.into())
                .collect(),
        }
    }
}

/// gRPC query to fetch the unreceived acknowledgements sequences associated with
/// the specified channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryUnreceivedAcksRequest {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub packet_ack_sequences: Vec<Sequence>,
}

impl From<QueryUnreceivedAcksRequest> for RawQueryUnreceivedAcksRequest {
    fn from(request: QueryUnreceivedAcksRequest) -> Self {
        RawQueryUnreceivedAcksRequest {
            port_id: request.port_id.to_string(),
            channel_id: request.channel_id.to_string(),
            packet_ack_sequences: request
                .packet_ack_sequences
                .into_iter()
                .map(|seq| seq.into())
                .collect(),
        }
    }
}

/// gRPC query to fetch the sequence number of the next packet to be
/// received at the given height by the specified channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryNextSequenceReceiveRequest {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub height: QueryHeight,
}

impl From<QueryNextSequenceReceiveRequest> for RawQueryNextSequenceReceiveRequest {
    fn from(request: QueryNextSequenceReceiveRequest) -> Self {
        RawQueryNextSequenceReceiveRequest {
            port_id: request.port_id.to_string(),
            channel_id: request.channel_id.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryHostConsensusStateRequest {
    pub height: QueryHeight,
}

impl From<QueryConsensusStateRequest> for RawQueryConsensusStateRequest {
    fn from(request: QueryConsensusStateRequest) -> Self {
        Self {
            client_id: request.client_id.to_string(),
            revision_number: request.consensus_height.revision_number(),
            revision_height: request.consensus_height.revision_height(),
            latest_height: matches!(request.query_height, QueryHeight::Latest),
        }
    }
}

/// Used for queries and not yet standardized in channel's query.proto
#[derive(Clone, Debug)]
pub enum QueryTxRequest {
    Client(QueryClientEventRequest),
    Transaction(QueryTxHash),
}

#[derive(Clone, Debug)]
pub struct QueryTxHash(pub TxHash);

/// Used to query packet events:
/// - for events of type `event_id`,
/// - for a specific channel
/// - with sequences in `sequences`
/// - that occurred at a height either smaller or equal to `height` or exactly at `height`,
///   as specified by `event_height_qualifier`
#[derive(Clone, Debug)]
pub struct QueryPacketEventDataRequest {
    pub event_id: WithBlockDataType,
    pub source_channel_id: ChannelId,
    pub source_port_id: PortId,
    pub destination_channel_id: ChannelId,
    pub destination_port_id: PortId,
    pub sequences: Vec<Sequence>,
    pub height: Qualified<QueryHeight>,
}

/// Refines an inner type by assigning it to refer to either a:
///     - range of values (when using variant `SmallerEqual`), or
///     - to a specific value (with variant `Equal`).
///
/// For example, the inner type is typically a [`QueryHeight`].
/// In this case, we can capture and handle the two separate cases
/// that can appear when we want to query for packet event data,
/// depending on the request: The request might refer to a specific
/// height (i.e., we want packets from a block _at height_ T), or to
/// a range of heights (i.e., all packets _up to height_ T).
#[derive(Clone, Copy, Debug)]
pub enum Qualified<T> {
    SmallerEqual(T),
    Equal(T),
}

impl<T> Qualified<T> {
    /// Access the inner type.
    pub fn get(self) -> T {
        match self {
            Qualified::SmallerEqual(t) => t,
            Qualified::Equal(t) => t,
        }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Qualified<U> {
        match self {
            Qualified::SmallerEqual(t) => Qualified::SmallerEqual(f(t)),
            Qualified::Equal(t) => Qualified::Equal(f(t)),
        }
    }
}

impl<T: Display> Display for Qualified<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Qualified::SmallerEqual(a) => write!(f, "<={a}"),
            Qualified::Equal(a) => write!(f, "=={a}"),
        }
    }
}

/// Query request for a single client event, identified by `event_id`, for `client_id`.
#[derive(Clone, Debug)]
pub struct QueryClientEventRequest {
    pub query_height: QueryHeight,
    pub event_id: WithBlockDataType,
    pub client_id: ClientId,
    pub consensus_height: Height,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CrossChainQueryRequest {
    pub chain_id: ChainId,
    pub query_id: String,
    pub query_type: String,
    /// hex encoded query request
    pub request: String,
    pub height: TMBlockHeight,
}
