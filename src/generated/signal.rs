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
pub type BindingGeneration = i64;
#[rustfmt::skip]
pub type LifecycleGeneration = i64;
#[rustfmt::skip]
pub type ProcessId = i64;
#[rustfmt::skip]
pub type ProcessStartTime = i64;
#[rustfmt::skip]
pub type EndpointPath = String;
#[rustfmt::skip]
pub type HerdrSessionName = String;
#[rustfmt::skip]
pub type HerdrAgentName = String;
#[rustfmt::skip]
pub type HerdrPaneId = String;
#[rustfmt::skip]
pub type HerdrTerminalId = String;
#[rustfmt::skip]
pub type NativeThread = String;
#[rustfmt::skip]
pub type HarnessSession = String;
#[rustfmt::skip]
pub type RouteIdentity = String;
#[rustfmt::skip]
pub type EndpointIdentity = String;
#[rustfmt::skip]
pub type AttemptId = String;
#[rustfmt::skip]
pub type SourceEventIdentifier = String;
#[rustfmt::skip]
pub type DeliveryToken = String;
#[rustfmt::skip]
pub type TransitionId = String;
#[rustfmt::skip]
pub type RegistrationId = String;
#[rustfmt::skip]
pub type TransportReceiptId = String;
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
    pub flow_type: FlowType,
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
    UnknownFlowType,
    LaunchRefused,
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
pub struct DeliveryBinding {
    pub native_thread: NativeThread,
    pub harness_session: HarnessSession,
    pub route_identity: RouteIdentity,
    pub endpoint_identity: EndpointIdentity,
    pub process_id: ProcessId,
    pub process_start_time: ProcessStartTime,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ExpectedOldBinding {
    pub native_thread: NativeThread,
    pub harness_session: HarnessSession,
    pub route_identity: RouteIdentity,
    pub endpoint_identity: EndpointIdentity,
    pub process_id: ProcessId,
    pub process_start_time: ProcessStartTime,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ExpectedOldBindingGeneration {
    pub binding_generation: BindingGeneration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryPermit {
    pub attempt_id: AttemptId,
    pub source_event_identifier: SourceEventIdentifier,
    pub delivery_token: DeliveryToken,
    pub binding_generation: BindingGeneration,
    pub delivery_binding: DeliveryBinding,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefreshHeld_Data {
    pub transition_id: TransitionId,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AdmissionGate {
    Open,
    RefreshHeld(RefreshHeld_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryState {
    pub delivery_binding: DeliveryBinding,
    pub binding_generation: BindingGeneration,
    pub lifecycle_generation: LifecycleGeneration,
    pub admission_gate: AdmissionGate,
    pub delivery_permit_option: Option<DeliveryPermit>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AcquireDelivery {
    pub flow_id: FlowId,
    pub delivery_binding: DeliveryBinding,
    pub binding_generation: BindingGeneration,
    pub attempt_id: AttemptId,
    pub source_event_identifier: SourceEventIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BeginRefresh {
    pub flow_id: FlowId,
    pub binding_generation: BindingGeneration,
    pub transition_id: TransitionId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReleaseConfirmed {
    pub flow_id: FlowId,
    pub attempt_id: AttemptId,
    pub delivery_token: DeliveryToken,
    pub delivery_binding: DeliveryBinding,
    pub binding_generation: BindingGeneration,
    pub transport_receipt_id: TransportReceiptId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReadyReattach {
    pub flow_id: FlowId,
    pub transition_id: TransitionId,
    pub expected_old_binding: ExpectedOldBinding,
    pub expected_old_binding_generation: ExpectedOldBindingGeneration,
    pub registration_id: RegistrationId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReadDeliveryState {
    pub flow_id: FlowId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefreshHeld {
    pub delivery_permit_option: Option<DeliveryPermit>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReleaseReceipt {
    pub attempt_id: AttemptId,
    pub transport_receipt_id: TransportReceiptId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReattachReceipt {
    pub binding_generation: BindingGeneration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryRejection {
    UnknownFlow,
    MissingState,
    BindingUnavailable,
    StaleBinding,
    StaleGeneration,
    RefreshHeld(RefreshHeld),
    Busy,
    AttemptConflict,
    StalePermit,
    TransitionConflict,
    ActivePermit,
    CorruptState,
    GenerationOverflow,
    CapacityExhausted,
    StoreRefused,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Start(StartRequest),
    Restart(RestartRequest),
    ResolveRecipient(RecipientResolutionRequest),
    AcquireDelivery(AcquireDelivery),
    BeginRefresh(BeginRefresh),
    ReleaseConfirmed(ReleaseConfirmed),
    ReadyReattach(ReadyReattach),
    ReadDeliveryState(ReadDeliveryState),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Started(Started),
    Restarted(Restarted),
    RecipientResolved(FlowNode),
    StartRejected(StartRejection),
    RestartRejected(RestartRejection),
    RecipientResolutionRejected(RecipientResolutionRejection),
    DeliveryGranted(DeliveryPermit),
    DeliveryAlreadyGranted(DeliveryPermit),
    DeliveryAcquireRejected(DeliveryRejection),
    RefreshHeld(RefreshHeld),
    RefreshAlreadyHeld(RefreshHeld),
    RefreshRejected(DeliveryRejection),
    DeliveryReleased(ReleaseReceipt),
    DeliveryAlreadyReleased(ReleaseReceipt),
    DeliveryReleaseRejected(DeliveryRejection),
    DeliveryReattached(ReattachReceipt),
    DeliveryAlreadyReattached(ReattachReceipt),
    DeliveryReattachRejected(DeliveryRejection),
    DeliveryStateRead(DeliveryState),
    DeliveryStateRejected(DeliveryRejection),
}
