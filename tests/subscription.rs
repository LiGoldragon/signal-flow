//! Subscribe: Flows, one entry for each permaflow, its last transition, and
//! each FlowTransition after it. Flows follows a transition by replacing the
//! entry of its Address, or by taking it in at the end; nothing is removed.

use signal::{Following, Frame};
use signal_flow::transition::{Cause, Event, Moment, State, Transition};
use signal_flow::{
    Address, Aspect, FlowId, FlowTransition, Flows, Input, Model, Output, Tokens, Topic,
};

fn address(topic: &str) -> Address {
    Address {
        aspect: Aspect::Mind,
        topic: Topic(topic.to_string()),
        model: Model::Opus,
    }
}

fn transition(flow: &str, topic: &str, state: State, cause: Cause, moment: i64) -> FlowTransition {
    FlowTransition {
        flow_id: FlowId(flow.to_string()),
        address: address(topic),
        transition: Transition {
            state,
            cause,
            moment: Moment(moment),
        },
    }
}

#[test]
fn a_transition_replaces_the_entry_of_its_address_and_takes_in_a_new_one() {
    let core = transition(
        "a1b2c3",
        "core",
        State::Acting,
        Cause::Reported(Event::Started),
        10,
    );
    let book = transition(
        "d4e5f6",
        "book",
        State::Resting,
        Cause::Reported(Event::Idle(Tokens(9))),
        11,
    );
    let flows = Flows(vec![core.clone(), book.clone()]);
    // The same permaflow's next flow replaces its entry where it stood.
    let successor = transition(
        "0a0b0c",
        "core",
        State::Asleep,
        Cause::Reported(Event::Ended),
        12,
    );
    let flows = Flows::after(flows, successor.clone());
    assert_eq!(flows, Flows(vec![successor.clone(), book.clone()]));
    // A permaflow Flows does not hold is taken in at the end.
    let fresh = transition("777777", "fresh", State::Acting, Cause::Overridden, 13);
    let flows = Flows::after(flows, fresh.clone());
    assert_eq!(flows, Flows(vec![successor, book, fresh]));
}

#[test]
fn every_frame_of_the_stream_round_trips_as_one_subscribed_output() {
    let core = transition(
        "a1b2c3",
        "core",
        State::Acting,
        Cause::Reported(Event::Started),
        10,
    );
    for frame in [
        Frame::Opened(Flows(vec![core.clone()])),
        Frame::Opened(Flows(Vec::new())),
        Frame::Changed(core.clone()),
        Frame::Lagged(Flows(vec![core.clone()])),
    ] {
        let output = Output::Subscribed(frame);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&output).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&archive).unwrap(),
            output
        );
    }
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&Input::Subscribe).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Input, rkyv::rancor::Error>(&archive).unwrap(),
        Input::Subscribe
    );
}

#[cfg(feature = "datom")]
#[test]
fn subscribe_and_its_frames_read_as_datom() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};
    let budget = || Budget {
        remaining: 4096,
        reader: ReaderBudget::try_from(4096).unwrap(),
        depth: 0,
        maximum_depth: 1024,
    };
    let input = Potential::<Input>::from("Subscribe")
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(input, Input::Subscribe);
    assert_eq!(input.datomize(vec![]).protosize().compact(), "Subscribe");
    for text in [
        "Subscribed.Opened.[ { a1b2c3 { Mind core Opus } { Acting Reported.Started 10 } } ]",
        "Subscribed.Changed.{ a1b2c3 { Mind core Opus } { Asleep Reported.Ended 12 } }",
        "Subscribed.Changed.{ a1b2c3 { Mind core Opus } { Acting Arrived.k7ph 13 } }",
    ] {
        let output = Potential::<Output>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(output.datomize(vec![]).protosize().compact(), text);
    }
}
