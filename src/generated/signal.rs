#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type FlowId = String;
#[rustfmt::skip]
pub type PredecessorFlowId = String;
#[rustfmt::skip]
pub type ReplacementFlowId = String;
#[rustfmt::skip]
pub type SessionId = String;
#[rustfmt::skip]
pub type TurnId = String;
#[rustfmt::skip]
pub type FlowType = String;
#[rustfmt::skip]
pub type Generation = i64;
#[rustfmt::skip]
pub type EndpointPath = String;
#[rustfmt::skip]
pub type HerdrSessionName = String;
#[rustfmt::skip]
pub type HerdrAgentName = String;
#[rustfmt::skip]
pub type HerdrWorkspaceId = String;
#[rustfmt::skip]
pub type HerdrPaneId = String;
#[rustfmt::skip]
pub type HerdrTerminalId = String;
#[rustfmt::skip]
pub type LaunchRequestId = String;
#[rustfmt::skip]
pub type SourcePath = String;
#[rustfmt::skip]
pub type SourceSha256 = String;
#[rustfmt::skip]
pub type SkillName = String;
#[rustfmt::skip]
pub type ModelName = String;
#[rustfmt::skip]
pub type Effort = String;
#[rustfmt::skip]
pub type RememberingDepth = i64;
#[rustfmt::skip]
pub type InstructionPrompt = String;
#[rustfmt::skip]
pub type FirstPromptBody = String;
#[rustfmt::skip]
pub type FirstPromptText = String;
#[rustfmt::skip]
pub type PromptSha256 = String;
#[rustfmt::skip]
pub type NativeSessionId = String;
#[rustfmt::skip]
pub type NativeTurnId = String;
#[rustfmt::skip]
pub type ReceiptSha256 = String;
#[rustfmt::skip]
pub type TranscriptDevice = String;
#[rustfmt::skip]
pub type TranscriptInode = String;
#[rustfmt::skip]
pub type TranscriptByteOffset = i64;
#[rustfmt::skip]
pub type TranscriptPrefixSha256 = String;
#[rustfmt::skip]
pub type TranscriptRootDevice = String;
#[rustfmt::skip]
pub type TranscriptRootInode = String;
#[rustfmt::skip]
pub type NativeSkillPath = String;
#[rustfmt::skip]
pub type NativeSkillSha256 = String;
#[rustfmt::skip]
pub type ProcessId = i64;
#[rustfmt::skip]
pub type ProcessUserId = i64;
#[rustfmt::skip]
pub type ProcessStartToken = String;
#[rustfmt::skip]
pub type TranscriptItemId = String;
#[rustfmt::skip]
pub type TranscriptTitle = String;
#[rustfmt::skip]
pub type TranscriptTimestampSeconds = i64;
#[rustfmt::skip]
pub type TranscriptRecordSha256 = String;
#[rustfmt::skip]
pub type HandoverByteOffset = i64;
#[rustfmt::skip]
pub type HandoverByteLength = i64;
#[rustfmt::skip]
pub type HandoverSelectionSha256 = String;
#[rustfmt::skip]
pub type ArchivePath = String;
#[rustfmt::skip]
pub type ArchiveSha256 = String;
#[rustfmt::skip]
pub type ArchiveIndexId = String;
#[rustfmt::skip]
pub type MaximumHandoverAgeSeconds = i64;
#[rustfmt::skip]
pub type DeliveryIdempotencyKey = String;
#[rustfmt::skip]
pub type HopLimit = i64;
#[rustfmt::skip]
pub type HopIndex = i64;
#[rustfmt::skip]
pub type RequestedFlowId = FlowId;
#[rustfmt::skip]
pub type RequestedFlowAspect = FlowAspect;
#[rustfmt::skip]
pub type CandidateFlowAspect = FlowAspect;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FlowAspect {
    Psyche,
    Mind,
    Field,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PowerLevel {
    High,
    Medium,
    Low,
    UltraLow,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LaunchSource {
    pub source_path: SourcePath,
    pub source_sha256: SourceSha256,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RememberedFlow {
    pub flow_id: FlowId,
    pub remembering_depth: RememberingDepth,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LaunchProfile {
    pub launch_request_id: LaunchRequestId,
    pub launch_source_vector: std::vec::Vec<LaunchSource>,
    pub skill_name_vector: std::vec::Vec<SkillName>,
    pub flow_aspect: FlowAspect,
    pub power_level: PowerLevel,
    pub harness_kind: HarnessKind,
    pub model_name: ModelName,
    pub effort: Effort,
    pub flow_id_option: Option<FlowId>,
    pub remembered_flow_vector: std::vec::Vec<RememberedFlow>,
    pub herdr_session_name: HerdrSessionName,
    pub instruction_prompt: InstructionPrompt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TargetReceiptRequest {
    pub launch_request_id: LaunchRequestId,
    pub prompt_sha256: PromptSha256,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FirstPromptPayload {
    pub first_prompt_body: FirstPromptBody,
    pub prompt_sha256: PromptSha256,
    pub first_prompt_text: FirstPromptText,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComposedLaunch {
    pub launch_profile: LaunchProfile,
    pub first_prompt_payload: FirstPromptPayload,
    pub target_receipt_request: TargetReceiptRequest,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HerdrPaneBinding {
    pub launch_request_id: LaunchRequestId,
    pub herdr_session_name: HerdrSessionName,
    pub herdr_agent_name: HerdrAgentName,
    pub herdr_workspace_id: HerdrWorkspaceId,
    pub herdr_pane_id: HerdrPaneId,
    pub herdr_terminal_id: HerdrTerminalId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeLaunchIntent {
    pub launch_request_id: LaunchRequestId,
    pub prompt_sha256: PromptSha256,
    pub harness_kind: HarnessKind,
    pub model_name: ModelName,
    pub effort: Effort,
    pub skill_name_vector: std::vec::Vec<SkillName>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeLaunchBinding {
    pub launch_request_id: LaunchRequestId,
    pub flow_id: FlowId,
    pub native_session_id: NativeSessionId,
    pub harness_kind: HarnessKind,
    pub herdr_pane_binding: HerdrPaneBinding,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RegistrationAcknowledgement {
    pub launch_request_id: LaunchRequestId,
    pub flow_id: FlowId,
    pub native_session_id: NativeSessionId,
    pub herdr_pane_binding: HerdrPaneBinding,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeTranscriptCursor {
    pub native_session_id: NativeSessionId,
    pub harness_kind: HarnessKind,
    pub transcript_device: TranscriptDevice,
    pub transcript_inode: TranscriptInode,
    pub transcript_byte_offset: TranscriptByteOffset,
    pub transcript_prefix_sha256: TranscriptPrefixSha256,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeTranscriptAbsence {
    pub native_session_id: NativeSessionId,
    pub harness_kind: HarnessKind,
    pub transcript_root_device: TranscriptRootDevice,
    pub transcript_root_inode: TranscriptRootInode,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum NativeTranscriptBoundary {
    Existing(NativeTranscriptCursor),
    Absent(NativeTranscriptAbsence),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeSkillSelection {
    pub skill_name: SkillName,
    pub native_skill_path: NativeSkillPath,
    pub native_skill_sha256: NativeSkillSha256,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptDeliveryIntent {
    pub launch_request_id: LaunchRequestId,
    pub prompt_sha256: PromptSha256,
    pub flow_id: FlowId,
    pub native_session_id: NativeSessionId,
    pub harness_kind: HarnessKind,
    pub model_name: ModelName,
    pub effort: Effort,
    pub native_skill_selection_vector: std::vec::Vec<NativeSkillSelection>,
    pub herdr_pane_binding: HerdrPaneBinding,
    pub native_transcript_boundary: NativeTranscriptBoundary,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeTargetReceipt {
    pub launch_request_id: LaunchRequestId,
    pub prompt_sha256: PromptSha256,
    pub flow_id: FlowId,
    pub native_session_id: NativeSessionId,
    pub native_turn_id: NativeTurnId,
    pub receipt_sha256: ReceiptSha256,
    pub model_name: ModelName,
    pub effort: Effort,
    pub native_skill_selection_vector: std::vec::Vec<NativeSkillSelection>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PromptDeliveryResult {
    Observed(NativeTargetReceipt),
    Ambiguous(PromptDeliveryIntent),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LaunchAttemptPhase {
    Reserved,
    NativeLaunchIntentRecorded,
    NativeBound,
    RegistrationAcknowledged,
    PromptIntentRecorded,
    PromptObserved,
    PromptAmbiguous,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LaunchAttempt {
    pub launch_request_id: LaunchRequestId,
    pub launch_profile: LaunchProfile,
    pub prompt_sha256: PromptSha256,
    pub origin_clue: OriginClue,
    pub launch_attempt_phase: LaunchAttemptPhase,
    pub native_launch_intent_option: Option<NativeLaunchIntent>,
    pub native_launch_binding_option: Option<NativeLaunchBinding>,
    pub registration_acknowledgement_option: Option<RegistrationAcknowledgement>,
    pub prompt_delivery_intent_option: Option<PromptDeliveryIntent>,
    pub prompt_delivery_result_option: Option<PromptDeliveryResult>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LaunchAttemptReservation {
    Reserved(LaunchAttempt),
    Existing(LaunchAttempt),
    Conflict,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OriginClue {
    pub flow_id: FlowId,
    pub session_id: SessionId,
    pub turn_id: TurnId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StartRequest {
    pub launch_profile: LaunchProfile,
    pub origin_clue: OriginClue,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ProcessIdentity {
    pub process_id: ProcessId,
    pub process_user_id: ProcessUserId,
    pub process_start_token: ProcessStartToken,
}
#[rustfmt::skip]
pub type CallerFlowHint = FlowId;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum CallerRelationship {
    Harness,
    Descendant,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CallerProof {
    pub flow_id: FlowId,
    pub process_identity: ProcessIdentity,
    pub caller_relationship: CallerRelationship,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefreshPolicy {
    pub maximum_handover_age_seconds: MaximumHandoverAgeSeconds,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TranscriptRole {
    Assistant,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HandoverByteSelection {
    pub handover_byte_offset: HandoverByteOffset,
    pub handover_byte_length: HandoverByteLength,
    pub handover_selection_sha256: HandoverSelectionSha256,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HandoverSelection {
    WholeMessage,
    SelectedBytes(HandoverByteSelection),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranscriptHandoverReference {
    pub harness_kind: HarnessKind,
    pub native_session_id: NativeSessionId,
    pub native_turn_id: NativeTurnId,
    pub transcript_item_id: TranscriptItemId,
    pub transcript_role: TranscriptRole,
    pub transcript_title: TranscriptTitle,
    pub transcript_timestamp_seconds: TranscriptTimestampSeconds,
    pub transcript_record_sha256: TranscriptRecordSha256,
    pub handover_selection: HandoverSelection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReplacementIdempotencyKey {
    pub flow_id: FlowId,
    pub transcript_record_sha256: TranscriptRecordSha256,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefreshRequest {
    pub flow_id: FlowId,
    pub caller_flow_hint: CallerFlowHint,
    pub transcript_handover_reference: TranscriptHandoverReference,
    pub launch_profile: LaunchProfile,
    pub origin_clue: OriginClue,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum InteractiveReadiness {
    Ready,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReplacementReadyProof {
    pub herdr_pane_binding: HerdrPaneBinding,
    pub process_identity: ProcessIdentity,
    pub interactive_readiness: InteractiveReadiness,
    pub native_target_receipt: NativeTargetReceipt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RegistrationAbsence {
    Unregistered,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ProcessLiveness {
    Dead,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OldRouteRemovalProof {
    pub flow_id: FlowId,
    pub herdr_route: HerdrRoute,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OldInactiveProof {
    pub flow_id: FlowId,
    pub process_identity: ProcessIdentity,
    pub registration_absence: RegistrationAbsence,
    pub process_liveness: ProcessLiveness,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ArchiveReceipt {
    pub archive_path: ArchivePath,
    pub archive_sha256: ArchiveSha256,
    pub archive_index_id: ArchiveIndexId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CutoverReceipt {
    pub predecessor_flow_id: PredecessorFlowId,
    pub replacement_flow_id: ReplacementFlowId,
    pub replacement_ready_proof: ReplacementReadyProof,
    pub old_route_removal_proof: OldRouteRemovalProof,
    pub old_inactive_proof: OldInactiveProof,
    pub archive_receipt: ArchiveReceipt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RefreshHoldReason {
    HandoverPolicyUnavailable,
    StaleHandover,
    CallerProofUnavailable,
    RouteLockUnavailable,
    ReplacementNotReady,
    OldRouteRemovalPending,
    OldStillRegistered,
    OldProcessStillAlive,
    ArchivePending,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RefreshAttemptPhase {
    RouteLocked,
    ReplacementLaunching,
    ReplacementRegisteredUnconfirmed,
    ReplacementReady,
    CutoverInProgress,
    Archived,
    Complete,
    Held(RefreshHoldReason),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefreshAttempt {
    pub replacement_idempotency_key: ReplacementIdempotencyKey,
    pub refresh_request: RefreshRequest,
    pub refresh_policy: RefreshPolicy,
    pub refresh_attempt_phase: RefreshAttemptPhase,
    pub flow_id_option: Option<FlowId>,
    pub caller_proof_option: Option<CallerProof>,
    pub replacement_ready_proof_option: Option<ReplacementReadyProof>,
    pub cutover_receipt_option: Option<CutoverReceipt>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefreshCompletion {
    pub replacement_idempotency_key: ReplacementIdempotencyKey,
    pub predecessor_flow_id: PredecessorFlowId,
    pub replacement_flow_id: ReplacementFlowId,
    pub cutover_receipt: CutoverReceipt,
}
#[rustfmt::skip]
pub type RecipientResolutionRequest = FlowId;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryResolutionRequest {
    pub requested_flow_id: RequestedFlowId,
    pub delivery_idempotency_key: DeliveryIdempotencyKey,
    pub hop_limit: HopLimit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HarnessKind {
    Codex,
    Claude,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RouteReadiness {
    Ready,
    Parked,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Available_Data {
    pub endpoint_path: EndpointPath,
    pub route_readiness: RouteReadiness,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum EndpointSelection {
    Available(Available_Data),
    Unavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HerdrRoute {
    pub herdr_session_name: HerdrSessionName,
    pub herdr_agent_name: HerdrAgentName,
    pub herdr_pane_id: HerdrPaneId,
    pub herdr_terminal_id: HerdrTerminalId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HerdrRouteSelection {
    Available(HerdrRoute),
    Unavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FlowLifecycle {
    RegisteredUnconfirmed,
    Ready,
    Retiring,
    Archived,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryHoldReason {
    NativeReceiptUnconfirmed,
    ReplacementNotReady,
    RouteTransferInProgress,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FlowNode {
    pub flow_id: FlowId,
    pub session_id: SessionId,
    pub harness_kind: HarnessKind,
    pub endpoint_selection: EndpointSelection,
    pub herdr_route_selection: HerdrRouteSelection,
    pub origin_clue: OriginClue,
    pub flow_lifecycle: FlowLifecycle,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RecipientHold {
    pub flow_id: FlowId,
    pub flow_lifecycle: FlowLifecycle,
    pub delivery_hold_reason: DeliveryHoldReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RecipientReroute {
    pub flow_id: FlowId,
    pub replacement_flow_id: ReplacementFlowId,
    pub flow_lifecycle: FlowLifecycle,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RecipientDisposition {
    Deliverable(FlowNode),
    Held(RecipientHold),
    Reroute(RecipientReroute),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReadyRecipient {
    pub flow_node: FlowNode,
    pub flow_aspect: FlowAspect,
    pub power_level: PowerLevel,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CrucialStart {
    pub launch_request_id: LaunchRequestId,
    pub flow_aspect: FlowAspect,
    pub power_level: PowerLevel,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PowerSearchPhase {
    Requested,
    Higher,
    Lower,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StartRefused_Data {
    pub launch_request_id: LaunchRequestId,
    pub start_rejection: StartRejection,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryTraceObservation {
    Ready,
    Held(RecipientHold),
    Rerouted(RecipientReroute),
    CrossAspectExcluded,
    StartPending(CrucialStart),
    StartRefused(StartRefused_Data),
    CycleDetected,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryTraceEntry {
    pub hop_index: HopIndex,
    pub flow_id: FlowId,
    pub flow_aspect: FlowAspect,
    pub power_level: PowerLevel,
    pub power_search_phase: PowerSearchPhase,
    pub delivery_trace_observation: DeliveryTraceObservation,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryResolutionHold {
    Lifecycle(RecipientHold),
    StartPending(CrucialStart),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CrossAspectCandidate {
    pub flow_id: FlowId,
    pub requested_flow_aspect: RequestedFlowAspect,
    pub candidate_flow_aspect: CandidateFlowAspect,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NoEligibleSameAspect {
    pub requested_flow_id: RequestedFlowId,
    pub flow_aspect: FlowAspect,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HopExhausted {
    pub hop_limit: HopLimit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StartRefusal {
    pub launch_request_id: LaunchRequestId,
    pub start_rejection: StartRejection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryResolutionRefusal {
    CrossAspectCandidate(CrossAspectCandidate),
    NoEligibleSameAspect(NoEligibleSameAspect),
    HopExhausted(HopExhausted),
    StartRefused(StartRefusal),
    CycleDetected(FlowId),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryResolutionDisposition {
    Selected(ReadyRecipient),
    Held(DeliveryResolutionHold),
    Refused(DeliveryResolutionRefusal),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryResolution {
    pub delivery_idempotency_key: DeliveryIdempotencyKey,
    pub requested_flow_id: RequestedFlowId,
    pub delivery_trace_entry_vector: std::vec::Vec<DeliveryTraceEntry>,
    pub delivery_resolution_disposition: DeliveryResolutionDisposition,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Started {
    pub flow_id: FlowId,
    pub session_id: SessionId,
    pub origin_clue: OriginClue,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum StartRejection {
    CompositionRefused,
    LaunchRequestConflict,
    LaunchPersistenceRefused,
    NativeLaunchRefused,
    BindingRefused,
    RegistrationRefused,
    IntentPersistenceRefused,
    OriginUnavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RefreshRejection {
    RefreshImplementationUnavailable,
    UnknownFlow,
    ProvenanceMismatch,
    HandoverPolicyUnavailable,
    StaleHandover,
    HandoverRoleMismatch,
    HandoverTitleMismatch,
    HandoverBeforeCallerReceipt,
    HandoverReferenceInvalid,
    CallerProofUnavailable,
    CallerProofMismatch,
    IdempotencyConflict,
    RefreshAlreadyInProgress,
    RefreshPersistenceRefused,
    RouteTransferRefused,
    RetirementRefused,
    ArchiveRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RecipientResolutionRejection {
    UnknownFlow,
    FlowUnavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Start(StartRequest),
    Refresh(RefreshRequest),
    ResolveRecipient(RecipientResolutionRequest),
    ResolveDelivery(DeliveryResolutionRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Started(Started),
    LaunchPending(LaunchAttempt),
    StartAmbiguous(PromptDeliveryIntent),
    RefreshProgress(RefreshAttempt),
    Refreshed(RefreshCompletion),
    RecipientDispositioned(RecipientDisposition),
    DeliveryResolved(DeliveryResolution),
    StartRejected(StartRejection),
    RefreshRejected(RefreshRejection),
    RecipientResolutionRejected(RecipientResolutionRejection),
}
