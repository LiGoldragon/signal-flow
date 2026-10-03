//! Report carries one harness event of a flow Flow holds; Reported answers
//! it, and a FlowId Flow does not hold is refused as Refused.UnknownFlow.

use signal_flow::{Event, Query, Refused_Data, Report_Data, Response};

fn reports() -> Vec<Query> {
    [
        Event::Started,
        Event::ToolUsed("Bash".into()),
        Event::Stopped,
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
        Response::Refused(Refused_Data::UnknownFlow("0a0a0a".into())),
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
            "Report.{ f1c841 Stopped }",
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
            .zip(["Reported", "Refused.UnknownFlow.0a0a0a"])
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
