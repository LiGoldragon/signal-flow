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
        "Start.{ { request-7 [ { Vision/flowNexus.md 54c08e7190360a308e560935c120c69b81c4aacb4975751a4841912b599f4f5a } ] [ spirit main-flow ] Field High Codex gpt-6-astra medium Some.836818 [ { 1b8ac0 1 } ] messaging-build «Carry this bounded launch request.» } { fac697 session-1 turn-2 } }",
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
    let reply = Response::RecipientDispositioned(signal_flow::RecipientDisposition::Deliverable(
        signal_flow::FlowNode {
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
            flow_lifecycle: signal_flow::FlowLifecycle::Ready,
        },
    ));
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

fn refresh_profile() -> signal_flow::LaunchProfile {
    signal_flow::LaunchProfile {
        launch_request_id: "refresh-request-1".into(),
        launch_source_vector: Vec::new(),
        skill_name_vector: vec!["spirit".into(), "main-flow".into()],
        flow_aspect: signal_flow::FlowAspect::Mind,
        power_level: signal_flow::PowerLevel::High,
        harness_kind: signal_flow::HarnessKind::Codex,
        model_name: "gpt-6-astra".into(),
        effort: "high".into(),
        flow_id_option: Some("47764b".into()),
        remembered_flow_vector: Vec::new(),
        herdr_session_name: "mind-high".into(),
        instruction_prompt: "Refresh from the referenced handoff.".into(),
    }
}

fn handover_reference() -> signal_flow::TranscriptHandoverReference {
    signal_flow::TranscriptHandoverReference {
        harness_kind: signal_flow::HarnessKind::Codex,
        native_session_id: "native-predecessor".into(),
        native_turn_id: "turn-handoff".into(),
        transcript_item_id: "item-handoff".into(),
        transcript_role: signal_flow::TranscriptRole::Assistant,
        transcript_title: "Handoff — Mind High 47764b".into(),
        transcript_timestamp_seconds: 1_790_000_000,
        transcript_record_sha256:
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
        handover_selection: signal_flow::HandoverSelection::SelectedBytes(
            signal_flow::HandoverByteSelection {
                handover_byte_offset: 128,
                handover_byte_length: 512,
                handover_selection_sha256:
                    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".into(),
            },
        ),
    }
}

#[test]
fn refresh_request_carries_stable_handover_identity_and_round_trips() {
    let reference = handover_reference();
    let request = signal_flow::RefreshRequest {
        flow_id: "47764b".into(),
        caller_flow_hint: "47764b".into(),
        transcript_handover_reference: reference.clone(),
        launch_profile: refresh_profile(),
        origin_clue: signal_flow::OriginClue {
            flow_id: "47764b".into(),
            session_id: "native-predecessor".into(),
            turn_id: "turn-refresh".into(),
        },
    };
    let query = Query::Refresh(request);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
        query
    );
    let text = query.datomize(vec![]).protosize().textualize();
    assert_eq!(
        Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap(),
        query
    );
    assert_eq!(
        signal_flow::ReplacementIdempotencyKey {
            flow_id: "47764b".into(),
            transcript_record_sha256: reference.transcript_record_sha256.clone(),
        }
        .transcript_record_sha256,
        reference.transcript_record_sha256
    );
}

#[test]
fn registered_unconfirmed_is_an_explicit_delivery_hold() {
    let reply = Response::RecipientDispositioned(signal_flow::RecipientDisposition::Held(
        signal_flow::RecipientHold {
            flow_id: "47764b".into(),
            flow_lifecycle: signal_flow::FlowLifecycle::RegisteredUnconfirmed,
            delivery_hold_reason: signal_flow::DeliveryHoldReason::NativeReceiptUnconfirmed,
        },
    ));
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        reply
    );
    assert!(matches!(
        reply,
        Response::RecipientDispositioned(signal_flow::RecipientDisposition::Held(_))
    ));
}

#[test]
fn retiring_and_archived_recipients_are_typed_reroutes() {
    for lifecycle in [
        signal_flow::FlowLifecycle::Retiring,
        signal_flow::FlowLifecycle::Archived,
    ] {
        let reply = Response::RecipientDispositioned(signal_flow::RecipientDisposition::Reroute(
            signal_flow::RecipientReroute {
                flow_id: "47764b".into(),
                replacement_flow_id: "replacement-1".into(),
                flow_lifecycle: lifecycle,
            },
        ));
        assert!(matches!(
            reply,
            Response::RecipientDispositioned(signal_flow::RecipientDisposition::Reroute(_))
        ));
    }
}

