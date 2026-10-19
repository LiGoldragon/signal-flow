//! Report carries one harness event of a flow Flow holds; Reported answers
//! it, and a FlowId Flow does not hold is refused as Refused.Unknown.FlowId.

use signal_flow::{
    Event, NativePrompt, NativeStop, PromptOrigin, Publication, Query, Refusal, Report_Data,
    Response, Unknown_Data,
};

fn reports() -> Vec<Query> {
    [
        Event::Started,
        Event::ToolUsed("Bash".into()),
        Event::Stopped(NativeStop {
            native_session_id_option: Some("s1".into()),
            transcript_path_option: Some("/p/s1.jsonl".into()),
        }),
        Event::Stopped(NativeStop {
            native_session_id_option: None,
            transcript_path_option: None,
        }),
        Event::PromptSubmitted(NativePrompt {
            native_session_id_option: Some("s1".into()),
            prompt_id_option: Some("p1".into()),
            prompt_sha256: "ab12".into(),
            prompt_origin: PromptOrigin::Psyche,
        }),
        Event::PromptSubmitted(NativePrompt {
            native_session_id_option: Some("s1".into()),
            prompt_id_option: None,
            prompt_sha256: "cd34".into(),
            prompt_origin: PromptOrigin::Queued,
        }),
        Event::StopHeld(NativeStop {
            native_session_id_option: Some("s1".into()),
            transcript_path_option: Some("/p/s1.jsonl".into()),
        }),
        Event::Published(Publication {
            native_session_id_option: Some("s1".into()),
            prompt_id_option: Some("p1".into()),
            artifact_url: "https://claude.ai/code/artifact/4fcc3326".into(),
        }),
    ]
    .into_iter()
    .map(|event| {
        Query::Report(Report_Data {
            flow_id: "f1c841".into(),
            event,
        })
    })
    .collect()
}

fn replies() -> Vec<Response> {
    vec![
        Response::Reported,
        Response::Refused(Refusal::Unknown(Unknown_Data::FlowId("0a0a0a".into()))),
        Response::TurnHeld("Make the book for the living's words before ending".into()),
    ]
}

#[test]
fn every_report_and_its_replies_round_trip_through_rkyv() {
    for query in reports() {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
    }
    for reply in replies() {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            reply
        );
    }
}

#[cfg(feature = "datom")]
mod datom {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};
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
    fn report_has_exact_inline_datoms() {
        for (query, expected) in super::reports().into_iter().zip([
            "Report.{ f1c841 Started }",
            "Report.{ f1c841 ToolUsed.Bash }",
            "Report.{ f1c841 Stopped.{ Some.s1 Some./p/s1.jsonl } }",
            "Report.{ f1c841 Stopped.{ None None } }",
            "Report.{ f1c841 PromptSubmitted.{ Some.s1 Some.p1 ab12 Psyche } }",
            "Report.{ f1c841 PromptSubmitted.{ Some.s1 None cd34 Queued } }",
            "Report.{ f1c841 StopHeld.{ Some.s1 Some./p/s1.jsonl } }",
            "Report.{ f1c841 Published.{ Some.s1 Some.p1 https://claude.ai/code/artifact/4fcc3326 } }",
        ]) {
            let text = query.datomize(vec![]).protosize().compact();
            assert_eq!(text, expected);
            assert_eq!(
                Potential::<Query>::from(text)
                    .actualize(&mut budget())
                    .unwrap(),
                query
            );
        }
        for refused in [
            "Report.{ f1c841 }",
            "Report.{ f1c841 Started extra }",
            "Report.{ f1c841 Paused }",
            "Report.{ f1c841 Stopped }",
            "Report.{ f1c841 Stopped.{ Some.s1 } }",
            "Report.{ f1c841 PromptSubmitted.{ Some.s1 None } }",
            "Report.{ f1c841 PromptSubmitted.{ Some.s1 Some.p1 ab12 } }",
            "Report.{ f1c841 PromptSubmitted.{ Some.s1 Some.p1 ab12 Living } }",
            "Report.{ f1c841 Published.{ Some.s1 Some.p1 } }",
        ] {
            assert!(
                Potential::<Query>::from(refused)
                    .actualize(&mut budget())
                    .is_err()
            );
        }
    }

    #[test]
    fn reported_and_unknown_flow_have_exact_inline_datoms() {
        for (reply, expected) in super::replies().into_iter().zip([
            "Reported",
            "Refused.Unknown.FlowId.0a0a0a",
            "TurnHeld.«Make the book for the living's words before ending»",
        ]) {
            let text = reply.datomize(vec![]).protosize().compact();
            assert_eq!(text, expected);
            assert_eq!(
                Potential::<Response>::from(text)
                    .actualize(&mut budget())
                    .unwrap(),
                reply
            );
        }
    }
}
