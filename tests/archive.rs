//! The CompositionRefusal variants 52.0.0 and 64.0.0 append leave every
//! earlier variant where it was: its tag and its payload. 52.0.0 left the
//! StartRejection holding one no larger. 64.0.0 widens it:
//! RecoveryPredecessorMismatch carries two Addresses and three FlowIds, so a
//! CompositionRefusal archives in 45 bytes where it took 18, a StartRejection
//! in 46 where it took 30 and a ReplaceRejection in 47 where it took 31. A
//! rejection archived by 51.0.0 to 63.0.0, widened by the 16 bytes 64.0.0
//! adds, reads as the same rejection. A store that archives a bare rejection
//! migrates it; one that archives it inside a record at least as wide reads
//! it the same.

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

/// The archived width of one ReplaceRejection from 51.0.0 to 63.0.0.
const EARLIER_WIDTH: usize = 31;
/// The archived width of one ReplaceRejection under 64.0.0.
const WIDTH: usize = 47;

/// The archive of `earlier` as 64.0.0 writes it: each rejection of the 51
/// archive, followed by the zeroed bytes 64.0.0 adds, then the vector's
/// root, a relative offset back to its first element and its length. Both
/// strings of `earlier` are short enough to be archived inline.
fn widened(archived: &[u8]) -> Vec<u8> {
    let count = earlier().len();
    let mut widened: Vec<u8> = archived[..count * EARLIER_WIDTH]
        .chunks(EARLIER_WIDTH)
        .flat_map(|rejection| {
            let mut wide = rejection.to_vec();
            wide.resize(WIDTH, 0);
            wide
        })
        .collect();
    let offset = -i32::try_from(count * WIDTH).unwrap();
    widened.extend_from_slice(&offset.to_le_bytes());
    widened.extend_from_slice(&u32::try_from(count).unwrap().to_le_bytes());
    widened
}

#[test]
fn a_rejection_51_archived_reads_the_same_widened() {
    let archived = widened(&bytes(ARCHIVED_BY_51));
    let read = rkyv::from_bytes::<Vec<ReplaceRejection>, rkyv::rancor::Error>(&archived).unwrap();
    assert_eq!(read, earlier());
    let written = rkyv::to_bytes::<rkyv::rancor::Error>(&earlier()).unwrap();
    assert_eq!(written.as_slice(), archived.as_slice());
}

#[test]
fn a_recovery_refusal_archives_and_reads_back() {
    let address = signal_flow::Address {
        aspect: signal_flow::Aspect::Mind,
        topic: signal_flow::Topic("flowNexus".to_string()),
        model: signal_flow::Model::Sol,
    };
    for refusal in [
        CompositionRefusal::RecoveryUnavailable(signal_flow::RecoveryUnavailable_Data {
            address: address.clone(),
            flow_id: signal_flow::FlowId("0e068b".to_string()),
            string: "the recovery text could not be read".to_string(),
        }),
        CompositionRefusal::RecoveryPredecessorMismatch(
            signal_flow::RecoveryPredecessorMismatch_Data {
                first_address: address.clone(),
                second_address: address.clone(),
                first_flow_id: signal_flow::FlowId("0e068b".to_string()),
                second_flow_id: signal_flow::FlowId("afc2ca".to_string()),
                third_flow_id: signal_flow::FlowId("28c285".to_string()),
            },
        ),
    ] {
        let rejection =
            ReplaceRejection::LaunchRefused(StartRejection::CompositionRefused(refusal));
        let written = rkyv::to_bytes::<rkyv::rancor::Error>(&rejection).unwrap();
        let read = rkyv::from_bytes::<ReplaceRejection, rkyv::rancor::Error>(&written).unwrap();
        assert_eq!(read, rejection);
    }
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
