#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Tokens(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Timestamp(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FlowType(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EndpointPath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HerdrSessionName(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HerdrAgentName(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HerdrWorkspaceId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HerdrPaneId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HerdrTerminalId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LaunchRequestId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ModuleIdentity(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemModules(pub std::vec::Vec<ModuleIdentity>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptModules(pub std::vec::Vec<ModuleIdentity>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Effort(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RememberingDepth(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Remembering {
    Predecessors(RememberingDepth),
    Custom(std::vec::Vec<flow_ethos::FlowId>),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LaunchRequest {
    pub address: flow_ethos::Address,
    pub remembering: Remembering,
    pub instruction_prompt: InstructionPrompt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemPromptText(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemPromptSha256(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InstructionPrompt(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Command(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FirstPromptBody(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptPartText(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptSha256(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeSessionId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeTurnId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReceiptSha256(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranscriptDevice(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranscriptInode(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranscriptByteOffset(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranscriptPrefixSha256(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranscriptRootDevice(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranscriptRootInode(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PartsSubmitted(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StopsReported(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LeaveOut {
    ClaudeAiConnectors,
    ConfiguredMcpServers,
    AgentTool,
    ExplorePlanAgents,
    FileTools,
    WorkflowTool,
    ScheduleWakeupTool,
    ReportFindingsTool,
    ListAgentsTool,
    CronTools,
    WebTools,
    DeferredTools,
    SkillTool,
    BundledSkills,
    ClaudeMd,
    GitInstructions,
    AttributionReminder,
    UserSettings,
    ToolSearch,
    BashTool,
    CodexSkillListing,
    CodexAgentsMd,
    CodexCollaborationMode,
    CodexEnvironmentContext,
    CodexPermissions,
    CodexDeveloperInstructions,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LeaveOuts(pub std::vec::Vec<LeaveOut>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FlowKind {
    Regular,
    Simplified,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ToolSet {
    Default,
    Explicit(std::vec::Vec<String>),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ArtifactToolsAllowed(pub bool);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LaunchProfile {
    pub launch_request_id: LaunchRequestId,
    pub system_modules: SystemModules,
    pub prompt_modules: PromptModules,
    pub remembering: Remembering,
    pub address: flow_ethos::Address,
    pub model_name: flow_ethos::ModelName,
    pub effort: Effort,
    pub flow_id_option: std::option::Option<flow_ethos::FlowId>,
    pub herdr_session_name: HerdrSessionName,
    pub leave_outs_option: std::option::Option<LeaveOuts>,
    pub flow_kind: FlowKind,
    pub tool_set: ToolSet,
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
pub struct PromptPart {
    pub prompt_part_text: PromptPartText,
    pub prompt_sha256: PromptSha256,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FirstPromptPayload {
    pub first_prompt_body: FirstPromptBody,
    pub prompt_sha256: PromptSha256,
    pub prompt_part_vector: std::vec::Vec<PromptPart>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemPrompt {
    pub system_prompt_text: SystemPromptText,
    pub system_prompt_sha256: SystemPromptSha256,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComposedLaunch {
    pub launch_profile: LaunchProfile,
    pub harness_kind: flow_ethos::HarnessKind,
    pub system_prompt: SystemPrompt,
    pub first_prompt_payload: FirstPromptPayload,
    pub target_receipt_request: TargetReceiptRequest,
    pub leave_outs: LeaveOuts,
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
    pub system_prompt_sha256: SystemPromptSha256,
    pub harness_kind: flow_ethos::HarnessKind,
    pub model_name: flow_ethos::ModelName,
    pub effort: Effort,
    pub prompt_sha256_vector: std::vec::Vec<PromptSha256>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeLaunchBinding {
    pub launch_request_id: LaunchRequestId,
    pub flow_id: flow_ethos::FlowId,
    pub native_session_id: NativeSessionId,
    pub harness_kind: flow_ethos::HarnessKind,
    pub herdr_pane_binding: HerdrPaneBinding,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RegistrationAcknowledgement {
    pub launch_request_id: LaunchRequestId,
    pub flow_id: flow_ethos::FlowId,
    pub native_session_id: NativeSessionId,
    pub herdr_pane_binding: HerdrPaneBinding,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeTranscriptCursor {
    pub native_session_id: NativeSessionId,
    pub harness_kind: flow_ethos::HarnessKind,
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
    pub harness_kind: flow_ethos::HarnessKind,
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
pub struct PromptDeliveryIntent {
    pub launch_request_id: LaunchRequestId,
    pub prompt_sha256: PromptSha256,
    pub flow_id: flow_ethos::FlowId,
    pub native_session_id: NativeSessionId,
    pub harness_kind: flow_ethos::HarnessKind,
    pub model_name: flow_ethos::ModelName,
    pub effort: Effort,
    pub prompt_sha256_vector: std::vec::Vec<PromptSha256>,
    pub parts_submitted: PartsSubmitted,
    pub stops_reported: StopsReported,
    pub leave_outs: LeaveOuts,
    pub herdr_pane_binding: HerdrPaneBinding,
    pub native_transcript_boundary: NativeTranscriptBoundary,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeTargetReceipt {
    pub launch_request_id: LaunchRequestId,
    pub prompt_sha256: PromptSha256,
    pub flow_id: flow_ethos::FlowId,
    pub native_session_id: NativeSessionId,
    pub native_turn_id: NativeTurnId,
    pub receipt_sha256: ReceiptSha256,
    pub model_name: flow_ethos::ModelName,
    pub effort: Effort,
    pub prompt_sha256_vector: std::vec::Vec<PromptSha256>,
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
    pub origin: Origin,
    pub launch_attempt_phase: LaunchAttemptPhase,
    pub native_launch_intent_option: std::option::Option<NativeLaunchIntent>,
    pub native_launch_binding_option: std::option::Option<NativeLaunchBinding>,
    pub registration_acknowledgement_option: std::option::Option<
        RegistrationAcknowledgement,
    >,
    pub prompt_delivery_intent_option: std::option::Option<PromptDeliveryIntent>,
    pub prompt_delivery_result_option: std::option::Option<PromptDeliveryResult>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LaunchStatusOutcome {
    Started(Launched),
    Replaced(Replaced),
    StartRejected(StartRejection),
    ReplaceRejected(ReplaceRejection),
    ReapPending(ReapPending),
    HandoverPending(HandoverPending),
    HandoverSuperseded(HandoverSuperseded),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LaunchInspection {
    pub launch_attempt: LaunchAttempt,
    pub launch_status_outcome_option: std::option::Option<LaunchStatusOutcome>,
    pub flow_node_option: std::option::Option<FlowNode>,
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
pub struct FlowOrigin {
    pub flow_id: flow_ethos::FlowId,
    pub session_id: SessionId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Origin {
    Flow(FlowOrigin),
    Unbound,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RecipientResolutionRequest(pub flow_ethos::FlowId);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StopRequest(pub flow_ethos::FlowId);
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
#[rustfmt::skip]
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
    Stopped,
    Retired,
    Exited,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FlowNode {
    pub flow_id: flow_ethos::FlowId,
    pub session_id: SessionId,
    pub harness_kind: flow_ethos::HarnessKind,
    pub endpoint_selection: EndpointSelection,
    pub herdr_route_selection: HerdrRouteSelection,
    pub origin: Origin,
    pub flow_lifecycle: FlowLifecycle,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Launched {
    pub flow_id: flow_ethos::FlowId,
    pub session_id: SessionId,
    pub herdr_route_selection: HerdrRouteSelection,
    pub origin: Origin,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Refreshed {
    pub address: flow_ethos::Address,
    pub flow_id: flow_ethos::FlowId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefreshQueued {
    pub address: flow_ethos::Address,
    pub flow_id: flow_ethos::FlowId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BindingRefusal {
    PaneMismatch,
    TerminalMismatch,
    HarnessMismatch,
    NotReady,
    ReservedIdentityMismatch,
    ClaimUnavailable,
    ObservationRefused,
    TransportRefused,
    Unclassified,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LeaveOutRefusal {
    Indispensable(LeaveOut),
    NeedsWorkflowLeftOut(LeaveOut),
    NotForHarness(LeaveOut),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ModelRefusal {
    Unconfigured,
    Differs(flow_ethos::ModelName),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ContextRefusal {
    ComposerUnavailable,
    ComposerRejected(String),
    MetaflowMismatch,
    PayloadMismatch,
    ModuleMissing(std::option::Option<ModuleIdentity>),
    RecordMissing,
    SystemTooLarge,
    ModuleTooLarge,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HerdrErrorCode(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HerdrErrorMessage(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HerdrRefusal {
    pub herdr_error_code_option: std::option::Option<HerdrErrorCode>,
    pub herdr_error_message: HerdrErrorMessage,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SpawnRefusalCause {
    Herdr(HerdrRefusal),
    ControlCharacter,
    CommandTooLong,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum FirstTurn {
    Answered,
    InProgress,
    Unanswered,
    Unread,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ThreadArchive {
    Archived,
    ArchiveRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OpenedThread {
    pub native_session_id: NativeSessionId,
    pub first_turn: FirstTurn,
    pub thread_archive: ThreadArchive,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessSpawnRefusal {
    pub spawn_refusal_cause: SpawnRefusalCause,
    pub opened_thread_option: std::option::Option<OpenedThread>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum NativeTargetRefusal {
    HerdrUnreadable,
    PaneMismatch,
    NoNativeSession,
    IdentityInexact,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RegistrationRefusal {
    RouteUnverified,
    RoleConflict,
    RoleUnrecorded,
    PaneBindingForeign,
    AcknowledgementMismatch,
    PromptNotCanonical,
    NativeTarget(NativeTargetRefusal),
    TranscriptBoundary,
    ThreadNotNamed,
    Unclassified,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum NativeLaunchStage {
    Open,
    CodexThread,
    Spawn,
    Bind,
    Naming,
    Receipt,
    Confirm,
    Unclassified,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum NativeLaunchFailure {
    PaneTargetMismatch,
    NativeSessionMissing,
    NativeIdentityConflict,
    FlowClaimUnavailable,
    HarnessMismatch,
    HerdrUnreadable,
    ForeignBinding,
    InvalidFlowId,
    NotReady,
    TranscriptUnreadable,
    TitleReadbackDiffers,
    CodexNamingRefused,
    PaneLabelRefused,
    AdapterRefused,
    BoundaryMismatch,
    ThreadNotNamed,
    PartDiffers,
    LeaveOutShown,
    ModelOrEffortDiffers,
    SessionUnrecorded,
    Unclassified,
    SendToolUnsupported,
    WorkspaceLabelRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeLaunchRefusal {
    pub native_launch_stage: NativeLaunchStage,
    pub native_launch_failure: NativeLaunchFailure,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HandoverUnreadable_Data {
    pub flow_id: flow_ethos::FlowId,
    pub handover_fault: HandoverFault,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum CompositionRefusal {
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
    HandoverUnreadable(HandoverUnreadable_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HandoverFault {
    InvalidPredecessor,
    RootUnresolved,
    NotUtf8,
    Unreadable(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HarnessFlagsRefusal {
    UnclosedSingleQuote,
    UnclosedDoubleQuote,
    TrailingEscape,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum StartRejection {
    CompositionRefused(CompositionRefusal),
    MalformedHarnessFlags(HarnessFlagsRefusal),
    LaunchRequestConflict,
    LaunchPersistenceRefused,
    NativeLaunchRefused(NativeLaunchRefusal),
    BindingRefused(BindingRefusal),
    RegistrationRefused(RegistrationRefusal),
    IntentPersistenceRefused,
    OriginUnavailable,
    AddressAwake,
    ContextRefused(ContextRefusal),
    LeaveOutRefused(LeaveOutRefusal),
    HarnessSpawnRefused(HarnessSpawnRefusal),
    ModelRefused(ModelRefusal),
    Undeclared,
    TopicRefused(flow_ethos::Topic),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RefreshRejection {
    NotAwake,
    Locked,
    CloseRefused,
    PersistenceRefused,
    HerdrUnavailable,
    SuccessorRefused(ReplaceRejection),
    Undeclared,
    SuccessorUnconfirmed,
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
pub enum StopRejection {
    UnknownFlow,
    AlreadyStopped,
    RouteUnavailable,
    CloseRefused,
    PersistenceRefused,
    Restarting,
    SpawnUnproven,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ResumeRefused_Data {
    pub flow_id: flow_ethos::FlowId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RestartRefusal {
    UnknownFlow(flow_ethos::FlowId),
    NoHeldSession(flow_ethos::FlowId),
    TranscriptAbsent(flow_ethos::FlowId),
    Busy(flow_ethos::FlowId),
    CloseRefused(flow_ethos::FlowId),
    ResumeRefused(ResumeRefused_Data),
    Interrupted(flow_ethos::FlowId),
    Unsupported(flow_ethos::FlowId),
    Asleep(flow_ethos::FlowId),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Replaced {
    pub flow_id: flow_ethos::FlowId,
    pub launched: Launched,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ReplaceRejection {
    PredecessorAbsent,
    UnknownPredecessor,
    PredecessorStopped,
    LaunchRefused(StartRejection),
    Locked,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReapPending {
    pub address: flow_ethos::Address,
    pub predecessor: Predecessor,
    pub launched: Launched,
    pub reap_standing: ReapStanding,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ReapStanding {
    Owed,
    Unrecorded,
    PredecessorMissing,
    PredecessorUnknown,
    HerdrUnavailable,
    CloseRefused,
    HandoverRefused(HandoverCloseRefusal),
    SettleUnrecorded,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HandoverPending {
    pub address: flow_ethos::Address,
    pub handover_owner: HandoverOwner,
    pub launched: Launched,
    pub handover_close_refusal: HandoverCloseRefusal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HandoverSuperseded {
    pub address: flow_ethos::Address,
    pub handover_owner: HandoverOwner,
    pub launched: Launched,
    pub successor: Successor,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HandoverOwner(pub flow_ethos::FlowId);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HandoverCloseRefusal {
    PersistenceRefused,
    Owner(HandoverOwnerRefusal),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HandoverOwnerRefusal {
    Live(flow_ethos::FlowId),
    Missing(flow_ethos::FlowId),
    Unknown(flow_ethos::FlowId),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ObserveSelection {
    Launch(LaunchRequestId),
    Agent(flow_ethos::FlowId),
    Retirement(flow_ethos::Address),
    Undecodable,
    TurnEnds,
    Metaflows,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HandoverGeneration(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Predecessor(pub flow_ethos::FlowId);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Successor(pub flow_ethos::FlowId);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HandoverGenerationKey {
    pub address: flow_ethos::Address,
    pub handover_generation: HandoverGeneration,
    pub predecessor: Predecessor,
    pub successor: Successor,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct GitCommit(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct GitBlob(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ArchiveDocumentPath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HandoverArchiveIdentity {
    pub git_commit: GitCommit,
    pub archive_document_path: ArchiveDocumentPath,
    pub git_blob: GitBlob,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RetirementSnapshot {
    pub address: flow_ethos::Address,
    pub handover_generation_key_vector: std::vec::Vec<HandoverGenerationKey>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ArchiveCommitted {
    pub handover_generation_key: HandoverGenerationKey,
    pub handover_archive_identity: HandoverArchiveIdentity,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RetirementChange {
    GenerationClosed(HandoverGenerationKey),
    ArchiveCommitted(ArchiveCommitted),
    GenerationRemoved(HandoverGenerationKey),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RetirementObservationFailure {
    Overflow,
    MissingEvidence(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AgentState {
    Idle,
    Working,
    Blocked,
    Done,
    Unknown,
    Gone,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AgentObservation {
    pub flow_id: flow_ethos::FlowId,
    pub agent_state: AgentState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LaunchStatusRejection {
    UnknownLaunchRequest,
    PersistenceRefused,
    ReadViewChanged,
    InconsistentRecords,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CallerIdentity {
    pub flow_id: flow_ethos::FlowId,
    pub aspect: flow_ethos::Aspect,
    pub model_name: flow_ethos::ModelName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum CallerResolutionRejection {
    CallerUnknown,
    CallerMismatch(CallerIdentity),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Title(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Presentation {
    pub title: Title,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranscriptPath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SeatPane {
    pub herdr_session_name: HerdrSessionName,
    pub herdr_pane_id: HerdrPaneId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PaneLabel(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ObservedAt(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SeatEvidence {
    pub seat_pane_option: std::option::Option<SeatPane>,
    pub pane_label_option: std::option::Option<PaneLabel>,
    pub native_session_id_option: std::option::Option<NativeSessionId>,
    pub observed_at: ObservedAt,
    pub process: flow_ethos::Process,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UndecodableFrame {
    pub flow_id_option: std::option::Option<flow_ethos::FlowId>,
    pub seat_evidence: SeatEvidence,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TurnEnd {
    pub address: flow_ethos::Address,
    pub harness_kind: flow_ethos::HarnessKind,
    pub flow_id: flow_ethos::FlowId,
    pub observed_at: ObservedAt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Unknown_Data {
    Address(flow_ethos::Address),
    Lock(flow_ethos::Lock),
    FlowId(flow_ethos::FlowId),
    Key(flow_ethos::Key),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Refusal {
    Locked,
    Held(flow_ethos::Lock),
    Awake(flow_ethos::FlowId),
    Asleep,
    Lapsed,
    Unknown(Unknown_Data),
    NotConfigured,
    NotMessage,
    Taken(flow_ethos::Address),
    Unidentified(SeatEvidence),
    Store(String),
    NotSendServer(flow_ethos::Process),
    Oversized(SendPayloadSize),
    Direct(String),
    TopicRefused(flow_ethos::Topic),
    Unresolved,
    RequestIdSpaceFull,
    RequestIdDrawsCollided,
    MinutesRefused(MinutesRefusal),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MinutesRefusal {
    Zero,
    Negative,
    Unreachable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ActualBytes(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MaximumBytes(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendPayloadSize {
    pub actual_bytes: ActualBytes,
    pub maximum_bytes: MaximumBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PromptId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LastAssistantMessageOption(pub std::option::Option<String>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BackgroundTaskId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BackgroundTaskKind {
    Shell,
    Subagent,
    Monitor,
    Workflow,
    Teammate,
    CloudSession,
    McpTask,
    Other(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BackgroundTaskStatus(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BackgroundTask {
    pub background_task_id: BackgroundTaskId,
    pub background_task_kind: BackgroundTaskKind,
    pub background_task_status: BackgroundTaskStatus,
    pub description: Description,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BackgroundWork(pub std::vec::Vec<BackgroundTask>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeStop {
    pub native_session_id_option: std::option::Option<NativeSessionId>,
    pub transcript_path_option: std::option::Option<TranscriptPath>,
    pub last_assistant_message_option: LastAssistantMessageOption,
    pub background_work: BackgroundWork,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativePrompt {
    pub native_session_id_option: std::option::Option<NativeSessionId>,
    pub prompt_id_option: std::option::Option<PromptId>,
    pub prompt_sha256: PromptSha256,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VisibleResultPayload_Answered_Data {
    pub request_id: flow_ethos::RequestId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum VisibleResultPayload {
    Answered(VisibleResultPayload_Answered_Data),
    Historical(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VisibleDonePayload_Answered_Data {
    pub request_id: flow_ethos::RequestId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum VisibleDonePayload {
    Answered(VisibleDonePayload_Answered_Data),
    Historical(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendableRequest_Living_Data {
    pub context: flow_ethos::Context,
    pub verbatim: flow_ethos::Verbatim,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendableRequest_Result_Data {
    pub request_id: flow_ethos::RequestId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendableRequest_Done_Data {
    pub request_id: flow_ethos::RequestId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SendableRequest {
    Order(String),
    Question(String),
    Living(SendableRequest_Living_Data),
    Result(SendableRequest_Result_Data),
    Notice(String),
    Task(flow_ethos::AnswerableTask),
    Done(SendableRequest_Done_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SendableMessage {
    HardAbrupt(SendableRequest),
    MiddleAbrupt(SendableRequest),
    Soft(SendableRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Order_Data {
    pub request_id: flow_ethos::RequestId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Question_Data {
    pub request_id: flow_ethos::RequestId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveredRequest_Living_Data {
    pub request_id: flow_ethos::RequestId,
    pub context: flow_ethos::Context,
    pub verbatim: flow_ethos::Verbatim,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveredRequest_Result_Data {
    pub request_id: flow_ethos::RequestId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Notice_Data {
    pub request_id: flow_ethos::RequestId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Task_Data {
    pub request_id: flow_ethos::RequestId,
    pub minutes: flow_ethos::Minutes,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveredRequest_Done_Data {
    pub request_id: flow_ethos::RequestId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveredRequest_Reminder_Data {
    pub request_id: flow_ethos::RequestId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveredRequest {
    Order(Order_Data),
    Question(Question_Data),
    Living(DeliveredRequest_Living_Data),
    Result(DeliveredRequest_Result_Data),
    Notice(Notice_Data),
    Task(Task_Data),
    Done(DeliveredRequest_Done_Data),
    Reminder(DeliveredRequest_Reminder_Data),
    PriorResult(String),
    PriorDone(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveredMessage {
    HardAbrupt(DeliveredRequest),
    MiddleAbrupt(DeliveredRequest),
    Soft(DeliveredRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VisibleRequest_Living_Data {
    pub context: flow_ethos::Context,
    pub verbatim: flow_ethos::Verbatim,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VisibleRequest_Reminder_Data {
    pub request_id: flow_ethos::RequestId,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum VisibleRequest {
    Order(String),
    Question(String),
    Living(VisibleRequest_Living_Data),
    Result(VisibleResultPayload),
    Notice(String),
    Task(flow_ethos::AnswerableTask),
    Done(VisibleDonePayload),
    Reminder(VisibleRequest_Reminder_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum VisibleSender {
    Sender(flow_ethos::Address),
    Unknown,
    Notification,
    Unbound,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VisibleNativeInput {
    pub visible_sender: VisibleSender,
    pub delivered_message: DeliveredMessage,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendRequest {
    pub address: flow_ethos::Address,
    pub sendable_message: SendableMessage,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryCursor(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Namespace(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Counter(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliverKey {
    pub namespace: Namespace,
    pub counter: Counter,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StoreIdentity(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativePromptId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NativeReceipt {
    pub harness_kind: flow_ethos::HarnessKind,
    pub native_session_id: NativeSessionId,
    pub native_prompt_id: NativePromptId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RetryCause {
    HistoryAbsent,
    HandOverRefused,
    OwnerResolvedAbsent,
    WakeRefused(StartRejection),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryState {
    Queued,
    Attempting,
    Retryable(RetryCause),
    Uncertain,
    Delivered(NativeReceipt),
    Read,
    Failed(Refusal),
    Withdrawn,
    Archived,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendReceipt {
    pub logical_delivery_id: flow_ethos::LogicalDeliveryId,
    pub request_id: flow_ethos::RequestId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendFailure_Admitted_Data {
    pub logical_delivery_id: flow_ethos::LogicalDeliveryId,
    pub request_id: flow_ethos::RequestId,
    pub send_failure_reason: SendFailureReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SendFailure {
    Unadmitted(SendFailureReason),
    Admitted(SendFailure_Admitted_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SendFailureReason {
    RecipientUnavailable(flow_ethos::Address),
    QueueNotPermitted,
    Refused(Refusal),
    ReceiptUncertain,
    NotSent(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendAcceptance {
    pub logical_delivery_id: flow_ethos::LogicalDeliveryId,
    pub request_id: flow_ethos::RequestId,
    pub delivery_cursor: DeliveryCursor,
    pub delivery_state: DeliveryState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliverySnapshot {
    pub delivery_cursor: DeliveryCursor,
    pub logical_delivery_id: flow_ethos::LogicalDeliveryId,
    pub request_id: flow_ethos::RequestId,
    pub sent_by: flow_ethos::SentBy,
    pub address: flow_ethos::Address,
    pub delivery_state: DeliveryState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SendObservation {
    pub store_identity: StoreIdentity,
    pub delivery_cursor: DeliveryCursor,
    pub delivery_snapshot_vector: std::vec::Vec<DeliverySnapshot>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DrainState {
    Delivered,
    Held(std::vec::Vec<flow_ethos::LogicalDeliveryId>),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DrainOutcome {
    pub address: flow_ethos::Address,
    pub drain_state: DrainState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PrivateSendResponse {
    SendAccepted(SendAcceptance),
    Refused(Refusal),
    Sent(SendReceipt),
    Failed(SendFailure),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum VisibleRefusal {
    Locked,
    Held(flow_ethos::Lock),
    Awake,
    Asleep,
    Lapsed,
    Unknown(VisibleUnknown),
    NotConfigured,
    NotMessage,
    Taken(flow_ethos::Address),
    Unidentified(SeatEvidence),
    Store(String),
    NotSendServer(flow_ethos::Process),
    Oversized(SendPayloadSize),
    Direct(String),
    TopicRefused(flow_ethos::Topic),
    Unresolved,
    RequestIdSpaceFull,
    RequestIdDrawsCollided,
    MinutesRefused(MinutesRefusal),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum VisibleUnknown {
    Address(flow_ethos::Address),
    Lock(flow_ethos::Lock),
    Flow,
    Key(flow_ethos::Key),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum VisibleDeliveryState {
    Queued,
    Attempting,
    Retryable(RetryCause),
    Uncertain,
    Delivered,
    Read,
    Failed(VisibleRefusal),
    Withdrawn,
    Archived,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VisibleSendReceipt(pub flow_ethos::RequestId);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VisibleSendFailure_Admitted_Data {
    pub request_id: flow_ethos::RequestId,
    pub visible_send_failure_reason: VisibleSendFailureReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum VisibleSendFailure {
    Unadmitted(VisibleSendFailureReason),
    Admitted(VisibleSendFailure_Admitted_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum VisibleSendFailureReason {
    RecipientUnavailable(flow_ethos::Address),
    QueueNotPermitted,
    Refused(VisibleRefusal),
    ReceiptUncertain,
    NotSent(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VisibleSendAcceptance {
    pub request_id: flow_ethos::RequestId,
    pub delivery_cursor: DeliveryCursor,
    pub visible_delivery_state: VisibleDeliveryState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VisibleDeliverySnapshot {
    pub request_id: flow_ethos::RequestId,
    pub delivery_cursor: DeliveryCursor,
    pub visible_sender: VisibleSender,
    pub address: flow_ethos::Address,
    pub visible_delivery_state: VisibleDeliveryState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VisibleSendObservation {
    pub store_identity: StoreIdentity,
    pub delivery_cursor: DeliveryCursor,
    pub visible_delivery_snapshot_vector: std::vec::Vec<VisibleDeliverySnapshot>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ArchiveRejection {
    UnknownRequest,
    NotYet,
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RequestRejection {
    UnknownRequest,
    NotYet,
    Closed,
    NotTask,
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Priority {
    HardAbrupt,
    MiddleAbrupt,
    Soft,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Queue {
    pub address: flow_ethos::Address,
    pub visible_native_input_vector: std::vec::Vec<VisibleNativeInput>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AnswerRejection {
    UnknownRequest,
    Open,
    Routed,
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Warmth {
    pub address: flow_ethos::Address,
    pub minutes_option: std::option::Option<flow_ethos::Minutes>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum VisibleDrainState {
    Delivered,
    Held,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VisibleDrainOutcome {
    pub address: flow_ethos::Address,
    pub visible_drain_state: VisibleDrainState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Description(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Body(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Section {
    pub title: Title,
    pub description: Description,
    pub body: Body,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Heading {
    pub title: Title,
    pub description: Description,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Known_Data {
    pub flow_id: flow_ethos::FlowId,
    pub tokens: Tokens,
    pub timestamp: Timestamp,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Touch {
    Unknown,
    Known(Known_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HandoverRefusal {
    MissingTitle,
    UnknownFlow,
    NotOwner,
    Store(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Event {
    Started(std::option::Option<NativeSessionId>),
    ToolUsed(String),
    Stopped(NativeStop),
    PromptSubmitted(NativePrompt),
    Ended(std::option::Option<NativeSessionId>),
    Cleared(std::option::Option<NativeSessionId>),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Report_Data {
    pub flow_id: flow_ethos::FlowId,
    pub process_option: std::option::Option<flow_ethos::Process>,
    pub event: Event,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Lock_Data {
    pub first_address: flow_ethos::Address,
    pub second_address: flow_ethos::Address,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Bind_Data {
    pub address: flow_ethos::Address,
    pub process: flow_ethos::Process,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Deliver_Data {
    pub lock: flow_ethos::Lock,
    pub deliver_key: DeliverKey,
    pub sendable_message: SendableMessage,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct IdentifyThread_Data {
    pub process: flow_ethos::Process,
    pub native_session_id: NativeSessionId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ObserveSender_Data {
    pub process: flow_ethos::Process,
    pub native_session_id_option: std::option::Option<NativeSessionId>,
    pub delivery_cursor_option: std::option::Option<DeliveryCursor>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BindSeat_Data {
    pub address: flow_ethos::Address,
    pub seat_pane: SeatPane,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Adopt_Data {
    pub address: flow_ethos::Address,
    pub seat_pane: SeatPane,
    pub native_session_id: NativeSessionId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Keep_Data {
    pub address: flow_ethos::Address,
    pub section: Section,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Drop_Data {
    pub address: flow_ethos::Address,
    pub title: Title,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Push_Data {
    pub address: flow_ethos::Address,
    pub priority: Priority,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Remind_Data {
    pub address: flow_ethos::Address,
    pub minutes: flow_ethos::Minutes,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Start(LaunchProfile),
    Refresh(flow_ethos::Address),
    ResolveRecipient(RecipientResolutionRequest),
    Stop(flow_ethos::FlowId),
    Replace(LaunchProfile),
    LaunchStatus(LaunchRequestId),
    Observe(ObserveSelection),
    ResolveCaller(std::option::Option<flow_ethos::FlowId>),
    Report(Report_Data),
    Lock(Lock_Data),
    Bind(Bind_Data),
    Identify(flow_ethos::Process),
    Deliver(Deliver_Data),
    Acknowledge(std::vec::Vec<DeliverKey>),
    ArchiveRequest(flow_ethos::RequestId),
    Release(flow_ethos::Lock),
    Current(flow_ethos::Address),
    Metaflows,
    IdentifyThread(IdentifyThread_Data),
    Send(SendRequest),
    ObserveSends(std::option::Option<DeliveryCursor>),
    ObserveSender(ObserveSender_Data),
    ObserveDeliveries(std::option::Option<DeliveryCursor>),
    DeliverAll(flow_ethos::Address),
    Launch(LaunchRequest),
    Rehearse(LaunchRequest),
    BindSeat(BindSeat_Data),
    Adopt(Adopt_Data),
    Restart(flow_ethos::FlowId),
    Keep(Keep_Data),
    Drop(Drop_Data),
    Glance(flow_ethos::Address),
    Read(flow_ethos::Address),
    Cancel(flow_ethos::RequestId),
    Silence(flow_ethos::RequestId),
    Clear(flow_ethos::Address),
    ClearAll,
    Push(Push_Data),
    Queues,
    Hot,
    Remind(Remind_Data),
    Answer(flow_ethos::RequestId),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Current_Data {
    Awake(flow_ethos::FlowId),
    Asleep,
    Unknown,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Kept_Data {
    pub address: flow_ethos::Address,
    pub title: Title,
    pub touch: Touch,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Dropped_Data {
    pub address: flow_ethos::Address,
    pub title: Title,
    pub touch: Touch,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Started(Launched),
    LaunchPending(LaunchAttempt),
    StartAmbiguous(PromptDeliveryIntent),
    Refreshed(Refreshed),
    RecipientResolved(FlowNode),
    Stopped(flow_ethos::FlowId),
    Listed(std::vec::Vec<flow_ethos::Metaflow>),
    StartRejected(StartRejection),
    Rehearsed(Command),
    RehearseRejected(StartRejection),
    RefreshRejected(RefreshRejection),
    RecipientResolutionRejected(RecipientResolutionRejection),
    StopRejected(StopRejection),
    Replaced(Replaced),
    ReplaceRejected(ReplaceRejection),
    LaunchInspected(LaunchInspection),
    LaunchStatusRejected(LaunchStatusRejection),
    CallerResolved(CallerIdentity),
    CallerResolutionRejected(CallerResolutionRejection),
    AgentObserved(AgentObservation),
    RetirementObserved(RetirementSnapshot),
    RetirementChanged(RetirementChange),
    RetirementObservationFailed(RetirementObservationFailure),
    Reported,
    Locked(flow_ethos::Lock),
    Bound(flow_ethos::FlowId),
    Identified(flow_ethos::Address),
    SendAccepted(VisibleSendAcceptance),
    SendsObserved(VisibleSendObservation),
    SendChanged(VisibleDeliverySnapshot),
    Drained(VisibleDrainOutcome),
    Woken(flow_ethos::FlowId),
    Adopted(flow_ethos::FlowId),
    Released,
    Current(Current_Data),
    Refused(Refusal),
    SendRefused(VisibleRefusal),
    Sent(VisibleSendReceipt),
    Failed(VisibleSendFailure),
    Restarted(flow_ethos::FlowId),
    RestartRefused(RestartRefusal),
    KeepRefused(HandoverRefusal),
    DropRefused(HandoverRefusal),
    GlanceRefused(HandoverRefusal),
    ReadRefused(HandoverRefusal),
    Kept(Kept_Data),
    Dropped(Dropped_Data),
    Acknowledged,
    DeliverKeyReused,
    RequestArchived(flow_ethos::RequestId),
    ArchiveRejected(ArchiveRejection),
    Glanced(std::vec::Vec<Heading>),
    Handover(std::vec::Vec<Section>),
    RefreshQueued(RefreshQueued),
    FrameRefused(UndecodableFrame),
    UndecodableObserved(std::vec::Vec<UndecodableFrame>),
    TurnEndsObserved(std::vec::Vec<TurnEnd>),
    TurnEnded(TurnEnd),
    MetaflowsObserved(std::vec::Vec<flow_ethos::Metaflow>),
    MetaflowChanged(flow_ethos::Metaflow),
    MetaflowGone(flow_ethos::Address),
    Cancelled(flow_ethos::RequestId),
    CancelRejected(RequestRejection),
    Silenced(flow_ethos::RequestId),
    SilenceRejected(RequestRejection),
    Cleared(std::vec::Vec<flow_ethos::RequestId>),
    Pushed(std::vec::Vec<flow_ethos::RequestId>),
    Queues(std::vec::Vec<Queue>),
    Hot(std::vec::Vec<Warmth>),
    Reminded(flow_ethos::RequestId),
    Answered(VisibleNativeInput),
    AnswerRejected(AnswerRejection),
    RestartQueued(flow_ethos::FlowId),
    ReapPending(ReapPending),
    HandoverPending(HandoverPending),
    HandoverSuperseded(HandoverSuperseded),
}
