#![cfg(feature = "datom")]

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
        "Start.{ { request-7 [ { Vision/flowNexus.md 54c08e7190360a308e560935c120c69b81c4aacb4975751a4841912b599f4f5a } ] [ spirit main-flow ] Field High Codex gpt-6-astra medium Some.836818 [ { 1b8ac0 1 } ] messaging-build /workspace/bundles/flow.md «Carry this bounded launch request.» } { fac697 session-1 turn-2 } }",
        "Restart.{ fac697 { fac697 session-1 turn-2 } }",
        "ResolveRecipient.fac697",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
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
        herdr_route_selection: signal_flow::HerdrRouteSelection::Available(
            signal_flow::HerdrRoute {
                herdr_session_name: "messaging-build".into(),
                herdr_agent_name: "psyche-mind-astra".into(),
                herdr_pane_id: "w1:p3".into(),
                herdr_terminal_id: "term-fixture".into(),
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
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        reply
    );
    let text = reply.datomize(vec![]).protosize().textualize();
    let restored = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(restored, reply);
}

#[test]
fn launch_composition_types_round_trip_without_changing_ordinary_variants() {
    let profile = signal_flow::LaunchProfile {
        launch_request_id: "request-7".into(),
        launch_source_vector: vec![signal_flow::LaunchSource {
            source_path: "Vision/flowNexus.md".into(),
            source_sha256: "54c08e7190360a308e560935c120c69b81c4aacb4975751a4841912b599f4f5a"
                .into(),
        }],
        skill_name_vector: vec!["spirit".into(), "main-flow".into()],
        flow_aspect: signal_flow::FlowAspect::Field,
        power_level: signal_flow::PowerLevel::High,
        harness_kind: signal_flow::HarnessKind::Codex,
        model_name: "gpt-6-astra".into(),
        effort: "medium".into(),
        flow_id_option: Some("836818".into()),
        remembered_flow_vector: vec![signal_flow::RememberedFlow {
            flow_id: "1b8ac0".into(),
            remembering_depth: 1,
        }],
        herdr_session_name: "messaging-build".into(),
        system_prompt_bundle_file: "/workspace/bundles/flow.md".into(),
        instruction_prompt: "Carry this bounded launch request.".into(),
    };
    let receipt_request = signal_flow::TargetReceiptRequest {
        launch_request_id: profile.launch_request_id.clone(),
        prompt_sha256: "0286307646b3fb93a6e70d7012eaa2d07239bb1eb7c81a668fe1d4b6f3d97b3b".into(),
    };
    let composed = signal_flow::ComposedLaunch {
        launch_profile: profile,
        first_prompt_payload: signal_flow::FirstPromptPayload {
            first_prompt_body: "prompt body".into(),
            prompt_sha256: receipt_request.prompt_sha256.clone(),
            first_prompt_text: "prompt body\nreceipt request".into(),
        },
        target_receipt_request: receipt_request,
    };

    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&composed).unwrap();
    assert_eq!(
        rkyv::from_bytes::<signal_flow::ComposedLaunch, rkyv::rancor::Error>(&archive).unwrap(),
        composed
    );
}

