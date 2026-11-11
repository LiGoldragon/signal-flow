pub mod generated;
pub use generated::signal::*;

pub use flow_ethos::{
    Address, AnswerableTask, Aspect, Context, DonePayload, FlowId, HarnessKind, Key, Lock,
    LogicalDeliveryId, Message, MetaSocketPath, Metaflow, Model, ModelName, NativeInput,
    OrdinarySocketPath, Process, QueueEntry, Request, RequestId, ResultPayload, SentBy, Source,
    Start, Subaspect, Topic, Verbatim,
};

pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
pub const WIRE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The contract is identified on the wire by the digest of its authored
/// Ethos source; the querying side greets with it.
impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
