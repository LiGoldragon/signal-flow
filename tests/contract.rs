#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use protos::{Compactable, Protosizable, ReaderBudget};
use signal_flow::{Aspect, Query, Refusal, Response};

fn budget() -> Budget {
    Budget {
        remaining: 4096,
        reader: ReaderBudget::try_from(4096).unwrap(),
        depth: 0,
        maximum_depth: 1024,
    }
}

#[test]
fn missing_module_refusal_carries_known_or_historical_identity() {
    for (text, expected) in [
        (
            "ModuleMissing.Some.vision:absent",
            signal_flow::ContextRefusal::ModuleMissing(Some(signal_flow::ModuleIdentity(
                "vision:absent".to_string(),
            ))),
        ),
        (
            "ModuleMissing.None",
            signal_flow::ContextRefusal::ModuleMissing(None),
        ),
    ] {
        let actual = Potential::<signal_flow::ContextRefusal>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(actual, expected);
        assert_eq!(actual.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn every_ordinary_request_has_a_concrete_datom() {
    for text in [
        "Start.{ request-7 [ spirit:core ] [ compensation:main-flow knowledge:flow ] Custom.[ 4e1d2a ] { Field flow Astra } gpt-6-astra medium Some.836818 messaging-build Some.[ SkillTool WorkflowTool ] Regular Default «Carry this bounded launch request.» }",
        "Start.{ field-flow-quaternary-1 [ spirit:core ] [ compensation:main-flow ] Predecessors.0 { Field flow Haiku } haiku medium None fieldFlow None Regular Default «Send the Order through Message.» }",
        "Refresh.{ Psyche core Fable }",
        "ResolveRecipient.fac697",
        "Adopt.{ { Mind core Sol } { main w1:p3 } 6c9e081f-70b0-4a35-9838-6cc7dca01971 }",
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
        flow_id: signal_flow::FlowId("fac697".to_string()),
        session_id: signal_flow::SessionId("session-1".to_string()),
        harness_kind: signal_flow::HarnessKind::Codex,
        endpoint_selection: signal_flow::EndpointSelection::Available(
            signal_flow::Available_Data {
                endpoint_path: signal_flow::EndpointPath("/run/user/1001/codex.sock".to_string()),
                route_readiness: signal_flow::RouteReadiness::Ready,
            },
        ),
        herdr_route_selection: signal_flow::HerdrRouteSelection::Available(
            signal_flow::HerdrRoute {
                herdr_session_name: signal_flow::HerdrSessionName("messaging-build".to_string()),
                herdr_agent_name: signal_flow::HerdrAgentName("psyche-mind-astra".to_string()),
                herdr_pane_id: signal_flow::HerdrPaneId("w1:p3".to_string()),
                herdr_terminal_id: signal_flow::HerdrTerminalId("term-fixture".to_string()),
            },
        ),
        origin: signal_flow::Origin::Flow(signal_flow::FlowOrigin {
            flow_id: signal_flow::FlowId("parent".to_string()),
            session_id: signal_flow::SessionId("session-0".to_string()),
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
        launch_request_id: signal_flow::LaunchRequestId("request-7".to_string()),
        system_modules: signal_flow::SystemModules(vec![signal_flow::ModuleIdentity(
            "spirit:core".to_string(),
        )]),
        prompt_modules: signal_flow::PromptModules(vec![
            signal_flow::ModuleIdentity("compensation:main-flow".to_string()),
            signal_flow::ModuleIdentity("knowledge:flow".to_string()),
        ]),
        remembering: signal_flow::Remembering::Custom(vec![signal_flow::FlowId(
            "4e1d2a".to_string(),
        )]),
        address: signal_flow::Address {
            aspect: signal_flow::Aspect::Field,
            topic: signal_flow::Topic("flow".to_string()),
            model: signal_flow::Model::Fable,
        },
        model_name: signal_flow::ModelName("gpt-6-astra".to_string()),
        effort: signal_flow::Effort("medium".to_string()),
        flow_id_option: Some(signal_flow::FlowId("836818".to_string())),
        herdr_session_name: signal_flow::HerdrSessionName("messaging-build".to_string()),
        leave_outs_option: Some(signal_flow::LeaveOuts(vec![
            signal_flow::LeaveOut::SkillTool,
            signal_flow::LeaveOut::WorkflowTool,
        ])),
        flow_kind: signal_flow::FlowKind::Regular,
        tool_set: signal_flow::ToolSet::Default,
        instruction_prompt: signal_flow::InstructionPrompt(
            "Carry this bounded launch request.".to_string(),
        ),
    };
    let receipt_request = signal_flow::TargetReceiptRequest {
        launch_request_id: profile.launch_request_id.clone(),
        prompt_sha256: signal_flow::PromptSha256(
            "0286307646b3fb93a6e70d7012eaa2d07239bb1eb7c81a668fe1d4b6f3d97b3b".to_string(),
        ),
    };
    let composed = signal_flow::ComposedLaunch {
        launch_profile: profile,
        harness_kind: signal_flow::HarnessKind::Codex,
        system_prompt: signal_flow::SystemPrompt {
            system_prompt_text: signal_flow::SystemPromptText("system text".to_string()),
            system_prompt_sha256: signal_flow::SystemPromptSha256(
                "1b8d0b2e3f3a6bd7c4bd3d3a5c8f1b5f2c1f6c7fd0a8e1f6f0b0c9d4e2a7b3c1".to_string(),
            ),
        },
        first_prompt_payload: signal_flow::FirstPromptPayload {
            first_prompt_body: signal_flow::FirstPromptBody("prompt body".to_string()),
            prompt_sha256: receipt_request.prompt_sha256.clone(),
            prompt_part_vector: vec![signal_flow::PromptPart {
                prompt_part_text: signal_flow::PromptPartText("prompt body".to_string()),
                prompt_sha256: receipt_request.prompt_sha256.clone(),
            }],
        },
        target_receipt_request: receipt_request,
        leave_outs: signal_flow::LeaveOuts(vec![
            signal_flow::LeaveOut::SkillTool,
            signal_flow::LeaveOut::WorkflowTool,
        ]),
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
        launch_request_id: signal_flow::LaunchRequestId("request-7".to_string()),
        herdr_session_name: signal_flow::HerdrSessionName("messaging-build".to_string()),
        herdr_agent_name: signal_flow::HerdrAgentName("field-high-of-836818".to_string()),
        herdr_workspace_id: signal_flow::HerdrWorkspaceId("workspace-1".to_string()),
        herdr_pane_id: signal_flow::HerdrPaneId("w1:p4".to_string()),
        herdr_terminal_id: signal_flow::HerdrTerminalId("term-4".to_string()),
    };
    let intent = signal_flow::PromptDeliveryIntent {
        launch_request_id: signal_flow::LaunchRequestId("request-7".to_string()),
        prompt_sha256: signal_flow::PromptSha256(
            "0286307646b3fb93a6e70d7012eaa2d07239bb1eb7c81a668fe1d4b6f3d97b3b".to_string(),
        ),
        flow_id: signal_flow::FlowId("908786".to_string()),
        native_session_id: signal_flow::NativeSessionId("native-session-1".to_string()),
        harness_kind: signal_flow::HarnessKind::Codex,
        model_name: signal_flow::ModelName("gpt-6-astra".to_string()),
        effort: signal_flow::Effort("medium".to_string()),
        prompt_sha256_vector: vec![
            signal_flow::PromptSha256(
                "b2149f4ce39c3ec8984ca8e671514900470eaca3f132c30c89c12980f3ac3b08".to_string(),
            ),
            signal_flow::PromptSha256(
                "c2149f4ce39c3ec8984ca8e671514900470eaca3f132c30c89c12980f3ac3b08".to_string(),
            ),
        ],
        parts_submitted: signal_flow::PartsSubmitted(1),
        stops_reported: signal_flow::StopsReported(0),
        leave_outs: signal_flow::LeaveOuts(Vec::new()),
        herdr_pane_binding: binding,
        native_transcript_boundary: signal_flow::NativeTranscriptBoundary::Existing(
            signal_flow::NativeTranscriptCursor {
                native_session_id: signal_flow::NativeSessionId("native-session-1".to_string()),
                harness_kind: signal_flow::HarnessKind::Codex,
                transcript_device: signal_flow::TranscriptDevice("2049".to_string()),
                transcript_inode: signal_flow::TranscriptInode("99142".to_string()),
                transcript_byte_offset: signal_flow::TranscriptByteOffset(4096),
                transcript_prefix_sha256: signal_flow::TranscriptPrefixSha256(
                    "a2149f4ce39c3ec8984ca8e671514900470eaca3f132c30c89c12980f3ac3b07".to_string(),
                ),
            },
        ),
    };
    let attempt = signal_flow::LaunchAttempt {
        launch_request_id: intent.launch_request_id.clone(),
        launch_profile: signal_flow::LaunchProfile {
            launch_request_id: intent.launch_request_id.clone(),
            system_modules: signal_flow::SystemModules(vec![signal_flow::ModuleIdentity(
                "spirit:core".to_string(),
            )]),
            prompt_modules: signal_flow::PromptModules(vec![signal_flow::ModuleIdentity(
                "compensation:main-flow".to_string(),
            )]),
            remembering: signal_flow::Remembering::Predecessors(signal_flow::RememberingDepth(0)),
            address: signal_flow::Address {
                aspect: signal_flow::Aspect::Field,
                topic: signal_flow::Topic("flow".to_string()),
                model: signal_flow::Model::Fable,
            },
            model_name: intent.model_name.clone(),
            effort: intent.effort.clone(),
            flow_id_option: None,
            herdr_session_name: signal_flow::HerdrSessionName("messaging-build".to_string()),
            leave_outs_option: None,
            flow_kind: signal_flow::FlowKind::Regular,
            tool_set: signal_flow::ToolSet::Default,
            instruction_prompt: signal_flow::InstructionPrompt(
                "Carry this bounded launch request.".to_string(),
            ),
        },
        prompt_sha256: intent.prompt_sha256.clone(),
        origin: signal_flow::Origin::Flow(signal_flow::FlowOrigin {
            flow_id: signal_flow::FlowId("fac697".to_string()),
            session_id: signal_flow::SessionId("caller-session".to_string()),
        }),
        launch_attempt_phase: signal_flow::LaunchAttemptPhase::PromptIntentRecorded,
        native_launch_intent_option: Some(signal_flow::NativeLaunchIntent {
            launch_request_id: intent.launch_request_id.clone(),
            prompt_sha256: intent.prompt_sha256.clone(),
            system_prompt_sha256: signal_flow::SystemPromptSha256(
                "1b8d0b2e3f3a6bd7c4bd3d3a5c8f1b5f2c1f6c7fd0a8e1f6f0b0c9d4e2a7b3c1".to_string(),
            ),
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
            native_session_id: signal_flow::NativeSessionId("native-session-2".to_string()),
            harness_kind: signal_flow::HarnessKind::Claude,
            transcript_root_device: signal_flow::TranscriptRootDevice("2049".to_string()),
            transcript_root_inode: signal_flow::TranscriptRootInode("99143".to_string()),
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
    let query = Query::Observe(signal_flow::ObserveSelection::Agent(signal_flow::FlowId(
        "7d41e0".to_string(),
    )));
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
            flow_id: signal_flow::FlowId("7d41e0".to_string()),
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
fn retirement_observation_carries_exact_generation_and_archive_identity() {
    use signal_flow::{
        Address, ArchiveCommitted, HandoverArchiveIdentity, HandoverGenerationKey, Model,
        ObserveSelection, RetirementChange, RetirementObservationFailure, RetirementSnapshot,
    };
    let address = Address {
        aspect: Aspect::Mind,
        topic: signal_flow::Topic("flowNexus".to_string()),
        model: Model::Sol,
    };
    let key = HandoverGenerationKey {
        address: address.clone(),
        handover_generation: signal_flow::HandoverGeneration(7),
        predecessor: signal_flow::Predecessor(signal_flow::FlowId("abc123".to_string())),
        successor: signal_flow::Successor(signal_flow::FlowId("def456".to_string())),
    };
    let values = [
        Query::Observe(ObserveSelection::Retirement(address.clone()))
            .datomize(vec![])
            .protosize()
            .compact(),
        Response::RetirementObserved(RetirementSnapshot {
            address,
            handover_generation_key_vector: vec![key.clone()],
        })
        .datomize(vec![])
        .protosize()
        .compact(),
        Response::RetirementChanged(RetirementChange::ArchiveCommitted(ArchiveCommitted {
            handover_generation_key: key,
            handover_archive_identity: HandoverArchiveIdentity {
                git_commit: signal_flow::GitCommit("commit".to_string()),
                archive_document_path: signal_flow::ArchiveDocumentPath(
                    "Mind/flowNexus/Sol/abc123.md".to_string(),
                ),
                git_blob: signal_flow::GitBlob("blob".to_string()),
            },
        }))
        .datomize(vec![])
        .protosize()
        .compact(),
        Response::RetirementObservationFailed(RetirementObservationFailure::Overflow)
            .datomize(vec![])
            .protosize()
            .compact(),
    ];
    assert_eq!(values[0], "Observe.Retirement.{ Mind flowNexus Sol }");
    for text in values {
        if text.starts_with("Observe") {
            let value = Potential::<Query>::from(text.clone())
                .actualize(&mut budget())
                .unwrap();
            let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
            assert_eq!(
                rkyv::from_bytes::<Query, rkyv::rancor::Error>(&bytes).unwrap(),
                value
            );
        } else {
            let value = Potential::<Response>::from(text)
                .actualize(&mut budget())
                .unwrap();
            let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
            assert_eq!(
                rkyv::from_bytes::<Response, rkyv::rancor::Error>(&bytes).unwrap(),
                value
            );
        }
    }
}

#[test]
fn a_send_names_its_recipient_and_deliver_all_names_a_metaflow() {
    // The recipient is a metaflow Address, never a FlowId nor a route
    // relative to the sender, and the request carries no sender: Flow reads
    // the caller from its socket.
    for text in [
        "Send.{ 7d41e0 «hello» }",
        "Send.{ Up HardAbrupt.Question.«Which store?» }",
        "Send.{ Address.{ Mind core Sol } Soft.Order.«Review the drain fix» }",
    ] {
        assert!(
            Potential::<Query>::from(text)
                .actualize(&mut budget())
                .is_err(),
            "{text} is refused"
        );
    }
    for text in [
        "Send.{ { Mind core Sol } Soft.Order.«Review the drain fix» }",
        "DeliverAll.{ Psyche core Opus }",
        "Launch.{ { Mind core Astra } Predecessors.2 brief }",
        "Launch.{ { Mind core Astra } Custom.[ 36b71b e3c8c1 ] brief }",
        "BindSeat.{ { Field core Sol } { main w1:p3 } }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        assert_eq!(query.datomize(Vec::new()).protosize().compact(), text);
    }
    for text in [
        "StartRejected.Undeclared",
        "Refused.TopicRefused.Primary_Committing",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        assert_eq!(response.datomize(Vec::new()).protosize().compact(), text);
    }
}

#[test]
fn replace_and_launch_status_queries_have_concrete_datoms() {
    for text in [
        "Replace.{ request-8 [ spirit:core ] [ compensation:main-flow ] Predecessors.1 { Field flow Sol } opus-5-5 high Some.fac697 messaging-build None Regular Default «Carry on from fac697.» }",
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
    let text = "Replace.{ request-8 [ spirit:core ] [] Predecessors.1 { Field flow Sol } opus-5-5 high Some.fac697 messaging-build None Regular Default «Carry on from fac697.» }";
    let Query::Replace(profile) = Potential::<Query>::from(text)
        .actualize(&mut budget())
        .unwrap()
    else {
        panic!("Replace");
    };
    assert_eq!(
        profile.flow_id_option,
        Some(signal_flow::FlowId("fac697".to_string()))
    );
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
        "Refreshed.{ { Psyche core Fable } 74e4d1 }",
        "RefreshQueued.{ { Psyche core Fable } 74e4d1 }",
        "RefreshRejected.NotAwake",
        "RefreshRejected.Locked",
        "RefreshRejected.CloseRefused",
        "RefreshRejected.PersistenceRefused",
        "RefreshRejected.HerdrUnavailable",
        "RefreshRejected.SuccessorRefused.LaunchRefused.Undeclared",
        "RefreshRejected.Undeclared",
        "RefreshRejected.SuccessorUnconfirmed",
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
            herdr_error_code_option: Some(signal_flow::HerdrErrorCode("timeout".to_string())),
            herdr_error_message: signal_flow::HerdrErrorMessage(
                "timed out waiting for agent startup".to_string()
            ),
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
            signal_flow::ModelRefusal::Differs(signal_flow::ModelName("gpt-6-sol".to_string()))
        ))
    );
}

#[test]
fn replaced_names_the_stopped_predecessor_and_the_started_successor() {
    let reply = Response::Replaced(signal_flow::Replaced {
        flow_id: signal_flow::FlowId("fac697".to_string()),
        launched: signal_flow::Launched {
            flow_id: signal_flow::FlowId("908786".to_string()),
            session_id: signal_flow::SessionId("session-2".to_string()),
            herdr_route_selection: signal_flow::HerdrRouteSelection::Available(
                signal_flow::HerdrRoute {
                    herdr_session_name: signal_flow::HerdrSessionName("default".to_string()),
                    herdr_agent_name: signal_flow::HerdrAgentName("claude-a1".to_string()),
                    herdr_pane_id: signal_flow::HerdrPaneId("w1:p4".to_string()),
                    herdr_terminal_id: signal_flow::HerdrTerminalId("term-4".to_string()),
                },
            ),
            origin: signal_flow::Origin::Flow(signal_flow::FlowOrigin {
                flow_id: signal_flow::FlowId("fac697".to_string()),
                session_id: signal_flow::SessionId("session-1".to_string()),
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
        "CallerResolved.{ fac697 Psyche claude-opus-5-5 }",
        "CallerResolutionRejected.CallerUnknown",
        "CallerResolutionRejected.CallerMismatch.{ fac697 Mind gpt-6-sol }",
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
fn launch_and_rehearse_share_one_request_and_have_typed_answers() {
    // datom-codec 0.33 prints a bare String bare: `Begin.` is the canonical
    // print of the brief the guillemet form `«Begin.»` also reads as.
    for text in [
        "Launch.{ { Psyche core Fable } Predecessors.1 Begin. }",
        "Rehearse.{ { Psyche core Fable } Predecessors.1 Begin. }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(
            Potential::<Query>::from(text.replace("Begin.", "«Begin.»"))
                .actualize(&mut budget())
                .unwrap(),
            query
        );
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Query, rkyv::rancor::Error>(&archive).unwrap(),
            query
        );
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    }
    for text in [
        "Rehearsed.«'herdr' '--session' 'main'»",
        "RehearseRejected.MalformedHarnessFlags.UnclosedSingleQuote",
        "StartRejected.MalformedHarnessFlags.TrailingEscape",
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn caller_names_the_flow_its_aspect_and_model() {
    let reply = Response::CallerResolved(signal_flow::CallerIdentity {
        flow_id: signal_flow::FlowId("38de5b".to_string()),
        aspect: signal_flow::Aspect::Psyche,
        model_name: signal_flow::ModelName("claude-opus-5-5".to_string()),
    });
    assert_eq!(
        reply.datomize(vec![]).protosize().compact(),
        "CallerResolved.{ 38de5b Psyche claude-opus-5-5 }"
    );
    assert!(
        Potential::<Response>::from("CallerResolved.{ 38de5b Psyche High claude-opus-5-5 }")
            .actualize(&mut budget())
            .is_err(),
        "a Caller carries no power"
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

    let text = "Listed.[ { { Psyche flow Fable } Awake.f1c841 [ past-1 ] [] } ]";
    let restored = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    let Response::Listed(rows) = &restored else {
        panic!("Metaflows answers with typed metaflow records")
    };
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].address.aspect, Aspect::Psyche);
    assert_eq!(
        rows[0].address.topic,
        signal_flow::Topic("flow".to_string())
    );
    assert_eq!(
        rows[0].state,
        flow_ethos::State::Awake(signal_flow::FlowId("f1c841".to_string()))
    );
    assert_eq!(
        rows[0].past,
        flow_ethos::Past(vec![signal_flow::FlowId("past-1".to_string())])
    );
    assert!(rows[0].queue.0.is_empty());
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
    let text = "Listed.[ { { Mind topics Astra } Asleep [] [] } ]";
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
    let text = "Start.{ field-flow-quaternary-1 [ spirit:core ] [ compensation:main-flow ] Custom.[ fac697 ] { Field flow Haiku } haiku medium None fieldFlow None Regular Default «Send the Order through Message.» }";
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
            topic: signal_flow::Topic("flow".to_string()),
            model: signal_flow::Model::Haiku,
        }
    );
    assert_eq!(
        profile.model_name,
        signal_flow::ModelName("haiku".to_string())
    );
    assert_eq!(profile.effort, signal_flow::Effort("medium".to_string()));
    for text in [
        "StartRejected.AddressAwake",
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
        Potential::<Query>::from("End.{ Psyche flow Fable }")
            .actualize(&mut budget())
            .is_err(),
        "End is refused"
    );
    for text in [
        "Ended",
        "Current.Ended",
        "Refused.Ended.{ Psyche flow Fable }",
        "Listed.[ { { Psyche flow Fable } Ended [] [] } ]",
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

/// No model ranks above another: no Start waits for another model's seat,
/// and no Send is refused for a sender with nothing above it.
#[test]
fn no_rank_refusal_is_a_datom_of_the_contract() {
    for text in [
        "StartRejected.NoSecondary",
        "Refused.NoneAbove",
        "Refused.NoLayer",
        "SendChanged.{ req1 1 Sender.{ Mind flow Sol } { Psyche flow Opus } Failed.NoneAbove }",
    ] {
        assert!(
            Potential::<Response>::from(text)
                .actualize(&mut budget())
                .is_err(),
            "{text} is refused"
        );
    }
    // The same failure carrying a refusal the contract keeps is a datom.
    let kept =
        "SendChanged.{ req1 1 Sender.{ Mind flow Sol } { Psyche flow Opus } Failed.NotConfigured }";
    let response = Potential::<Response>::from(kept)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(response.datomize(vec![]).protosize().compact(), kept);
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
                pid: flow_ethos::Pid(4242),
                started: flow_ethos::Started(987654),
            },
            native_session_id: signal_flow::NativeSessionId(
                "01a0fdcc-c1a1-7f43-8c23-06d41fa34438".to_string()
            ),
        })
    );
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    for text in [
        "Identified.{ Mind flow Sol }",
        "Refused.NotSendServer.{ 4242 987654 }",
        "Refused.Unidentified.{ None None None 1761800000 { 4242 987654 } }",
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
        "Send.{ { Mind flow Sol } MiddleAbrupt.Notice.ready }",
        "ObserveSends.None",
        "ObserveSender.{ { 17 99 } None Some.4 }",
        "ObserveDeliveries.Some.4",
        "Acknowledge.[ { messageMemory 0 } { messageMemory 1 } ]",
    ] {
        let value = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(value.datomize(vec![]).protosize().compact(), text);
    }
    for text in [
        "SendAccepted.{ req1 1 Queued }",
        "SendAccepted.{ req4 4 Retryable.OwnerResolvedAbsent }",
        "SendsObserved.{ store1 2 [ { req2 2 Sender.{ Mind flow Sol } { Psyche flow Opus } Attempting } ] }",
        "SendChanged.{ req3 3 Sender.{ Mind flow Sol } { Psyche flow Opus } Delivered }",
        "SendChanged.{ req4 4 Sender.{ Mind flow Sol } { Psyche flow Opus } Withdrawn }",
        "SendChanged.{ req5 5 Sender.{ Mind flow Sol } { Psyche flow Opus } Retryable.WakeRefused.ContextRefused.RecordMissing }",
        "SendsObserved.{ store1 6 [ { req6 6 Sender.{ Mind flow Sol } { Psyche flow Opus } Retryable.WakeRefused.Undeclared } ] }",
        "Drained.{ { Psyche flow Opus } Held }",
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
    use signal_flow::{VisibleSendFailure, VisibleSendFailureReason};
    let replies = [
        Response::Sent(signal_flow::VisibleSendReceipt(signal_flow::RequestId(
            "direct1".to_string(),
        ))),
        Response::Failed(VisibleSendFailure::Unadmitted(
            VisibleSendFailureReason::QueueNotPermitted,
        )),
        Response::Failed(VisibleSendFailure::Unadmitted(
            VisibleSendFailureReason::ReceiptUncertain,
        )),
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
            signal_flow::Topic("PrimaryCommitting".to_string())
        ))
    );
    assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
        response
    );
}

/// A launch that could not be composed names why: an empty Custom
/// Remembering is InvalidRemembering, and every other reason is a variant
/// of its own.
#[test]
fn composition_refused_carries_its_reason() {
    use signal_flow::{CompositionRefusal, StartRejection};
    for (text, refusal) in [
        (
            "StartRejected.CompositionRefused.InvalidRemembering",
            CompositionRefusal::InvalidRemembering,
        ),
        (
            "StartRejected.CompositionRefused.InvalidSystemModules",
            CompositionRefusal::InvalidSystemModules,
        ),
        (
            "StartRejected.CompositionRefused.InexactModel",
            CompositionRefusal::InexactModel,
        ),
        (
            "StartRejected.CompositionRefused.Unconfigured",
            CompositionRefusal::Unconfigured,
        ),
        (
            "StartRejected.CompositionRefused.PresentationUnavailable",
            CompositionRefusal::PresentationUnavailable,
        ),
        (
            "StartRejected.CompositionRefused.Unrecorded",
            CompositionRefusal::Unrecorded,
        ),
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(
            response,
            Response::StartRejected(StartRejection::CompositionRefused(refusal))
        );
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
    }
    // The reason is not optional: a bare CompositionRefused no longer reads.
    assert!(
        Potential::<Response>::from("StartRejected.CompositionRefused")
            .actualize(&mut budget())
            .is_err()
    );
}

/// A Task with its Minutes replaces Remind: the ordinary socket reads no
/// Remind, answers no Reminded and refuses no unreadable instant.
#[test]
fn remind_is_gone_and_a_task_with_minutes_stands_in_its_place() {
    let remind = "Remind.{ 2026-10-29T23:30:00-06:00 «Check the capsule run.» }";
    assert!(
        Potential::<Query>::from(remind)
            .actualize(&mut budget())
            .is_err(),
        "{remind} still reads"
    );
    for text in ["Reminded", "Refused.InstantUnreadable.tomorrow"] {
        assert!(
            Potential::<Response>::from(text)
                .actualize(&mut budget())
                .is_err(),
            "{text} still reads"
        );
    }
    let task = "Send.{ { Mind core Sol } Soft.Task.{ 30 «Check the capsule run.» } }";
    let query = Potential::<Query>::from(task)
        .actualize(&mut budget())
        .unwrap_or_else(|error| panic!("{task}: {error:?}"));
    assert_eq!(query.datomize(Vec::new()).protosize().compact(), task);
}

/// Every refusal a Task meets at admission is carried whole:
/// datom text reads into the Response, the Response crosses the wire as
/// rkyv and reads back equal, and prints as the same text.
#[test]
fn task_refusals_round_trip_from_datom_through_rkyv_to_datom() {
    use signal_flow::MinutesRefusal;
    let refused = |refusal| Response::Refused(refusal);
    for (text, expected) in [
        (
            "Refused.MinutesRefused.Zero",
            refused(Refusal::MinutesRefused(MinutesRefusal::Zero)),
        ),
        (
            "Refused.MinutesRefused.Negative",
            refused(Refusal::MinutesRefused(MinutesRefusal::Negative)),
        ),
        (
            "Refused.MinutesRefused.Unreachable",
            refused(Refusal::MinutesRefused(MinutesRefusal::Unreachable)),
        ),
        // An abrupt Send's refusal is answered as its failure.
        (
            "Failed.Unadmitted.Refused.MinutesRefused.Zero",
            Response::Failed(signal_flow::VisibleSendFailure::Unadmitted(
                signal_flow::VisibleSendFailureReason::Refused(
                    signal_flow::VisibleRefusal::MinutesRefused(MinutesRefusal::Zero),
                ),
            )),
        ),
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        assert_eq!(response, expected, "{text}");
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        let carried = rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap();
        assert_eq!(carried, response, "{text}");
        assert_eq!(carried.datomize(Vec::new()).protosize().compact(), text);
    }
}

/// A sender nobody could identify stays unidentified: the refusal carries the
/// raw evidence read at the seat, never an invented identity. Datom to rkyv
/// to datom returns the same text.
#[test]
fn an_unidentified_refusal_carries_its_seat_evidence_round_trip() {
    for text in [
        "Refused.Unidentified.{ Some.{ herdr-1 w1-2 } Some.«Claude research» Some.01a0fdcc-c1a1 1761800000 { 4242 987654 } }",
        "Refused.Unidentified.{ Some.{ herdr-1 w1-2 } None None 1761800000 { 4242 987654 } }",
        "Refused.Unidentified.{ None None None 1761800000 { 4242 987654 } }",
    ] {
        let value = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
        let read = rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap();
        assert_eq!(read, value);
        assert_eq!(read.datomize(vec![]).protosize().compact(), text);
    }
    let refusal = Potential::<Refusal>::from(
        "Unidentified.{ Some.{ herdr-1 w1-2 } Some.«Claude research» None 1761800000 { 4242 987654 } }",
    )
    .actualize(&mut budget())
    .unwrap();
    assert!(matches!(refusal, Refusal::Unidentified(_)));
}

/// A frame that decodes as no Query is refused with what Flow read of its
/// caller, the flow bound to the caller's seat and the seat's evidence, and
/// Observe.Undecodable shows every such record. Datom to rkyv to datom
/// returns the same text.
#[test]
fn an_undecodable_frame_is_refused_with_its_caller_and_observed() {
    let query = Query::Observe(signal_flow::ObserveSelection::Undecodable);
    let query_text = query.datomize(vec![]).protosize().compact();
    assert_eq!(query_text, "Observe.Undecodable");
    assert_eq!(
        Potential::<Query>::from(query_text)
            .actualize(&mut budget())
            .unwrap(),
        query
    );
    for text in [
        "FrameRefused.{ Some.7d41e0 { Some.{ herdr-1 w1-2 } Some.«Claude research» Some.01a0fdcc-c1a1 1761800000 { 4242 987654 } } }",
        "FrameRefused.{ None { None None None 1761800000 { 4242 987654 } } }",
        "UndecodableObserved.[ { Some.7d41e0 { Some.{ herdr-1 w1-2 } None None 1761800000 { 4242 987654 } } } ]",
        "UndecodableObserved.[]",
    ] {
        let value = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
        let read = rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap();
        assert_eq!(read, value);
        assert_eq!(read.datomize(vec![]).protosize().compact(), text);
    }
}

/// Observe.TurnEnds subscribes to turn ends: the last of each harness on
/// open, then each one as it comes, naming the metaflow's Address, the
/// harness, the flow and when Flow observed it. Datom to rkyv to datom
/// returns the same text.
#[test]
fn a_turn_end_subscription_names_address_harness_and_flow_round_trip() {
    let query = Query::Observe(signal_flow::ObserveSelection::TurnEnds);
    let query_text = query.datomize(vec![]).protosize().compact();
    assert_eq!(query_text, "Observe.TurnEnds");
    assert_eq!(
        Potential::<Query>::from(query_text)
            .actualize(&mut budget())
            .unwrap(),
        query
    );
    for text in [
        "TurnEndsObserved.[ { { Psyche core Opus } Claude 7d41e0 1761800000 } { { Mind flow Sol } Codex 01a0fd 1761800100 } ]",
        "TurnEndsObserved.[]",
        "TurnEnded.{ { Field message Luna } Codex 74e4d1 1761800200 }",
    ] {
        let value = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
        let read = rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap();
        assert_eq!(read, value);
        assert_eq!(read.datomize(vec![]).protosize().compact(), text);
    }
}

/// Observe.Metaflows subscribes to the metaflows: as Metaflows lists them
/// on open, then each one that changes as it now stands, and the Address
/// of each no longer listed. Datom to rkyv to datom returns the same text.
#[test]
fn a_metaflow_subscription_opens_on_the_list_then_names_each_change_round_trip() {
    let query = Query::Observe(signal_flow::ObserveSelection::Metaflows);
    let query_text = query.datomize(vec![]).protosize().compact();
    assert_eq!(query_text, "Observe.Metaflows");
    assert_eq!(
        Potential::<Query>::from(query_text)
            .actualize(&mut budget())
            .unwrap(),
        query
    );
    for text in [
        "MetaflowsObserved.[ { { Psyche flow Fable } Awake.f1c841 [ past-1 ] [] } { { Mind topics Astra } Asleep [] [] } ]",
        "MetaflowsObserved.[]",
        "MetaflowChanged.{ { Psyche flow Fable } Asleep [ f1c841 past-1 ] [] }",
        "MetaflowGone.{ Mind topics Astra }",
    ] {
        let value = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
        let read = rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap();
        assert_eq!(read, value);
        assert_eq!(read.datomize(vec![]).protosize().compact(), text);
    }
}

/// Restart names the flow alone; each answer names the flow again, and a
/// resume refused carries its fixed category. The concrete text is the
/// specification.
#[test]
fn a_restart_and_each_of_its_answers_round_trip_as_text() {
    let query = Potential::<Query>::from("Restart.c1a0de")
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(
        query,
        Query::Restart(signal_flow::FlowId("c1a0de".to_string()))
    );
    assert_eq!(
        query.datomize(vec![]).protosize().compact(),
        "Restart.c1a0de"
    );
    for (text, expected) in [
        (
            "Restarted.c1a0de",
            Response::Restarted(signal_flow::FlowId("c1a0de".to_string())),
        ),
        (
            "RestartRefused.Busy.c1a0de",
            Response::RestartRefused(signal_flow::RestartRefusal::Busy(signal_flow::FlowId(
                "c1a0de".to_string(),
            ))),
        ),
        (
            "RestartRefused.ResumeRefused.{ c1a0de BindMismatch }",
            Response::RestartRefused(signal_flow::RestartRefusal::ResumeRefused(
                signal_flow::ResumeRefused_Data {
                    flow_id: signal_flow::FlowId("c1a0de".to_string()),
                    string: "BindMismatch".into(),
                },
            )),
        ),
        (
            "StopRejected.SpawnUnproven",
            Response::StopRejected(signal_flow::StopRejection::SpawnUnproven),
        ),
    ] {
        let actual = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(actual, expected);
        assert_eq!(actual.datomize(vec![]).protosize().compact(), text);
    }
}

/// Each operation that ends, empties, forces or reads a request or a queue
/// has a concrete datom, asked and answered; the text is the specification.
#[test]
fn every_queue_and_request_operation_round_trips_as_text() {
    for text in [
        "Cancel.k7ph",
        "Silence.k7ph",
        "Clear.{ Mind core Sol }",
        "ClearAll",
        "Push.{ { Mind core Sol } HardAbrupt }",
        "Queues",
        "Hot",
        "Remind.{ { Mind core Sol } 30 «Check the gate.» }",
        "Answer.k7ph",
        "Send.{ { Psyche core Fable } Soft.Living.{ «the drain» «Fix it today.» } }",
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        assert_eq!(query.datomize(Vec::new()).protosize().compact(), text);
    }
    for (text, expected) in [
        (
            "Cancelled.k7ph",
            Response::Cancelled(signal_flow::RequestId("k7ph".to_string())),
        ),
        (
            "CancelRejected.Closed",
            Response::CancelRejected(signal_flow::RequestRejection::Closed),
        ),
        (
            "SilenceRejected.NotTask",
            Response::SilenceRejected(signal_flow::RequestRejection::NotTask),
        ),
        (
            "Cleared.[ k7ph m2qa ]",
            Response::Cleared(vec![
                signal_flow::RequestId("k7ph".to_string()),
                signal_flow::RequestId("m2qa".to_string()),
            ]),
        ),
        (
            "Pushed.[ p5lg ]",
            Response::Pushed(vec![signal_flow::RequestId("p5lg".to_string())]),
        ),
        (
            "Queues.[ { { Mind core Sol } [ { Sender.{ Psyche core Fable } MiddleAbrupt.Order.{ k7ph «Review the drain fix.» } } { Unbound Soft.Living.{ m2qa «the drain» «Fix it today.» } } ] } ]",
            Response::Queues(vec![signal_flow::Queue {
                address: signal_flow::Address {
                    aspect: Aspect::Mind,
                    topic: signal_flow::Topic("core".to_string()),
                    model: signal_flow::Model::Sol,
                },
                visible_native_input_vector: vec![
                    signal_flow::VisibleNativeInput {
                        visible_sender: signal_flow::VisibleSender::Sender(signal_flow::Address {
                            aspect: Aspect::Psyche,
                            topic: signal_flow::Topic("core".to_string()),
                            model: signal_flow::Model::Fable,
                        }),
                        delivered_message: signal_flow::DeliveredMessage::MiddleAbrupt(
                            signal_flow::DeliveredRequest::Order(signal_flow::Order_Data {
                                request_id: signal_flow::RequestId("k7ph".to_string()),
                                string: "Review the drain fix.".to_string(),
                            }),
                        ),
                    },
                    signal_flow::VisibleNativeInput {
                        visible_sender: signal_flow::VisibleSender::Unbound,
                        delivered_message: signal_flow::DeliveredMessage::Soft(
                            signal_flow::DeliveredRequest::Living(
                                signal_flow::DeliveredRequest_Living_Data {
                                    request_id: signal_flow::RequestId("m2qa".to_string()),
                                    context: signal_flow::Context("the drain".to_string()),
                                    verbatim: signal_flow::Verbatim("Fix it today.".to_string()),
                                },
                            ),
                        ),
                    },
                ],
            }]),
        ),
        (
            "Reminded.k7ph",
            Response::Reminded(signal_flow::RequestId("k7ph".to_string())),
        ),
        (
            "Answered.{ Sender.{ Mind core Sol } Soft.Result.{ k7ph «Reviewed; green.» } }",
            Response::Answered(signal_flow::VisibleNativeInput {
                visible_sender: signal_flow::VisibleSender::Sender(signal_flow::Address {
                    aspect: Aspect::Mind,
                    topic: signal_flow::Topic("core".to_string()),
                    model: signal_flow::Model::Sol,
                }),
                delivered_message: signal_flow::DeliveredMessage::Soft(
                    signal_flow::DeliveredRequest::Result(
                        signal_flow::DeliveredRequest_Result_Data {
                            request_id: signal_flow::RequestId("k7ph".to_string()),
                            string: "Reviewed; green.".to_string(),
                        },
                    ),
                ),
            }),
        ),
        (
            "AnswerRejected.Open",
            Response::AnswerRejected(signal_flow::AnswerRejection::Open),
        ),
        (
            "RestartQueued.74e4d1",
            Response::RestartQueued(signal_flow::FlowId("74e4d1".to_string())),
        ),
        (
            "RestartRefused.Asleep.74e4d1",
            Response::RestartRefused(signal_flow::RestartRefusal::Asleep(signal_flow::FlowId(
                "74e4d1".to_string(),
            ))),
        ),
        (
            "Hot.[ { { Mind core Sol } Some.3 } { { Psyche core Fable } None } ]",
            Response::Hot(vec![
                signal_flow::Warmth {
                    address: signal_flow::Address {
                        aspect: Aspect::Mind,
                        topic: signal_flow::Topic("core".to_string()),
                        model: signal_flow::Model::Sol,
                    },
                    minutes_option: Some(flow_ethos::Minutes(3)),
                },
                signal_flow::Warmth {
                    address: signal_flow::Address {
                        aspect: Aspect::Psyche,
                        topic: signal_flow::Topic("core".to_string()),
                        model: signal_flow::Model::Fable,
                    },
                    minutes_option: None,
                },
            ]),
        ),
    ] {
        let actual = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        assert_eq!(actual, expected);
        assert_eq!(actual.datomize(vec![]).protosize().compact(), text);
    }
}

/// A guillemet string is read by balance: an inner « » pair is content, and
/// an unbalanced guillemet inside one is escaped. Each reads and prints back
/// byte for byte, in a Send as a flow writes one.
#[test]
fn a_send_holding_nested_and_escaped_guillemets_round_trips_as_text() {
    for (text, order) in [
        (
            "Send.{ { Mind core Sol } Soft.Order.«Title «The Belief Nexus made small»» }",
            "Title «The Belief Nexus made small»",
        ),
        (
            "Send.{ { Mind core Sol } Soft.Order.«an unbalanced \\» inside» }",
            "an unbalanced » inside",
        ),
    ] {
        let query = Potential::<Query>::from(text)
            .actualize(&mut budget())
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        let Query::Send(send) = &query else {
            panic!("a Send: {query:?}")
        };
        assert_eq!(
            send.sendable_message,
            signal_flow::SendableMessage::Soft(signal_flow::SendableRequest::Order(order.into()))
        );
        assert_eq!(query.datomize(vec![]).protosize().compact(), text);
    }
}

/// A composer's rejection carries the composer's own text, whole.
#[test]
fn a_composer_rejection_carries_its_text() {
    let text = "StartRejected.ContextRefused.ComposerRejected.«missing module knowledge:absent»";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(
        response,
        Response::StartRejected(signal_flow::StartRejection::ContextRefused(
            signal_flow::ContextRefusal::ComposerRejected("missing module knowledge:absent".into())
        ))
    );
    assert_eq!(response.datomize(vec![]).protosize().compact(), text);
}

/// A composition refused for a predecessor's handover carries the
/// predecessor and the fault, typed.
#[test]
fn a_handover_refusal_carries_its_predecessor_and_fault() {
    for (text, fault) in [
        (
            "StartRejected.CompositionRefused.HandoverUnreadable.{ abc123 Unreadable.«flows/abc123/handover.md: Permission denied (os error 13)» }",
            signal_flow::HandoverFault::Unreadable(
                "flows/abc123/handover.md: Permission denied (os error 13)".into(),
            ),
        ),
        (
            "StartRejected.CompositionRefused.HandoverUnreadable.{ abc123 NotUtf8 }",
            signal_flow::HandoverFault::NotUtf8,
        ),
        (
            "StartRejected.CompositionRefused.HandoverUnreadable.{ abc123 InvalidPredecessor }",
            signal_flow::HandoverFault::InvalidPredecessor,
        ),
        (
            "StartRejected.CompositionRefused.HandoverUnreadable.{ abc123 RootUnresolved }",
            signal_flow::HandoverFault::RootUnresolved,
        ),
    ] {
        let response = Potential::<Response>::from(text)
            .actualize(&mut budget())
            .unwrap();
        assert_eq!(
            response,
            Response::StartRejected(signal_flow::StartRejection::CompositionRefused(
                signal_flow::CompositionRefusal::HandoverUnreadable(
                    signal_flow::HandoverUnreadable_Data {
                        flow_id: signal_flow::FlowId("abc123".to_string()),
                        handover_fault: fault,
                    }
                )
            ))
        );
        assert_eq!(response.datomize(vec![]).protosize().compact(), text);
    }
}

/// A replacement whose successor is confirmed answers ReapPending as soon as
/// its reap is owed; a fresh start whose handover could not move answers
/// HandoverPending; a pending handover a later launch took over reads
/// HandoverSuperseded. Each is a reply and a launch outcome.
#[test]
fn pending_reaps_and_handovers_round_trip_as_replies_and_outcomes() {
    let launched = "{ 908786 session-2 Available.{ default claude-a1 w1:p4 term-4 } Flow.{ fac697 session-1 } }";
    let standings = [
        "Owed",
        "Unrecorded",
        "PredecessorMissing",
        "PredecessorUnknown",
        "HerdrUnavailable",
        "CloseRefused",
        "HandoverRefused.PersistenceRefused",
        "HandoverRefused.Owner.Live.751881",
        "HandoverRefused.Owner.Missing.751881",
        "HandoverRefused.Owner.Unknown.751881",
        "SettleUnrecorded",
    ];
    let refusals = [
        "PersistenceRefused",
        "Owner.Live.751881",
        "Owner.Missing.751881",
        "Owner.Unknown.751881",
    ];
    let mut texts = Vec::new();
    for standing in standings {
        texts.push(format!(
            "ReapPending.{{ {{ Psyche core Fable }} fac697 {launched} {standing} }}"
        ));
    }
    for refusal in refusals {
        texts.push(format!(
            "HandoverPending.{{ {{ Psyche core Fable }} 751881 {launched} {refusal} }}"
        ));
    }
    texts.push(format!(
        "HandoverSuperseded.{{ {{ Psyche core Fable }} 751881 {launched} 4e1d2a }}"
    ));
    for text in &texts {
        let response = Potential::<Response>::from(text.as_str())
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&response).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Response, rkyv::rancor::Error>(&archive).unwrap(),
            response
        );
        assert_eq!(&response.datomize(vec![]).protosize().compact(), text);
        let outcome = Potential::<signal_flow::LaunchStatusOutcome>::from(text.as_str())
            .actualize(&mut budget())
            .unwrap();
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&outcome).unwrap();
        assert_eq!(
            rkyv::from_bytes::<signal_flow::LaunchStatusOutcome, rkyv::rancor::Error>(&archive)
                .unwrap(),
            outcome
        );
        assert_eq!(&outcome.datomize(vec![]).protosize().compact(), text);
    }
}

#[test]
fn a_pending_reap_names_its_address_predecessor_successor_and_standing() {
    let text = "ReapPending.{ { Psyche core Fable } fac697 { 908786 session-2 Unavailable Unbound } HandoverRefused.Owner.Live.751881 }";
    let response = Potential::<Response>::from(text)
        .actualize(&mut budget())
        .unwrap();
    assert_eq!(
        response,
        Response::ReapPending(signal_flow::ReapPending {
            address: signal_flow::Address {
                aspect: Aspect::Psyche,
                topic: signal_flow::Topic("core".to_string()),
                model: signal_flow::Model::Fable,
            },
            predecessor: signal_flow::Predecessor(signal_flow::FlowId("fac697".to_string())),
            launched: signal_flow::Launched {
                flow_id: signal_flow::FlowId("908786".to_string()),
                session_id: signal_flow::SessionId("session-2".to_string()),
                herdr_route_selection: signal_flow::HerdrRouteSelection::Unavailable,
                origin: signal_flow::Origin::Unbound,
            },
            reap_standing: signal_flow::ReapStanding::HandoverRefused(
                signal_flow::HandoverCloseRefusal::Owner(signal_flow::HandoverOwnerRefusal::Live(
                    signal_flow::FlowId("751881".to_string())
                ))
            ),
        })
    );
}

/// Every reap follows a confirmed successor, so a refused reap is no reason
/// a replacement did not take place: ReapRefused is no datom of 61.
#[test]
fn a_refused_reap_is_no_replace_rejection() {
    for text in [
        "ReplaceRejected.ReapRefused.CloseRefused",
        "RefreshRejected.SuccessorRefused.ReapRefused.CloseRefused",
    ] {
        assert!(
            Potential::<Response>::from(text)
                .actualize(&mut budget())
                .is_err(),
            "{text}"
        );
    }
}