#[test]
fn decided_refresh_policy_is_explicit_in_progress() {
    let request = signal_flow::RefreshRequest {
        flow_id: "47764b".into(),
        caller_flow_hint: "47764b".into(),
        transcript_handover_reference: handover_reference(),
        launch_profile: refresh_profile(),
        origin_clue: signal_flow::OriginClue {
            flow_id: "47764b".into(),
            session_id: "native-predecessor".into(),
            turn_id: "turn-refresh".into(),
        },
    };
    let progress = signal_flow::RefreshAttempt {
        replacement_idempotency_key: signal_flow::ReplacementIdempotencyKey {
            flow_id: "47764b".into(),
            transcript_record_sha256: request
                .transcript_handover_reference
                .transcript_record_sha256
                .clone(),
        },
        refresh_request: request,
        refresh_policy: signal_flow::RefreshPolicy {
            maximum_handover_age_seconds: 86_400,
        },
        refresh_attempt_phase: signal_flow::RefreshAttemptPhase::ReplacementReady,
        flow_id_option: Some("replacement-1".into()),
        caller_proof_option: None,
        replacement_ready_proof_option: None,
        cutover_receipt_option: None,
    };
    assert_eq!(progress.refresh_policy.maximum_handover_age_seconds, 86_400);
    assert_eq!(
        progress.refresh_attempt_phase,
        signal_flow::RefreshAttemptPhase::ReplacementReady
    );
    assert!(progress.cutover_receipt_option.is_none());
}

#[test]
fn cutover_receipt_requires_ready_route_inactive_predecessor_and_archive() {
    let process = signal_flow::ProcessIdentity {
        process_id: 4242,
        process_user_id: 1001,
        process_start_token: "9328441".into(),
    };
    let pane = signal_flow::HerdrPaneBinding {
        launch_request_id: "refresh-request-1".into(),
        herdr_session_name: "mind-high".into(),
        herdr_agent_name: "mind-high-of-47764b".into(),
        herdr_workspace_id: "workspace-1".into(),
        herdr_pane_id: "w1:p4".into(),
        herdr_terminal_id: "terminal-1".into(),
    };
    let ready = signal_flow::ReplacementReadyProof {
        herdr_pane_binding: pane.clone(),
        process_identity: process.clone(),
        interactive_readiness: signal_flow::InteractiveReadiness::Ready,
        native_target_receipt: signal_flow::NativeTargetReceipt {
            launch_request_id: "refresh-request-1".into(),
            prompt_sha256: "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
                .into(),
            flow_id: "replacement-1".into(),
            native_session_id: "native-replacement".into(),
            native_turn_id: "first-turn".into(),
            receipt_sha256: "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
                .into(),
            model_name: "gpt-6-astra".into(),
            effort: "high".into(),
            native_skill_selection_vector: Vec::new(),
        },
    };
    let cutover = signal_flow::CutoverReceipt {
        predecessor_flow_id: "47764b".into(),
        replacement_flow_id: "replacement-1".into(),
        replacement_ready_proof: ready,
        old_route_removal_proof: signal_flow::OldRouteRemovalProof {
            flow_id: "47764b".into(),
            herdr_route: signal_flow::HerdrRoute {
                herdr_session_name: "mind-high".into(),
                herdr_agent_name: "mind-high-of-47764b".into(),
                herdr_pane_id: "w1:p3".into(),
                herdr_terminal_id: "terminal-old".into(),
            },
        },
        old_inactive_proof: signal_flow::OldInactiveProof {
            flow_id: "47764b".into(),
            process_identity: process,
            registration_absence: signal_flow::RegistrationAbsence::Unregistered,
            process_liveness: signal_flow::ProcessLiveness::Dead,
        },
        archive_receipt: signal_flow::ArchiveReceipt {
            archive_path: "/archive/native-predecessor.jsonl".into(),
            archive_sha256: "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .into(),
            archive_index_id: "archive-1".into(),
        },
    };
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&cutover).unwrap();
    assert_eq!(
        rkyv::from_bytes::<signal_flow::CutoverReceipt, rkyv::rancor::Error>(&archive).unwrap(),
        cutover
    );
}
