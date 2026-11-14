//! A rejection 51.0.0 archived reads the same under 52.0.0: the
//! CompositionRefusal variant 52.0.0 appends leaves every earlier variant
//! where it was, and the StartRejection holding one no larger.

use signal_flow::{CompositionRefusal, ContextRefusal, ReplaceRejection, StartRejection};

/// `rkyv::to_bytes` of [`earlier`] under signal-flow 51.0.0 (9f159d9).
const ARCHIVED_BY_51: &str = concat!(
    "0300000000000000000000000000000000000000000000000000000000000003",
    "0001000000000000000000000000000000000000000000000000000000000300",
    "0200000000000000000000000000000000000000000000000000000000030003",
    "0000000000000000000000000000000000000000000000000000000003000400",
    "0000000000000000000000000000000000000000000000000000000300050000",
    "0000000000000000000000000000000000000000000000000000030006000000",
    "0000000000000000000000000000000000000000000000000003000700000000",
    "0000000000000000000000000000000000000000000000000300080000000000",
    "0000000000000000000000000000000000000000000000030009000000000000",
    "0000000000000000000000000000000000000000000003000a00000000000000",
    "00000000000000000000000000000000000000000003000b0000000000000000",
    "0000000000000000000000000000000000000000030a016d697373696e67ff00",
    "00000000000000000000000000000000000000030f746f706963ffffff000000",
    "0000000000000000000000000000000000004efeffff0e000000",
);

/// Every CompositionRefusal 51.0.0 knew, and two string-carrying
/// rejections after it, each as a Replace's refused launch.
fn earlier() -> Vec<ReplaceRejection> {
    use CompositionRefusal::*;
    let mut values: Vec<ReplaceRejection> = [
        InvalidLaunchRequestId,
        InvalidModelName,
        InvalidEffort,
        InvalidToolSet,
        InvalidHerdrSessionName,
        InvalidSystemModules,
        InvalidPromptModules,
        InvalidRemembering,
        InexactModel,
        Unconfigured,
        PresentationUnavailable,
        Unrecorded,
    ]
    .into_iter()
    .map(|refusal| ReplaceRejection::LaunchRefused(StartRejection::CompositionRefused(refusal)))
    .collect();
    values.push(ReplaceRejection::LaunchRefused(
        StartRejection::ContextRefused(ContextRefusal::ComposerRejected("missing".into())),
    ));
    values.push(ReplaceRejection::LaunchRefused(
        StartRejection::TopicRefused(signal_flow::Topic("topic".to_string())),
    ));
    values
}

fn bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).unwrap())
        .collect()
}

#[test]
fn a_rejection_51_archived_reads_the_same() {
    let archived = bytes(ARCHIVED_BY_51);
    let read = rkyv::from_bytes::<Vec<ReplaceRejection>, rkyv::rancor::Error>(&archived).unwrap();
    assert_eq!(read, earlier());
    let written = rkyv::to_bytes::<rkyv::rancor::Error>(&earlier()).unwrap();
    assert_eq!(written.as_slice(), archived.as_slice());
}

#[test]
fn a_handover_refusal_archives_and_reads_back() {
    let refusal = ReplaceRejection::LaunchRefused(StartRejection::CompositionRefused(
        CompositionRefusal::HandoverUnreadable(signal_flow::HandoverUnreadable_Data {
            flow_id: signal_flow::FlowId("abc123".to_string()),
            handover_fault: signal_flow::HandoverFault::Unreadable(
                "flows/abc123/handover.md: Permission denied (os error 13)".into(),
            ),
        }),
    ));
    let written = rkyv::to_bytes::<rkyv::rancor::Error>(&refusal).unwrap();
    let read = rkyv::from_bytes::<ReplaceRejection, rkyv::rancor::Error>(&written).unwrap();
    assert_eq!(read, refusal);
}
