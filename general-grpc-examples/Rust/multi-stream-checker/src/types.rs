use std::fmt;

/// Which of the two transaction streams an event came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamId {
    One,
    Two,
}

impl StreamId {
    /// Short tag used as a prefix on every log line.
    pub fn tag(self) -> &'static str {
        match self {
            StreamId::One => "S1",
            StreamId::Two => "S2",
        }
    }
}

impl fmt::Display for StreamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.tag())
    }
}

/// Events emitted by the three gRPC reader tasks onto the shared channel.
#[derive(Debug)]
pub enum StreamEvent {
    /// A non-vote, non-failed transaction observed on one of the tx streams.
    Transaction {
        stream: StreamId,
        slot: u64,
        signature: String,
        /// Wall-clock arrival, Unix milliseconds. Stamped the moment the update
        /// was pulled off the gRPC stream — before any channel hop — so queueing
        /// inside this process never inflates the measured latency.
        recv_ms: i64,
    },
    /// Block metadata for a slot, carrying the on-chain execution timestamp.
    BlockMeta {
        slot: u64,
        /// `block_time` from blocks_meta, converted to Unix milliseconds.
        /// `None` when the node reported no block time for the slot.
        block_time_ms: Option<i64>,
    },
}
