//! Report carries one harness event of a flow Flow holds; Reported answers
//! it, and a FlowId Flow does not hold is refused as Refused.Unknown.FlowId.

use signal_flow::{
    Address, Aspect, BackgroundTask, BackgroundTaskKind, Event, Model, NativePrompt, NativeStop,
    Query, Report_Data, Response, Topic, VisibleRefusal, VisibleUnknown,
};

fn reports() -> Vec<Query> {
    [
        Event::Started(Some(signal_flow::NativeSessionId("s2".to_string()))),
        Event::Started(None),
        Event::ToolUsed("Bash".into()),
        Event::Stopped(NativeStop {
            native_session_id_option: Some(signal_flow::NativeSessionId("s1".to_string())),
            transcript_path_option: Some(signal_flow::TranscriptPath("/p/s1.jsonl".to_string())),
            last_assistant_message_option: signal_flow::LastAssistantMessageOption(None),
            background_work: signal_flow::BackgroundWork(vec![]),
        }),
        Event::Stopped(NativeStop {
            native_session_id_option: None,
            transcript_path_option: None,
            last_assistant_message_option: signal_flow::LastAssistantMessageOption(None),
            background_work: signal_flow::BackgroundWork(vec![]),
        }),
        // A turn that ended with a subagent and a monitor still in flight.
        Event::Stopped(NativeStop {
            native_session_id_option: Some(signal_flow::NativeSessionId("s1".to_string())),
            transcript_path_option: Some(signal_flow::TranscriptPath("/p/s1.jsonl".to_string())),
            last_assistant_message_option: signal_flow::LastAssistantMessageOption(Some(
                "waiting".to_string(),
            )),
            background_work: signal_flow::BackgroundWork(vec![
                BackgroundTask {
                    background_task_id: signal_flow::BackgroundTaskId("a1".to_string()),
                    background_task_kind: BackgroundTaskKind::Subagent,
                    background_task_status: signal_flow::BackgroundTaskStatus(
                        "running".to_string(),
                    ),
                    description: signal_flow::Description("review".to_string()),
                },
                BackgroundTask {
                    background_task_id: signal_flow::BackgroundTaskId("m1".to_string()),
                    background_task_kind: BackgroundTaskKind::Monitor,
                    background_task_status: signal_flow::BackgroundTaskStatus(
                        "running".to_string(),
                    ),
                    description: signal_flow::Description("watch".to_string()),
                },
                BackgroundTask {
                    background_task_id: signal_flow::BackgroundTaskId("c1".to_string()),
                    background_task_kind: BackgroundTaskKind::Other("cron".into()),
                    background_task_status: signal_flow::BackgroundTaskStatus(
                        "pending".to_string(),
                    ),
                    description: signal_flow::Description("later".to_string()),
                },
            ]),
        }),
        Event::PromptSubmitted(NativePrompt {
            native_session_id_option: Some(signal_flow::NativeSessionId("s1".to_string())),
            prompt_id_option: Some(signal_flow::PromptId("p1".to_string())),
            prompt_sha256: signal_flow::PromptSha256("ab12".to_string()),
        }),
        Event::Ended(Some(signal_flow::NativeSessionId("s1".to_string()))),
        Event::Ended(None),
        Event::Cleared(Some(signal_flow::NativeSessionId("s1".to_string()))),
        Event::Cleared(None),
    ]
    .into_iter()
    .map(|event| {
        Query::Report(Report_Data {
            flow_id: signal_flow::FlowId("f1c841".to_string()),
            process_option: None,
            event,
        })
    })
    .collect()
}

fn replies() -> Vec<Response> {
    vec![
        Response::Reported,
        Response::SendRefused(VisibleRefusal::Unknown(VisibleUnknown::Address(Address {
            aspect: Aspect::Mind,
            topic: Topic("core".to_string()),
            model: Model::Sol,
        }))),
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
            reader: ReaderBudget::try_from(4096).unwrap(),
            depth: 0,
            maximum_depth: 1024,
        }
    }

    #[test]
    fn report_has_exact_inline_datoms() {
        for (query, expected) in super::reports().into_iter().zip([
            "Report.{ f1c841 None Started.Some.s2 }",
            "Report.{ f1c841 None Started.None }",
            "Report.{ f1c841 None ToolUsed.Bash }",
            "Report.{ f1c841 None Stopped.{ Some.s1 Some./p/s1.jsonl None [] } }",
            "Report.{ f1c841 None Stopped.{ None None None [] } }",
            "Report.{ f1c841 None Stopped.{ Some.s1 Some./p/s1.jsonl Some.waiting [ { a1 Subagent running review } { m1 Monitor running watch } { c1 Other.cron pending later } ] } }",
            "Report.{ f1c841 None PromptSubmitted.{ Some.s1 Some.p1 ab12 } }",
            "Report.{ f1c841 None Ended.Some.s1 }",
            "Report.{ f1c841 None Ended.None }",
            "Report.{ f1c841 None Cleared.Some.s1 }",
            "Report.{ f1c841 None Cleared.None }",
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
            "Report.{ f1c841 None }",
            "Report.{ f1c841 None Started }",
            "Report.{ f1c841 None Started.Some.s2 extra }",
            "Report.{ f1c841 Paused }",
            "Report.{ f1c841 None Stopped }",
            "Report.{ f1c841 None Stopped.{ Some.s1 } }",
            // A Stop without its background work, as signal-flow 56 wrote it.
            "Report.{ f1c841 None Stopped.{ None None None } }",
            "Report.{ f1c841 None PromptSubmitted.{ Some.s1 None } }",
            // A Report without its harness's birth, as signal-flow 59 wrote it.
            "Report.{ f1c841 Ended.Some.s1 }",
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
        for (reply, expected) in super::replies()
            .into_iter()
            .zip(["Reported", "SendRefused.Unknown.Address.{ Mind core Sol }"])
        {
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
