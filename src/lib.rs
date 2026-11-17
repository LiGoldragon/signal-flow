// The generated modules name this crate's own types by its name, as a
// dependent would: the Signal names its Transition `signal_flow::transition`.
extern crate self as signal_flow;

pub mod generated;
pub use generated::signal::*;
/// A flow's transitions (`ethos/transition.ethos`): the state entered, its
/// cause and its moment, as the Flow Nexus records them and Subscribe
/// carries them.
pub use generated::transition;

pub use flow_ethos::{
    Address, AnswerableTask, Aspect, Context, DonePayload, FlowId, HarnessKind, Key, Lock,
    LogicalDeliveryId, Message, MetaSocketPath, Metaflow, Model, ModelName, NativeInput,
    OrdinarySocketPath, Process, QueueEntry, Request, RequestId, ResultPayload, SentBy, Source,
    Start, Subaspect, Topic, Verbatim,
};

pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

/// The greeting every ordinary connection opens with, re-exported so a peer
/// greets through this contract alone, whatever Signal its own contracts
/// are built on.
pub use signal::{ContractDigest, Contracted, Handshake, HandshakeReceipt, HandshakeRejection};

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The transition vocabulary the Signal carries.
pub const TRANSITION_ETHOS: &str = include_str!("../ethos/transition.ethos");
/// The whole contract this socket speaks: the Signal, then the transitions
/// it carries.
pub const CONTRACT: &str = concat!(
    include_str!("../ethos/signal.ethos"),
    include_str!("../ethos/transition.ethos")
);
pub const WIRE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The contract is identified on the wire by the digest of its authored
/// Ethos sources; the querying side greets with it.
impl signal::Contracted for Input {
    const CONTRACT_SOURCE: &'static str = CONTRACT;
}

/// A subscription's state follows its changes: a transition replaces the
/// entry of its Address, and an Address Flows does not hold yet is taken in
/// at the end. Nothing is removed, so Flows holds one entry for each
/// permaflow, its last transition.
impl signal::Following for Flows {
    type Change = FlowTransition;

    fn after(flows: Self, change: FlowTransition) -> Self {
        let Flows(mut entries) = flows;
        match entries
            .iter()
            .position(|entry| entry.address == change.address)
        {
            Some(at) => entries[at] = change,
            None => entries.push(change),
        }
        Flows(entries)
    }
}
