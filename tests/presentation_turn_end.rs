#![cfg(feature = "datom")]

use datom_codec::{
    Actualizing, Budget, Composable, DatomForming, Datomizable, Potential, Variantizing,
};
use protos::{Protosizable, ReaderBudget, Textualizable};
use signal_flow::{Presentation, Query, Response, TurnEndRejection, TurnEndRequest};

fn budget() -> Budget {
    Budget {
        remaining: 4096,
        reader: ReaderBudget { remaining: 4096 },
        depth: 0,
        maximum_depth: 1024,
    }
}

#[test]
fn presentation_is_one_title_and_escapes_delimiters() {
    let value = Presentation {
        title: "A title with » and trailing \\".into(),
    };
    let text = value
        .datomize(vec![1])
        .named_variant(vec![], "Presentation")
        .protosize()
        .textualize();
    assert_eq!(text, r"Presentation.{ «A title with \» and trailing \\» }");
    let datom = text.protosize().unwrap().datom_form(vec![]).unwrap();
    let mut limits = budget();
    let (head, body) = datom.variant(&mut limits, "Presentation").unwrap();
    assert_eq!(head, "Presentation");
    assert_eq!(body.compose::<Presentation>(&mut limits).unwrap(), value);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Presentation, rkyv::rancor::Error>(&archive).unwrap(),
        value
    );
    assert!(
        Potential::<Presentation>::from("{ title extra }")
            .actualize(&mut budget())
            .is_err()
    );
}

#[test]
fn queue_turn_end_has_native_session_turn_and_optional_transcript_reference() {
    for (path, expected) in [
        (
            Some("/tmp/a transcript»\\.jsonl".into()),
            r"QueueTurnEnd.{ session-1 turn-2 Some.«/tmp/a transcript\»\.jsonl» }",
        ),
        (None, "QueueTurnEnd.{ session-1 turn-2 None }"),
    ] {
        let request = TurnEndRequest {
            session_id: "session-1".into(),
            turn_id: "turn-2".into(),
            transcript_path_option: path,
        };
        let query = Query::QueueTurnEnd(request);
        let text = query.datomize(vec![]).protosize().textualize();
        assert_eq!(text, expected);
        assert_eq!(
            Potential::<Query>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            query
        );
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
    }
    assert!(
        Potential::<Query>::from("QueueTurnEnd.{ session-1 None }")
            .actualize(&mut budget())
            .is_err()
    );
    assert!(
        Potential::<Query>::from("QueueTurnEnd.{ session-1 turn-2 None extra }")
            .actualize(&mut budget())
            .is_err()
    );
}

#[test]
fn enqueue_acknowledgement_and_refusals_have_exact_signal_and_datom_forms() {
    let queued = Response::TurnEndQueued(TurnEndRequest {
        session_id: "session-1".into(),
        turn_id: "turn-2".into(),
        transcript_path_option: None,
    });
    for (value, expected) in [
        (queued, "TurnEndQueued.{ session-1 turn-2 None }"),
        (
            Response::TurnEndRejected(TurnEndRejection::UnknownSession),
            "TurnEndRejected.UnknownSession",
        ),
        (
            Response::TurnEndRejected(TurnEndRejection::SessionMismatch),
            "TurnEndRejected.SessionMismatch",
        ),
        (
            Response::TurnEndRejected(TurnEndRejection::InvalidTurnId),
            "TurnEndRejected.InvalidTurnId",
        ),
        (
            Response::TurnEndRejected(TurnEndRejection::InvalidTranscriptPath),
            "TurnEndRejected.InvalidTranscriptPath",
        ),
        (
            Response::TurnEndRejected(TurnEndRejection::QueueRefused),
            "TurnEndRejected.QueueRefused",
        ),
    ] {
        let text = value.datomize(vec![]).protosize().textualize();
        assert_eq!(text, expected);
        assert_eq!(
            Potential::<Response>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            value
        );
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            value
        );
    }
}