#[test]
fn launch_attempt_journal_round_trips_with_one_shot_intent() {
    let binding = signal_flow::HerdrPaneBinding {
        launch_request_id: "request-7".into(),
        herdr_session_name: "messaging-build".into(),
        herdr_agent_name: "field-high-of-836818".into(),
        herdr_workspace_id: "workspace-1".into(),
        herdr_pane_id: "w1:p4".into(),
        herdr_terminal_id: "term-4".into(),
    };
    let intent = signal_flow::PromptDeliveryIntent {
        launch_request_id: "request-7".into(),
        prompt_sha256: "0286307646b3fb93a6e70d7012eaa2d07239bb1eb7c81a668fe1d4b6f3d97b3b".into(),
        flow_id: "908786".into(),
        native_session_id: "native-session-1".into(),
        harness_kind: signal_flow::HarnessKind::Codex,
        model_name: "gpt-6-astra".into(),
        effort: "medium".into(),
        native_skill_selection_vector: vec![signal_flow::NativeSkillSelection {
            skill_name: "spirit".into(),
            native_skill_path: "/configured/skills/spirit/SKILL.md".into(),
            native_skill_sha256: "b2149f4ce39c3ec8984ca8e671514900470eaca3f132c30c89c12980f3ac3b08"
                .into(),
        }],
        herdr_pane_binding: binding,
        native_transcript_boundary: signal_flow::NativeTranscriptBoundary::Existing(
            signal_flow::NativeTranscriptCursor {
                native_session_id: "native-session-1".into(),
                harness_kind: signal_flow::HarnessKind::Codex,
                transcript_device: "2049".into(),
                transcript_inode: "99142".into(),
                transcript_byte_offset: 4096,
                transcript_prefix_sha256:
                    "a2149f4ce39c3ec8984ca8e671514900470eaca3f132c30c89c12980f3ac3b07".into(),
            },
        ),
    };
    let attempt = signal_flow::LaunchAttempt {
        launch_request_id: intent.launch_request_id.clone(),
        launch_profile: signal_flow::LaunchProfile {
            launch_request_id: intent.launch_request_id.clone(),
            launch_source_vector: Vec::new(),
            skill_name_vector: vec!["spirit".into()],
            flow_aspect: signal_flow::FlowAspect::Field,
            power_level: signal_flow::PowerLevel::High,
            harness_kind: signal_flow::HarnessKind::Codex,
            model_name: intent.model_name.clone(),
            effort: intent.effort.clone(),
            flow_id_option: None,
            remembered_flow_vector: Vec::new(),
            herdr_session_name: "messaging-build".into(),
            system_prompt_bundle_file: "/workspace/bundles/flow.md".into(),
            instruction_prompt: "Carry this bounded launch request.".into(),
        },
        prompt_sha256: intent.prompt_sha256.clone(),
        origin_clue: signal_flow::OriginClue {
            flow_id: "fac697".into(),
            session_id: "caller-session".into(),
            turn_id: "caller-turn".into(),
        },
        launch_attempt_phase: signal_flow::LaunchAttemptPhase::PromptIntentRecorded,
        native_launch_intent_option: Some(signal_flow::NativeLaunchIntent {
            launch_request_id: intent.launch_request_id.clone(),
            prompt_sha256: intent.prompt_sha256.clone(),
            harness_kind: signal_flow::HarnessKind::Codex,
            model_name: intent.model_name.clone(),
            effort: intent.effort.clone(),
            skill_name_vector: vec!["spirit".into()],
        }),
        native_launch_binding_option: None,
        registration_acknowledgement_option: None,
        prompt_delivery_intent_option: Some(intent),
        prompt_delivery_result_option: None,
    };

    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&attempt).unwrap();
    assert_eq!(
        rkyv::from_bytes::<signal_flow::LaunchAttempt, rkyv::rancor::Error>(&archive).unwrap(),
        attempt
    );

    let absent =
        signal_flow::NativeTranscriptBoundary::Absent(signal_flow::NativeTranscriptAbsence {
            native_session_id: "native-session-2".into(),
            harness_kind: signal_flow::HarnessKind::Claude,
            transcript_root_device: "2049".into(),
            transcript_root_inode: "99143".into(),
        });
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&absent).unwrap();
    assert_eq!(
        rkyv::from_bytes::<signal_flow::NativeTranscriptBoundary, rkyv::rancor::Error>(&archive)
            .unwrap(),
        absent
    );
}

#[test]
fn send_grades_distinguish_untyped_accepted_presented_and_uncertain() {
    let not_delivered = Response::SendRejected(signal_flow::SendRejection::NotDelivered);
    let accepted = Response::Sent(signal_flow::SendOutcome::Accepted("908786".into()));
    let presented = Response::Sent(signal_flow::SendOutcome::Presented(
        signal_flow::PresentationReceipt {
            flow_id: "908786".into(),
            herdr_pane_id: "w1:p3".into(),
            presentation_observed_unix_milliseconds: 1_727_200_000_123,
        },
    ));
    let uncertain = Response::Sent(signal_flow::SendOutcome::Uncertain("908786".into()));

    for (response, expected) in [
        (not_delivered, "SendRejected.NotDelivered"),
        (accepted, "Sent.Accepted.908786"),
        (presented, "Sent.Presented.{ 908786 w1:p3 1727200000123 }"),
        (uncertain, "Sent.Uncertain.908786"),
    ] {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
        let text = response.datomize(vec![]).protosize().textualize();
        assert_eq!(text, expected);
        assert_eq!(
            Potential::<Response>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            response
        );
    }
}

