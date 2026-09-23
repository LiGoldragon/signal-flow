#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type FlowId = String;
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
pub struct NativeTranscriptBoundary {
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
pub struct PromptDeliveryIntent {
    pub launch_request_id: LaunchRequestId,
    pub prompt_sha256: PromptSha256,
    pub flow_id: FlowId,
    pub native_session_id: NativeSessionId,
    pub harness_kind: HarnessKind,
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
pub struct RestartRequest {
    pub flow_id: FlowId,
    pub origin_clue: OriginClue,
}
#[rustfmt::skip]
pub type RecipientResolutionRequest = FlowId;
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
    Pending,
    Active,
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
pub struct Started {
    pub flow_id: FlowId,
    pub session_id: SessionId,
    pub origin_clue: OriginClue,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Restarted {
    pub flow_id: FlowId,
    pub session_id: SessionId,
    pub generation: Generation,
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
pub enum RestartRejection {
    ProvenanceMismatch,
    UnknownFlow,
    ResumeRefused,
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
    Restart(RestartRequest),
    ResolveRecipient(RecipientResolutionRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Started(Started),
    LaunchPending(LaunchAttempt),
    StartAmbiguous(PromptDeliveryIntent),
    Restarted(Restarted),
    RecipientResolved(FlowNode),
    StartRejected(StartRejection),
    RestartRejected(RestartRejection),
    RecipientResolutionRejected(RecipientResolutionRejection),
}
