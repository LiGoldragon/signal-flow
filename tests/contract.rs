#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use protos::{Compactable, Protosizable, ReaderBudget};
use signal_flow::{Aspect, Query, Refusal, Response};

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
        "Start.{ request-7 [ spirit:core ] [ compensation:main-flow knowledge:flow ] Custom.[ 4e1d2a ] { Field flow Primary } gpt-6-astra medium Some.836818 messaging-build Some.[ SkillTool WorkflowTool ] Regular Default «Carry this bounded launch request.» }",
        "Start.{ field-flow-quaternary-1 [ spirit:core ] [ compensation:main-flow ] Predecessors.0 { Field flow Quaternary } haiku medium None fieldFlow None Regular Default «Send the Order through Message.» }",
        "Refresh.{ Psyche core Primary }",
        "ResolveRecipient.fac697",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
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
        origin: signal_flow::Origin::Flow(signal_flow::FlowOrigin {
            flow_id: "parent".into(),
            session_id: "session-0".into(),
        }),
        flow_lifecycle: signal_flow::FlowLifecycle::Active,
    });
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        reply
    );
    let text = reply.datomize(vec![]).protosize().compact();
    let restored = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(restored, reply);
}

#[test]
fn launch_composition_types_round_trip_without_changing_ordinary_variants() {
    let profile = signal_flow::LaunchProfile {
        launch_request_id: "request-7".into(),
        system_modules: vec!["spirit:core".into()],
        user_modules: vec!["compensation:main-flow".into(), "knowledge:flow".into()],
        remembering: signal_flow::Remembering::Custom(vec!["4e1d2a".into()]),
        address: signal_flow::Address {
            aspect: signal_flow::Aspect::Field,
            topic: "flow".into(),
            layer: signal_flow::Layer::Primary,
        },
        model_name: "gpt-6-astra".into(),
        effort: "medium".into(),
        flow_id_option: Some("836818".into()),
        herdr_session_name: "messaging-build".into(),
        leave_outs_option: Some(vec![
            signal_flow::LeaveOut::SkillTool,
            signal_flow::LeaveOut::WorkflowTool,
        ]),
        flow_kind: signal_flow::FlowKind::Regular,
        tool_set: signal_flow::ToolSet::Default,
        instruction_prompt: "Carry this bounded launch request.".into(),
    };
    let receipt_request = signal_flow::TargetReceiptRequest {
        launch_request_id: profile.launch_request_id.clone(),
        prompt_sha256: "0286307646b3fb93a6e70d7012eaa2d07239bb1eb7c81a668fe1d4b6f3d97b3b".into(),
    };
    let composed = signal_flow::ComposedLaunch {
        launch_profile: profile,
        harness_kind: signal_flow::HarnessKind::Codex,
        system_prompt: signal_flow::SystemPrompt {
            system_prompt_text: "system text".into(),
            system_prompt_sha256:
                "1b8d0b2e3f3a6bd7c4bd3d3a5c8f1b5f2c1f6c7fd0a8e1f6f0b0c9d4e2a7b3c1".into(),
        },
        first_prompt_payload: signal_flow::FirstPromptPayload {
            first_prompt_body: "prompt body".into(),
            prompt_sha256: receipt_request.prompt_sha256.clone(),
            prompt_part_vector: vec![signal_flow::PromptPart {
                prompt_part_text: "prompt body".into(),
                prompt_sha256: receipt_request.prompt_sha256.clone(),
            }],
        },
        target_receipt_request: receipt_request,
        leave_outs: vec![
            signal_flow::LeaveOut::SkillTool,
            signal_flow::LeaveOut::WorkflowTool,
        ],
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
        prompt_sha256_vector: vec![
            "b2149f4ce39c3ec8984ca8e671514900470eaca3f132c30c89c12980f3ac3b08".into(),
            "c2149f4ce39c3ec8984ca8e671514900470eaca3f132c30c89c12980f3ac3b08".into(),
        ],
        parts_submitted: 1,
        stops_reported: 0,
        leave_outs: Vec::new(),
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
            system_modules: vec!["spirit:core".into()],
            user_modules: vec!["compensation:main-flow".into()],
            remembering: signal_flow::Remembering::Predecessors(0),
            address: signal_flow::Address {
                aspect: signal_flow::Aspect::Field,
                topic: "flow".into(),
                layer: signal_flow::Layer::Primary,
            },
            model_name: intent.model_name.clone(),
            effort: intent.effort.clone(),
            flow_id_option: None,
            herdr_session_name: "messaging-build".into(),
            leave_outs_option: None,
            flow_kind: signal_flow::FlowKind::Regular,
            tool_set: signal_flow::ToolSet::Default,
            instruction_prompt: "Carry this bounded launch request.".into(),
        },
        prompt_sha256: intent.prompt_sha256.clone(),
        origin: signal_flow::Origin::Flow(signal_flow::FlowOrigin {
            flow_id: "fac697".into(),
            session_id: "caller-session".into(),
        }),
        launch_attempt_phase: signal_flow::LaunchAttemptPhase::PromptIntentRecorded,
        native_launch_intent_option: Some(signal_flow::NativeLaunchIntent {
            launch_request_id: intent.launch_request_id.clone(),
            prompt_sha256: intent.prompt_sha256.clone(),
            system_prompt_sha256:
                "1b8d0b2e3f3a6bd7c4bd3d3a5c8f1b5f2c1f6c7fd0a8e1f6f0b0c9d4e2a7b3c1".into(),
            harness_kind: signal_flow::HarnessKind::Codex,
            model_name: intent.model_name.clone(),
            effort: intent.effort.clone(),
            prompt_sha256_vector: intent.prompt_sha256_vector.clone(),
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
fn agent_observation_carries_the_flow_and_its_herdr_state() {
    let query = Query::Observe(signal_flow::ObserveSelection::Agent("7d41e0".into()));
    let query_text = query.datomize(vec![]).protosize().compact();
    assert_eq!(query_text, "Observe.Agent.7d41e0");
    assert_eq!(
        Potential::<Query>::from(query_text)
            .actualize(&mut budget())
            .unwrap(),
        query
    );
    for (state, expected) in [
        (
            signal_flow::AgentState::Idle,
            "AgentObserved.{ 7d41e0 Idle }",
        ),
        (
            signal_flow::AgentState::Working,
            "AgentObserved.{ 7d41e0 Working }",
        ),
        (
            signal_flow::AgentState::Blocked,
            "AgentObserved.{ 7d41e0 Blocked }",
        ),
        (
            signal_flow::AgentState::Done,
            "AgentObserved.{ 7d41e0 Done }",
        ),
        (
            signal_flow::AgentState::Unknown,
            "AgentObserved.{ 7d41e0 Unknown }",
        ),
        (
            signal_flow::AgentState::Gone,
            "AgentObserved.{ 7d41e0 Gone }",
        ),
    ] {
        let response = Response::AgentObserved(signal_flow::AgentObservation {
            flow_id: "7d41e0".into(),
            agent_state: state,
        });
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
        let text = response.datomize(vec![]).protosize().compact();
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
fn a_send_names_delivery_and_deliver_all_names_a_metaflow() {
    // The recipient is a metaflow Address or Up, never a FlowId, and the
    // request carries no sender: Flow reads the caller from its socket.
    assert!(
        Potential::<Query>::from("Send.{ 7d41e0 «hello» }")
            .actualize(&mut budget())
            .is_err()
    );
    for text in [
        "Send.{ Address.{ Mind core Secondary } Queue Soft.Order.«Review the drain fix» }",
        "Send.{ Up Wake HardAbrupt.Question.«Which store?» }",
        "Send.{ Address.{ Psyche core Secondary } Wake MiddleAbrupt.Result.done }",
        "DeliverAll.{ Psyche core Secondary }",
        "Launch.{ { Mind core Primary } Predecessors.2 brief }",
        "Launch.{ { Mind core Primary } Custom.[ 36b71b e3c8c1 ] brief }",
        "BindSeat.{ Field core Secondary }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        assert_eq!(query.datomize(Vec::new()).protosize().compact(), text);
    }
    for text in ["Refused.OffRoute", "StartRejected.Undeclared"] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        assert_eq!(response.datomize(Vec::new()).protosize().compact(), text);
    }
}

#[test]
fn replace_and_launch_status_queries_have_concrete_datoms() {
    for text in [
        "Replace.{ request-8 [ spirit:core ] [ compensation:main-flow ] Predecessors.1 { Field flow Secondary } opus-5-5 high Some.fac697 messaging-build None Regular Default «Carry on from fac697.» }",
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
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn replace_carries_the_predecessor_in_its_launch_profile() {
    let text = "Replace.{ request-8 [ spirit:core ] [] Predecessors.1 { Field flow Secondary } opus-5-5 high Some.fac697 messaging-build None Regular Default «Carry on from fac697.» }";
    let Query::Replace(profile) = Potential::<Query>::from(text)
        .actualize(&mut budget())
        .unwrap()
    else {
        panic!("Replace");
    };
    assert_eq!(profile.flow_id_option, Some("fac697".to_string()));
}

#[test]
fn replace_and_launch_status_responses_have_concrete_datoms() {
    for text in [
        "Replaced.{ fac697 { 908786 session-2 Available.{ default claude-a1 w1:p4 term-4 } Flow.{ fac697 session-1 } } }",
        "Replaced.{ fac697 { 908786 session-2 Unavailable Unbound } }",
        "ReplaceRejected.PredecessorAbsent",
        "ReplaceRejected.UnknownPredecessor",
        "ReplaceRejected.PredecessorStopped",
        "ReplaceRejected.LaunchRefused.NativeLaunchRefused.{ Naming PaneLabelRefused }",
        "ReplaceRejected.LaunchRefused.NativeLaunchRefused.{ Naming WorkspaceLabelRefused }",
        "ReplaceRejected.ReapRefused.CloseRefused",
        "ReplaceRejected.Locked",
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
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn refresh_answers_have_concrete_datoms() {
    for text in [
        "Refreshed.{ { Psyche core Primary } 74e4d1 }",
        "RefreshRejected.NotAwake",
        "RefreshRejected.Locked",
        "RefreshRejected.CloseRefused",
        "RefreshRejected.PersistenceRefused",
        "RefreshRejected.HerdrUnavailable",
        "RefreshRejected.SuccessorRefused.LaunchRefused.Undeclared",
        "RefreshRejected.SuccessorRefused.ReapRefused.CloseRefused",
        "RefreshRejected.Undeclared",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn a_refused_harness_start_carries_its_cause_and_its_opened_thread() {
    for text in [
        "StartRejected.HarnessSpawnRefused.{ Herdr.{ Some.invalid_agent_argument «agent arguments cannot be encoded safely for the target shell» } None }",
        "StartRejected.HarnessSpawnRefused.{ Herdr.{ None «missing value for --kind» } Some.{ 01a14773-f58c-7ca1-978b-1ac675d19399 Answered Archived } }",
        "StartRejected.HarnessSpawnRefused.{ ControlCharacter None }",
        "StartRejected.HarnessSpawnRefused.{ CommandTooLong Some.{ thread-1 InProgress ArchiveRefused } }",
        "StartRejected.NativeLaunchRefused.{ Bind NativeSessionMissing }",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
    let Response::StartRejected(signal_flow::StartRejection::HarnessSpawnRefused(refusal)) =
        Potential::<Response>::from(
            "StartRejected.HarnessSpawnRefused.{ Herdr.{ Some.timeout «timed out waiting for agent startup» } None }",
        )
        .actualize(&mut budget())
        .unwrap()
    else {
        panic!("HarnessSpawnRefused");
    };
    assert_eq!(
        refusal.spawn_refusal_cause,
        signal_flow::SpawnRefusalCause::Herdr(signal_flow::HerdrRefusal {
            herdr_error_code_option: Some("timeout".into()),
            herdr_error_message: "timed out waiting for agent startup".into(),
        })
    );
}

#[test]
fn a_refused_registration_and_a_refused_native_step_carry_their_stage() {
    for text in [
        "StartRejected.RegistrationRefused.RouteUnverified",
        "StartRejected.RegistrationRefused.RoleConflict",
        "StartRejected.RegistrationRefused.RoleUnrecorded",
        "StartRejected.RegistrationRefused.PaneBindingForeign",
        "StartRejected.RegistrationRefused.AcknowledgementMismatch",
        "StartRejected.RegistrationRefused.PromptNotCanonical",
        "StartRejected.RegistrationRefused.NativeTarget.HerdrUnreadable",
        "StartRejected.RegistrationRefused.NativeTarget.IdentityInexact",
        "StartRejected.RegistrationRefused.TranscriptBoundary",
        "StartRejected.RegistrationRefused.ThreadNotNamed",
        "StartRejected.NativeLaunchRefused.{ Open AdapterRefused }",
        "StartRejected.NativeLaunchRefused.{ CodexThread AdapterRefused }",
        "StartRejected.NativeLaunchRefused.{ Receipt PartDiffers }",
        "StartRejected.NativeLaunchRefused.{ Receipt ModelOrEffortDiffers }",
        "StartRejected.NativeLaunchRefused.{ Confirm SessionUnrecorded }",
        "StartRejected.NativeLaunchRefused.{ Unclassified Unclassified }",
        "StartRejected.NativeLaunchRefused.{ Naming WorkspaceLabelRefused }",
        "StartRejected.RegistrationRefused.Unclassified",
        "ReplaceRejected.LaunchRefused.RegistrationRefused.NativeTarget.NoNativeSession",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
    let Response::StartRejected(signal_flow::StartRejection::NativeLaunchRefused(refusal)) =
        Potential::<Response>::from("StartRejected.NativeLaunchRefused.{ Receipt LeaveOutShown }")
            .actualize(&mut budget())
            .unwrap()
    else {
        panic!("NativeLaunchRefused");
    };
    assert_eq!(
        refusal,
        signal_flow::NativeLaunchRefusal {
            native_launch_stage: signal_flow::NativeLaunchStage::Receipt,
            native_launch_failure: signal_flow::NativeLaunchFailure::LeaveOutShown,
        }
    );
}

#[test]
fn a_start_whose_model_is_not_the_configured_one_is_refused_with_its_reason() {
    for text in [
        "StartRejected.ModelRefused.Unconfigured",
        "StartRejected.ModelRefused.Differs.claude-opus-5-5",
        "ReplaceRejected.LaunchRefused.ModelRefused.Differs.gpt-6-sol",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
    assert_eq!(
        Potential::<Response>::from("StartRejected.ModelRefused.Differs.gpt-6-sol")
            .actualize(&mut budget())
            .unwrap(),
        Response::StartRejected(signal_flow::StartRejection::ModelRefused(
            signal_flow::ModelRefusal::Differs("gpt-6-sol".into())
        ))
    );
}

#[test]
fn replaced_names_the_stopped_predecessor_and_the_started_successor() {
    let reply = Response::Replaced(signal_flow::Replaced {
        flow_id: "fac697".into(),
        launched: signal_flow::Launched {
            flow_id: "908786".into(),
            session_id: "session-2".into(),
            herdr_route_selection: signal_flow::HerdrRouteSelection::Available(
                signal_flow::HerdrRoute {
                    herdr_session_name: "default".into(),
                    herdr_agent_name: "claude-a1".into(),
                    herdr_pane_id: "w1:p4".into(),
                    herdr_terminal_id: "term-4".into(),
                },
            ),
            origin: signal_flow::Origin::Flow(signal_flow::FlowOrigin {
                flow_id: "fac697".into(),
                session_id: "session-1".into(),
            }),
        },
    });
    assert_eq!(
        reply.datomize(vec![]).protosize().compact(),
        "Replaced.{ fac697 { 908786 session-2 Available.{ default claude-a1 w1:p4 term-4 } Flow.{ fac697 session-1 } } }"
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
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
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
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn caller_names_the_flow_its_aspect_power_and_model() {
    let reply = Response::CallerResolved(signal_flow::Caller {
        flow_id: "38de5b".into(),
        aspect: signal_flow::Aspect::Psyche,
        power_level: signal_flow::PowerLevel::High,
        model_name: "claude-opus-5-5".into(),
    });
    assert_eq!(
        reply.datomize(vec![]).protosize().compact(),
        "CallerResolved.{ 38de5b Psyche High claude-opus-5-5 }"
    );
}

/// Metaflows lists preserve the address, current state, past flows and queued
/// requests as one typed record. The concrete text is the specification.
#[test]
fn metaflows_query_and_listed_metaflow_roundtrip() {
    let query = Potential::<Query>::from("Metaflows")
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(query, Query::Metaflows);

    let text = "Listed.[ { { Psyche flow Primary } Awake.f1c841 [ past-1 ] [ { logical-1 Sender.{ Mind flow Tertiary } Wake Soft.Order.continue } { logical-2 Unknown Queue Soft.Notice.kept } { logical-3 Flow Queue Soft.Result.landed } ] } ]";
    let restored = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    let Response::Listed(rows) = &restored else {
        panic!("Metaflows answers with typed metaflow records")
    };
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].address.aspect, Aspect::Psyche);
    assert_eq!(rows[0].address.topic, "flow");
    assert_eq!(rows[0].state, flow_ethos::State::Awake("f1c841".into()));
    assert_eq!(rows[0].past, vec!["past-1".to_owned()]);
    assert_eq!(
        rows[0].queue,
        vec![
            flow_ethos::QueueEntry {
                logical_delivery_id: "logical-1".into(),
                delivery: flow_ethos::Delivery::Wake,
                sent_by: flow_ethos::SentBy::Sender(flow_ethos::Address {
                    aspect: Aspect::Mind,
                    topic: "flow".into(),
                    layer: flow_ethos::Layer::Tertiary,
                }),
                message: flow_ethos::Message::Soft(flow_ethos::Request::Order("continue".into())),
            },
            flow_ethos::QueueEntry {
                logical_delivery_id: "logical-2".into(),
                delivery: flow_ethos::Delivery::Queue,
                sent_by: flow_ethos::SentBy::Unknown,
                message: flow_ethos::Message::Soft(flow_ethos::Request::Notice("kept".into())),
            },
            flow_ethos::QueueEntry {
                logical_delivery_id: "logical-3".into(),
                delivery: flow_ethos::Delivery::Queue,
                sent_by: flow_ethos::SentBy::Flow,
                message: flow_ethos::Message::Soft(flow_ethos::Request::Result("landed".into())),
            },
        ]
    );
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&restored).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        restored
    );
    assert_eq!(restored.datomize(vec![]).protosize().compact(), text);
}

/// Store failures retain their diagnostic as a typed Signal payload.
#[test]
fn store_refusal_roundtrips_with_its_error_text() {
    let text = "Refused.Store.«database unavailable»";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(
        response,
        Response::Refused(Refusal::Store("database unavailable".into()))
    );
    assert_eq!(response.datomize(vec![]).protosize().compact(), text);
}

/// A sleeping metaflow retains its queue without claiming an awake flow.
#[test]
fn listed_asleep_metaflow_roundtrips_without_a_current_flow() {
    let text = "Listed.[ { { Mind topics Primary } Asleep [] [] } ]";
    let restored = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    let Response::Listed(rows) = &restored else {
        panic!("Metaflows answers with typed metaflow records")
    };
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].state, flow_ethos::State::Asleep);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&restored).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        restored
    );
    assert_eq!(restored.datomize(vec![]).protosize().compact(), text);
}

#[test]
fn start_names_the_address_of_the_seat_it_launches() {
    let text = "Start.{ field-flow-quaternary-1 [ spirit:core ] [ compensation:main-flow ] Custom.[ fac697 ] { Field flow Quaternary } haiku medium None fieldFlow None Regular Default «Send the Order through Message.» }";
    let Query::Start(profile) = Potential::<Query>::from(text)
        .actualize(&mut budget())
        .unwrap()
    else {
        panic!("a Start query")
    };
    assert_eq!(
        profile.address,
        signal_flow::Address {
            aspect: Aspect::Field,
            topic: "flow".into(),
            layer: signal_flow::Layer::Quaternary,
        }
    );
    assert_eq!(profile.model_name, "haiku");
    assert_eq!(profile.effort, "medium");
    for text in [
        "StartRejected.AddressAwake",
        "StartRejected.NoSecondary",
        "StartRejected.TopicRefused.FlowRepair",
        "StartRejected.ContextRefused.MetaflowMismatch",
        "StartRejected.ContextRefused.ModuleTooLarge",
        "StartRejected.LeaveOutRefused.Indispensable.ToolSearch",
        "StartRejected.LeaveOutRefused.NeedsWorkflowLeftOut.SkillTool",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
}

/// A topic never dies: a metaflow is Awake or Asleep. The End query, its
/// Ended answer, the Ended refusal, an Ended state and the AddressEnded
/// rejection are no datom of this contract.
#[test]
fn an_ended_metaflow_is_no_datom_of_the_contract() {
    assert!(
        Potential::<Query>::from("End.{ Psyche flow Primary }")
            .actualize(&mut budget())
            .is_err(),
        "End is refused"
    );
    for text in [
        "Ended",
        "Current.Ended",
        "Refused.Ended.{ Psyche flow Primary }",
        "Listed.[ { { Psyche flow Primary } Ended [] [] } ]",
        "StartRejected.AddressEnded",
    ] {
        assert!(
            Potential::<Response>::from(text)
                .actualize(&mut budget())
                .is_err(),
            "{text} is refused"
        );
    }
    for text in ["Current.Asleep", "Current.Awake.f1c841", "Current.Unknown"] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
}

/// A Start of a Primary whose Secondary metaflow is not recorded is refused
/// with NoSecondary.
#[test]
fn no_secondary_start_rejection_round_trips() {
    let response = Potential::<Response>::from("StartRejected.NoSecondary")
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(
        response,
        Response::StartRejected(signal_flow::StartRejection::NoSecondary)
    );
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        response
    );
}

/// A send server's identification names its process and the Codex thread
/// the call was stamped with; a process that is no trusted send server is
/// refused with that process.
#[test]
fn identify_thread_and_its_refusal_have_concrete_datoms() {
    let text = "IdentifyThread.{ { 4242 987654 } 01a0fdcc-c1a1-7f43-8c23-06d41fa34438 }";
    let query = Potential::<Query>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(
        query,
        Query::IdentifyThread(signal_flow::IdentifyThread_Data {
            process: signal_flow::Process {
                pid: 4242,
                started: 987654,
            },
            native_session_id: "01a0fdcc-c1a1-7f43-8c23-06d41fa34438".into(),
        })
    );
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    for text in [
        "Identified.{ Mind flow Secondary }",
        "Refused.NotSendServer.{ 4242 987654 }",
        "Refused.Unidentified.{ 4242 987654 }",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn durable_admission_and_resumable_sender_outcomes_round_trip() {
    for text in [
        "Send.{ Address.{ Mind flow Secondary } Wake MiddleAbrupt.Notice.ready }",
        "ObserveSends.None",
        "ObserveSender.{ { 17 99 } None Some.4 }",
    ] {
        let value = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(value.datomize(vec![]).protosize().compact(), text);
    }
    for text in [
        "SendAccepted.{ logical-1 1 Queued }",
        "SendAccepted.{ logical-1 4 Retryable.OwnerResolvedAbsent }",
        "SendsObserved.{ 2 [ { 2 logical-1 Sender.{ Mind flow Secondary } { Psyche flow Secondary } Attempting } ] }",
        "SendChanged.{ 3 logical-1 Sender.{ Mind flow Secondary } { Psyche flow Secondary } Delivered.{ Claude session-1 prompt-1 } }",
        "SendChanged.{ 4 logical-2 Sender.{ Mind flow Secondary } { Psyche flow Secondary } Withdrawn }",
        "Drained.{ { Psyche flow Secondary } Held.[ logical-1 ] }",
    ] {
        let value = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            value
        );
        assert_eq!(value.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn direct_send_replies_carry_receipt_or_failure_never_queue_state() {
    use signal_flow::{SendFailure, SendFailureReason, SendReceipt};
    let replies = [
        Response::Sent(SendReceipt {
            logical_delivery_id: "logical-direct-1".into(),
            native_receipt: signal_flow::NativeReceipt {
                harness_kind: signal_flow::HarnessKind::Codex,
                native_session_id: "native-direct-1".into(),
                native_prompt_id: "prompt-direct-1".into(),
            },
        }),
        Response::Failed(SendFailure {
            logical_delivery_id_option: None,
            send_failure_reason: SendFailureReason::QueueNotPermitted,
        }),
        Response::Failed(SendFailure {
            logical_delivery_id_option: Some("logical-direct-2".into()),
            send_failure_reason: SendFailureReason::ReceiptUncertain,
        }),
    ];
    for reply in replies {
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            reply
        );
        let text = reply.datomize(vec![]).protosize().compact();
        assert_eq!(
            Potential::<Response>::from(text)
                .actualize(&mut budget())
                .unwrap(),
            reply
        );
    }
}

/// A Start whose topic is not an uncapitalized camelCase identifier is
/// refused with TopicRefused, carrying the topic as written.
#[test]
fn topic_refused_start_rejection_round_trips() {
    let text = "StartRejected.TopicRefused.PrimaryCommitting";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(
        response,
        Response::StartRejected(signal_flow::StartRejection::TopicRefused(
            "PrimaryCommitting".into()
        ))
    );
    assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        response
    );
}
