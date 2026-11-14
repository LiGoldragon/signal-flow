use signal_flow::{Contracted, ETHOS, Handshake, HandshakeReceipt, HandshakeRejection, Query};

#[test]
fn a_peer_built_from_this_source_is_greeted() {
    let digest = Query::contract_digest();
    assert_eq!(digest, Handshake::of_source(ETHOS).contract_digest);
    assert_eq!(
        Query::receipt(&Handshake::of_source(ETHOS)),
        HandshakeReceipt::Greeted(digest)
    );
}

#[test]
fn a_peer_built_from_another_source_is_refused_with_this_digest() {
    let older = format!("{ETHOS}\n");
    assert_eq!(
        Query::receipt(&Handshake::of_source(&older)),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Query::contract_digest()
        ))
    );
}

/// A peer built on signal-flow 61.0.0 greets with 61's digest and is
/// refused with 62's, before any of its queries is read.
#[test]
fn a_signal_flow_61_peer_is_refused_with_this_digest() {
    let older = include_str!("fixtures/signal-flow-61.ethos");
    assert_ne!(Handshake::of_source(older), Query::greeting());
    assert_eq!(
        Query::receipt(&Handshake::of_source(older)),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            Query::contract_digest()
        ))
    );
}

/// The greeting a peer writes is the rkyv archive of its Handshake, and it
/// reads the receipt back the same way.
#[test]
fn the_greeting_and_its_receipt_round_trip_as_frames() {
    let greeting = Query::greeting();
    let frame = rkyv::to_bytes::<rkyv::rancor::Error>(&greeting).unwrap();
    let read = rkyv::from_bytes::<Handshake, rkyv::rancor::Error>(&frame).unwrap();
    assert_eq!(read, greeting);
    let receipt = Query::receipt(&read);
    let frame = rkyv::to_bytes::<rkyv::rancor::Error>(&receipt).unwrap();
    assert_eq!(
        rkyv::from_bytes::<HandshakeReceipt, rkyv::rancor::Error>(&frame).unwrap(),
        HandshakeReceipt::Greeted(Query::contract_digest())
    );
}
