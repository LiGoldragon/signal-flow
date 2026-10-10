pub mod generated;
pub use generated::signal::*;

pub use flow_ethos::{
    Address, FlowAspect, FlowId, HarnessKind, Key, Layer, Lock, MetaSocketPath, Metaflow,
    ModelName, OrdinarySocketPath, Process, Recipient, Request, Said, Sender, Source, Start,
    Subaspect, Topic,
};

pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
pub const WIRE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The contract is identified on the wire by the digest of its authored
/// Ethos source; the querying side greets with it.
impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