#[test]
fn replace_and_launch_status_queries_have_concrete_datoms() {
    for text in [
        "Replace.{ { request-8 [] [ spirit main-flow ] Field High Claude opus-5-5 high Some.fac697 [] messaging-build /workspace/bundles/flow.md «Carry on from fac697.» } { fac697 session-1 turn-2 } }",
        "LaunchStatus.request-8",
        "Observe.Launch.request-8",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
        assert_eq!(query.datomize(vec![]).protosize().textualize(), text);
    }
}

#[test]
fn replace_carries_the_predecessor_in_its_launch_profile() {
    let text = "Replace.{ { request-8 [] [ spirit ] Field High Claude opus-5-5 high Some.fac697 [] messaging-build /workspace/bundles/flow.md «Carry on from fac697.» } { fac697 session-1 turn-2 } }";
    let Query::Replace(request) = Potential::<Query>::from(text)
        .actualize(&mut budget())
        .unwrap()
    else {
        panic!("Replace");
    };
    assert_eq!(
        request.launch_profile.flow_id_option,
        Some("fac697".to_string())
    );
}

#[test]
fn replace_and_launch_status_responses_have_concrete_datoms() {
    for text in [
        "Replaced.{ fac697 { 908786 session-2 { fac697 session-1 turn-2 } } }",
        "ReplaceRejected.PredecessorAbsent",
        "ReplaceRejected.UnknownPredecessor",
        "ReplaceRejected.PredecessorStopped",
        "ReplaceRejected.LaunchRefused.NativeLaunchRefused",
        "ReplaceRejected.ReapRefused.CloseRefused",
        "LaunchStatusRejected.UnknownLaunchRequest",
        "LaunchStatusRejected.PersistenceRefused",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
        assert_eq!(response.datomize(vec![]).protosize().textualize(), text);
    }
}

#[test]
fn replaced_names_the_stopped_predecessor_and_the_started_successor() {
    let reply = Response::Replaced(signal_flow::Replaced {
        flow_id: "fac697".into(),
        started: signal_flow::Started {
            flow_id: "908786".into(),
            session_id: "session-2".into(),
            origin_clue: signal_flow::OriginClue {
                flow_id: "fac697".into(),
                session_id: "session-1".into(),
                turn_id: "turn-2".into(),
            },
        },
    });
    assert_eq!(
        reply.datomize(vec![]).protosize().textualize(),
        "Replaced.{ fac697 { 908786 session-2 { fac697 session-1 turn-2 } } }"
    );
}

#[test]
fn resolve_caller_queries_and_replies_have_concrete_datoms() {
    for text in ["ResolveCaller.None", "ResolveCaller.Some.fac697"] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
        assert_eq!(query.datomize(vec![]).protosize().textualize(), text);
    }
    for text in [
        "CallerResolved.{ fac697 Psyche High claude-opus-5-5 }",
        "CallerResolutionRejected.CallerUnknown",
        "CallerResolutionRejected.CallerMismatch.{ fac697 Mind Medium gpt-6-sol }",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
        assert_eq!(response.datomize(vec![]).protosize().textualize(), text);
    }
}

#[test]
fn caller_names_the_flow_its_aspect_power_and_model() {
    let reply = Response::CallerResolved(signal_flow::Caller {
        flow_id: "38de5b".into(),
        flow_aspect: signal_flow::FlowAspect::Psyche,
        power_level: signal_flow::PowerLevel::High,
        model_name: "claude-opus-5-5".into(),
    });
    assert_eq!(
        reply.datomize(vec![]).protosize().textualize(),
        "CallerResolved.{ 38de5b Psyche High claude-opus-5-5 }"
    );
}

/// ResolveCaller and its replies are appended: every variant the 5.0 wire
/// carried keeps its archived form, so a 5.0 peer still reads them. The
/// expected archives were read from signal-flow 5.0.0 (cf3648f).
#[test]
fn resolve_caller_leaves_earlier_variants_archived_as_before() {
    let archived_5_0 = |tag: u8, length: usize| {
        let mut bytes = vec![tag];
        bytes.extend_from_slice(b"fac697");
        bytes.extend_from_slice(&[255, 255]);
        bytes.resize(length, 0);
        bytes
    };
    assert_eq!(
        rkyv::to_bytes::<rkyv::rancor::Error>(&Query::ResolveRecipient("fac697".into()))
            .unwrap()
            .to_vec(),
        archived_5_0(2, 109)
    );
    assert_eq!(
        rkyv::to_bytes::<rkyv::rancor::Error>(&Response::Stopped("fac697".into()))
            .unwrap()
            .to_vec(),
        archived_5_0(6, 612)
    );
}
