use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use protos::{Protosizable, ReaderBudget, Textualizable};
use signal_flow::{Query, Response};

fn budget() -> Budget {
    Budget {
        remaining: 4096,
        reader: ReaderBudget { remaining: 4096 },
        depth: 0,
        maximum_depth: 1024,
    }
}

#[test]
fn every_ordinary_request_has_a_concrete_datom() {
    for text in [
        "Start.{ codex-medium { fac697 session-1 turn-2 } }",
        "Restart.{ fac697 fac697 }",
        "ResolveRecipient.fac697",
    ] {
        let query = Potential::<Query>::from(text).actualize(&mut budget()).unwrap();
        assert_eq!(query.datomize(vec![]).protosize().textualize(), text);
    }
}

#[test]
fn flow_node_round_trips_over_signal_and_datom() {
    let reply = Response::RecipientResolved(signal_flow::FlowNode {
        flow_id: "fac697".into(),
        session_id: "session-1".into(),
        harness_kind: signal_flow::HarnessKind::Codex,
        endpoint_selection: signal_flow::EndpointSelection::Available(
            signal_flow::Available_Data {
                endpoint_path: "/run/user/1001/codex.sock".into(),
                route_readiness: signal_flow::RouteReadiness::Ready,
            },
        ),
        origin_clue: signal_flow::OriginClue {
            flow_id: "parent".into(),
            session_id: "session-0".into(),
            turn_id: "turn-1".into(),
        },
        flow_lifecycle: signal_flow::FlowLifecycle::Active,
    });
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
    assert_eq!(rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(), reply);
    let text = reply.datomize(vec![]).protosize().textualize();
    let restored = Potential::<Response>::from(text).actualize(&mut budget()).unwrap();
    assert_eq!(restored, reply);
}
