# Upgrades

## 39.0.0: configured Claude lifecycle is typed

- `Response` gains the unit `ClaudeReady` variant. The configured Claude
  SessionStart hook sends its rkyv archive under the standard big-endian u32
  length prefix on Flow's private per-launch readiness transport.
- `PaneCloseOutcome` is `Closed` or `Refused.PaneCloseFailure`.
  `PaneCloseFailure` distinguishes `IdentityUnavailable`,
  `IdentityMismatch`, `StopTimedOut`, and `HerdrCloseFailed`.
- `RefreshRejection.CloseRefused` and `StopRejection.CloseRefused` carry the
  same `PaneCloseFailure` end to end. Their successful replies stay unchanged.

## 38.0.0: every launch uses the regular path

- `FlowKind` and the Simplified launch shape are removed. `LaunchProfile`
  no longer carries `FlowKind`, and `CompositionRefusal` no longer carries
  `InvalidToolSet` or `PresentationUnavailable`. Consumers provide
  presentation instructions through context modules.

## 37.0.0: harness events name their reporting session

- `Report` identifies its reporter as either a Flow session or a native
  harness session. `Event` includes harness notifications.

## 36.0.0: a caller carries its layer

- `Caller` carries `Layer` directly. `PowerLevel` is removed; a caller no
  longer encodes its layer a second way.

## 35.0.0: declarations name required metaflows

- `StartRejection.NoSecondary` is replaced by `RequirementUnmet.Address`.
  Flow starts each Address in a declaration’s `Requires` before that
  metaflow and names the requirement when it cannot be started.

How to deploy each breaking change of signal-flow.

## 34.0.0: an abrupt request may be queued

- `SendFailureReason` loses `QueueNotPermitted`: `Delivery.Queue` admits
  every priority to the recipient's durable queue. Consumers upgrade a
  stored failure of that variant to `NotSent.«queue not permitted»`.

## 33.0.0: a Send is never refused for the seat's state or its route

- `SendFailureReason` loses `RecipientBusy.Address`: no path produces it.
  A Send to a seat with a queue, an outstanding delivery, a hold or a
  passed context threshold is queued and answers
  `SendAccepted.{ ... Queued }`. `RecipientUnavailable` stays.
- `Refusal` loses `OffRoute`: a Send is not refused for its sender's aspect,
  topic or layer. Consumers drop both variants; no compatibility path.

## 32.0.0: Refresh replaces Restart; a harness session's end is an event

- `Query` loses `Restart.FlowId` and gains `Refresh.Address`:
  `Refresh.{ Psyche core Primary }` ends the flow the metaflow is Awake as,
  without a turn, under the metaflow's lock; a Send arriving under the lock
  queues. A Maintained metaflow, or one whose queue holds a waking request,
  has its successor started and confirmed before the ending flow is reaped;
  any other is left Asleep and wakes on its next waking request. A Launch
  addressed to a metaflow Awake as a live flow is such a Refresh, its
  successor the Launch's, and answers `Replaced`.
- `Response` loses `Restarted` and `RestartRejected` and gains
  `Refreshed.{ Address FlowId }`, naming the metaflow and the flow that
  ended, and `RefreshRejected.RefreshRejection`. `Generation` leaves with
  `Restarted`.
- `RefreshRejection` is `[ NotAwake Locked CloseRefused PersistenceRefused
  HerdrUnavailable SuccessorRefused.ReplaceRejection Undeclared
  SuccessorUnconfirmed ]`. `SuccessorRefused` carries the rejection of the
  Replace that starts an owed successor; `Undeclared` answers a successor
  no declaration or configured model names; `SuccessorUnconfirmed` answers
  a Refresh whose owed successor's launch is still pending or ambiguous
  when the confirmation wait ends, and the successor stays owed:
  `RefreshRejected.SuccessorRefused.Locked`.
- `StartRejection.CompositionRefused` carries its reason,
  `CompositionRefusal.[ InvalidLaunchRequestId InvalidModelName
  InvalidEffort InvalidToolSet InvalidHerdrSessionName InvalidSystemModules
  InvalidUserModules InvalidRemembering InexactModel Unconfigured
  PresentationUnavailable Unrecorded ]`: a Launch remembering `Custom.[]`
  answers `StartRejected.CompositionRefused.InvalidRemembering`.
  `Unrecorded` answers an outcome archived before the reason was kept. A
  bare `StartRejected.CompositionRefused` no longer reads.
- `ReplaceRejection` appends `Locked`: a Replace whose metaflow a refresh
  or a waking holds is refused `ReplaceRejected.Locked`. A refresh's or a
  replace's successor is owed in Flow's memory until it is confirmed, and a
  Nexus that starts with one owed starts it.
- `DeliveryState` appends `Withdrawn`: a queued request the owner withdraws
  from its metaflow's queue (meta `Withdraw`) before it reached the seat
  ends `Withdrawn`, so its sender sees its delivery end.
- `RetryCause` appends `WakeRefused.StartRejection`: a queued request whose
  waking start was refused stays queued, its delivery
  `Retryable.WakeRefused.<rejection>`, e.g.
  `Retryable.WakeRefused.ContextRefused.RecordMissing`, so the sender's
  `ObserveSends` shows why; the next waking request retries the start.
- `Refusal` appends `TopicRefused.Topic`: a Send whose recipient Address's
  topic is not an uncapitalized camelCase identifier is refused,
  `Refused.TopicRefused.Primary_Committing`.
- `Event` appends `Ended.Option<NativeSessionId>`: Claude's SessionEnd
  hook reports `Report.{ FlowId Ended.Some.«session» }`, and Flow records
  the flow Exited when the session is its own, with nothing looking at its
  pane.
- `Event` appends `Cleared.Option<NativeSessionId>`: a SessionEnd with
  reason `clear` reports `Report.{ FlowId Cleared.Some.«session» }`. The
  flow has not exited; its pane stays.
- `Event`'s `Started` carries `Option<NativeSessionId>`, the session
  SessionStart names: `Report.{ FlowId Started.Some.«session» }`. A Started
  that follows the flow's own Cleared rebinds the flow to its new session;
  its FlowId stays. A bare `Started` no longer reads; Flow's hook and its
  Nexus deploy together.
- Every ordinary-socket peer and every Flow store repins; a store keeping
  events upgrades them.

## 31.0.0: a topic that is not camelCase is refused by type

- `StartRejection` appends `TopicRefused.Topic`: a Start, Replace or
  Launch whose Address's topic is not an uncapitalized camelCase
  identifier is refused before anything launches or is written, carrying
  the topic as written: `StartRejected.TopicRefused.Primary_Committing`.
- `NoSecondary` now means the Secondary of the Primary's aspect and topic
  is not Awake as a live flow; a recorded but Asleep Secondary no longer
  admits its Primary.
- The variant is appended; earlier values keep their discriminants.

## 27.0.0: oversized Send requests have a typed refusal

- `Refusal` appends `Oversized.SendPayloadSize`, carrying the rendered
  native-prompt byte length and Flow's maximum. A peer can distinguish a
  Send rejected before delivery or queueing from other refusals.
- The variant is appended; earlier values retain their discriminants.

## 26.0.0: the sender chooses whether delivery wakes

- `Send` is now `Send.{ Recipient Delivery Message }`, where
  `Delivery.[ Queue Wake ]` makes waking an explicit sender choice rather
  than a consequence of the message kind.
- `Query` appends `DeliverAll.Address`; an empty queue is a successful no-op.
  Every ordinary-socket producer and consumer repins before using either
  changed query.

## 25.0.0: a request Flow raised itself is sent by Flow

- flow-ethos (flow dc2cba6) appends `Flow` to `SentBy.[ Sender Unknown
  Flow ]`: a queued request Flow raised itself, such as a landing's orders
  and results or a compaction's investigation, reads `{ Flow Order.«…» }`.
- The variant is appended, so a 24.0.0 peer reads every earlier queued
  request unchanged; a 24.0.0 peer reading a `Flow` sender refuses that
  value. Every peer that reads `Listed` or a queued request repins, and a
  store keeping archived `QueuedRequest`s gives it a new identity. The
  contract digest is unchanged: this contract's own source did not change.

## 24.0.0: a flow sends, launches and binds through Flow itself

- `Query` appends `Send.{ Recipient Message }`, `Launch.{ Address
  InstructionPrompt }` and `BindSeat.Address`; `Message.[ HardAbrupt.Request
  MiddleAbrupt.Request Soft.Request ]` is the request under its Priority
  head. `Refusal` appends `OffRoute`; `StartRejection` appends
  `Undeclared`.
- Every change is an appended variant: no earlier discriminant moves and no
  earlier type changes shape, so a peer built against 23.0.0 that sends
  none of the new queries keeps reading and writing the same bytes. Rust
  dependents that link signal-flow beside meta-signal-flow repin both.

## 23.0.0: a metaflow is Awake or Asleep; a Primary starts after its Secondary

- A topic never dies: it goes idle. `Query.End`, `Response.Ended`,
  `Current.Ended`, `Refusal.Ended` and `StartRejection.AddressEnded` are
  removed, and flow-ethos `State` is `[ Awake.FlowId Asleep ]` (flow
  06171a5). `Refusal.OffRoute`, which only `End` answered, is removed. `End.{ Psyche flow Primary }` and `Refused.Ended.{ Psyche flow
  Primary }` no longer read. Every later `Query`, `Response`, `Refusal` and
  `StartRejection` discriminant moves, so every ordinary-socket peer and
  every store that keeps an archived `StartRejection` or `State` upgrades.
- `StartRejection` appends `NoSecondary`: a Start whose Address is a
  Primary is refused before anything launches or is written unless a
  metaflow is recorded, Awake or Asleep, at the Secondary of its aspect and
  topic: `StartRejected.NoSecondary`.
- The contract digest changes from 5aa2992669c14992 to 32765a41a9454b06:
  every ordinary-socket peer repins.

## 22.0.0: workspace label refusal, model refusal, thread identification, native Stop and prompt identity

- `NativeLaunchFailure` appends `WorkspaceLabelRefused`: Herdr refused the
  label Flow sets on the started flow's workspace
  (`StartRejected.NativeLaunchRefused.{ Naming WorkspaceLabelRefused }`).
  `PaneLabelRefused` remains the pane's own label.
- `StartRejection` appends `ModelRefused.ModelRefusal`,
  `ModelRefusal.[ Unconfigured Differs.ModelName ]`: the profile's model is
  not the model configured for its Address's layer and aspect, compared name
  for name; `Differs` carries the configured model, `Unconfigured` says none
  is configured for that layer and aspect. The Nexus refuses such a Start, or
  the successor of a Replace, before anything launches or is written:
  `StartRejected.ModelRefused.Differs.claude-opus-5-5`,
  `ReplaceRejected.LaunchRefused.ModelRefused.Unconfigured`.
- `Query` gains `IdentifyThread.{ Process NativeSessionId }`: Message sends
  it for an MCP send server, with the process its socket reports for that
  server and the thread id Codex stamped on the call
  (`IdentifyThread.{ { 4242 987654 } 01a0fdcc-c1a1-7f43-8c23-06d41fa34438 }`).
  It answers `Identified.Address`, `Refused.Unidentified.Process`,
  `Refused.NotConfigured`, `Refused.NotMessage`, or the new
  `Refused.NotSendServer.Process`. `Refusal` appends `NotSendServer.Process`.
- `Event.Stopped` carries `NativeStop.{ Option<NativeSessionId>
  Option<TranscriptPath> }`, the session and transcript the harness gave its
  Stop hook: `Report.{ f1c841 Stopped.{ Some.s1 Some./p/s1.jsonl } }`.
- `Event` appends `PromptSubmitted.NativePrompt`, `NativePrompt.{
  Option<NativeSessionId> Option<PromptId> PromptSha256 }`: the SHA-256 of a
  prompt the harness took, with its paste wrapper taken off, and the
  harness's prompt id when it gave one.
- `Query.QueueTurnEnd`, `Response.TurnEndQueued`, `Response.TurnEndRejected`,
  `TurnEndRequest`, `TurnEndRejection` and `TurnId` are removed: the
  correlated Stop report is the turn end. Every later `Query` and `Response`
  discriminant moves, so every ordinary-socket peer and every store that
  keeps an archived `Event` upgrades. The appended `NativeLaunchFailure`,
  `StartRejection` and `Refusal` variants leave every earlier discriminant
  of those enums unchanged.
- The contract digest changes from 5bf78e23e2f14c52 to 5aa2992669c14992:
  every ordinary-socket peer repins.

## 20.0.0: a queued request carries its sender; the send tool's refusal has its category

- A metaflow's queue holds `QueuedRequest.{ SentBy Request }` (flow-ethos),
  `SentBy.[ Sender Unknown ]`: the Sender Flow resolved for the request, or
  `Unknown` for one queued before Flow kept senders. `Listed` carries it:
  `Listed.[ { { Psyche flow Primary } Ended [ past-1 ] [ { Sender.{ Mind flow
  Tertiary } Order.continue } ] } ]`. A store that keeps queues upgrades its
  metaflow table, each older entry reading `Unknown`.
- `NativeLaunchFailure` appends `SendToolUnsupported`: the Codex app-server
  was not shown to accept Flow's send tool, so no thread was opened
  (`StartRejected.NativeLaunchRefused.{ CodexThread SendToolUnsupported }`).
  It is appended last, so every earlier archived discriminant is unchanged.
- flow-ethos is pinned at flow 9e417bb.
- The contract digest changes from baa84beb4f2cc84b to 5bf78e23e2f14c52:
  every ordinary-socket peer repins.

## 19.0.0: a refused registration and a refused native step carry their stage

`StartRejection`'s `NativeLaunchRefused` and `RegistrationRefused` carry
their typed reason; the archived discriminants of every variant are
unchanged, but both variants now carry data, so an archived
`StartRejection` has a new size and a store that keeps one upgrades its
table.

- `RegistrationRefused.RegistrationRefusal`, `RegistrationRefusal.[
  RouteUnverified RoleConflict RoleUnrecorded PaneBindingForeign
  AcknowledgementMismatch PromptNotCanonical NativeTarget.NativeTargetRefusal
  TranscriptBoundary ThreadNotNamed Unclassified ]`, `NativeTargetRefusal.[
  HerdrUnreadable PaneMismatch NoNativeSession IdentityInexact ]`:
  `StartRejected.RegistrationRefused.NativeTarget.IdentityInexact`.
- `NativeLaunchRefused.NativeLaunchRefusal`, `NativeLaunchRefusal.{
  NativeLaunchStage NativeLaunchFailure }`, `NativeLaunchStage.[ Open
  CodexThread Spawn Bind Naming Receipt Confirm Unclassified ]`, and
  `NativeLaunchFailure` the fixed category, no adapter text:
  `StartRejected.NativeLaunchRefused.{ Receipt PartDiffers }`.
- `Unclassified` (in `RegistrationRefusal`, `NativeLaunchStage` and
  `NativeLaunchFailure`) is what a stored 18.0.0 unit refusal upgrades to.
- The contract digest changes: every ordinary-socket peer repins.

## 18.0.0: a refused harness start carries its cause

`StartRejection` appends `HarnessSpawnRefused.HarnessSpawnRefusal`; the
archived discriminants of its other variants are unchanged, but the variant
carries text, so an archived `StartRejection` has a new size and a store that
keeps one upgrades its table.

- `HarnessSpawnRefusal.{ SpawnRefusalCause Option<OpenedThread> }`.
- `SpawnRefusalCause.[ Herdr.HerdrRefusal ControlCharacter CommandTooLong ]`:
  Herdr refused the agent start, or Flow refused it before calling Herdr.
- `HerdrRefusal.{ Option<HerdrErrorCode> HerdrErrorMessage }` keeps Herdr's
  JSON error code and message, or no code and Herdr's whole printed text.
- `OpenedThread.{ NativeSessionId FirstTurn ThreadArchive }` accounts for a
  Codex thread opened before the refused start:
  `FirstTurn.[ Answered InProgress Unanswered Unread ]`,
  `ThreadArchive.[ Archived ArchiveRefused ]`.
- `NativeLaunchRefused` now names only a failure after the harness started.
- The contract digest changes: every ordinary-socket peer repins.

## 17.0.0: a launch names its context by module identity

`LaunchProfile` names the seat's context only by module identity:
`SystemModules` and `UserModules` (each `Vector<ModuleIdentity>`) take the
places of `Vector<LaunchSource>` and `Vector<SkillName>`, and
`SystemPromptBundleFile` is removed. `LaunchSource`, `SourcePath`,
`SourceSha256`, `SkillName`, `NativeSkillSelection`, `NativeSkillPath`,
`NativeSkillSha256` and `FirstPromptText` are removed.

- A Start datom names the two module vectors where it named sources and
  skills, and no bundle file:
  `Start.{ id [ spirit-core ] [ operation-main-flow ] { Field flow
  Quaternary } haiku medium None [] session None «brief» }`.
- `ComposedLaunch` carries the composed `SystemPrompt` (text and SHA-256);
  `FirstPromptPayload` carries the `PromptPart` vector the first USER
  message is sent as.
- `NativeLaunchIntent`, `PromptDeliveryIntent` and `NativeTargetReceipt`
  carry the part SHA-256 vector where they carried skill selections;
  `PromptDeliveryIntent` adds `PartsSubmitted` and `StopsReported`, and
  `NativeLaunchIntent` adds `SystemPromptSha256`.
- `Launched` adds the flow's `HerdrRouteSelection` between `SessionId` and
  `OriginClue`.
- `StartRejection` appends `ContextRefused.ContextRefusal`; the archived
  discriminants of its other variants are unchanged.
- A `LeaveOut` names one part of a harness start a flow goes without (a
  tool or tool group, an instruction block, a harness feature).
  `LaunchProfile` carries `Option<LeaveOuts>` before `InstructionPrompt`
  (None takes the metaflow's configured default); `ComposedLaunch` and
  `PromptDeliveryIntent` carry the effective `LeaveOuts`; `StartRejection`
  appends `LeaveOutRefused.LeaveOutRefusal`.

The build generates with ethos-zero 17.0.0 at 07714b0; the generated module
is byte-identical to 16.0.0's output for the same source.

- `Start` and `Replace` carry the `LaunchProfile` itself and `Restart` the
  `FlowId`; `StartRequest`, `RestartRequest` and `OriginClue` are removed.
  The Nexus derives a launch's origin from the calling process:
  `Origin.[ Flow.{ FlowId SessionId } Unbound ]`, which `FlowNode`,
  `Launched` and `LaunchAttempt` carry where they carried `OriginClue`.
  There is no TurnId.
- `LaunchProfile` no longer names `PowerLevel` (its Address layer gives it)
  or `HarnessKind` (Codex when a configured Codex endpoint serves its model,
  else Claude); `ComposedLaunch` carries the derived `HarnessKind`.

Repin signal-flow 17.0.0 in every consumer together with the Flow Nexus;
stored launch attempts of 16.0.0 no longer read as 17.0.0.

## 16.0.0: a launch names its Address

`LaunchProfile` carries the launched seat's `Address` (`{ Aspect Topic
Layer }`) in the position that held its `Aspect`. `StartRejection` gains
`AddressAwake` and `AddressEnded`. The contract digest changes, so an
older peer is refused at the greeting.

- A Start datom names the Address where it named the Aspect:
  `Start.{ { id [] [ skills ] { Field flow Quaternary } UltraLow Claude
  haiku medium None [] session bundle «prompt» } { origin session turn } }`.
- When the launch is confirmed, Flow records that Address's metaflow Awake
  as the launched flow. A Start whose Address is Awake as a live flow, or
  named by another unsettled launch, answers `StartRejected.AddressAwake`;
  one whose Address was ended answers `StartRejected.AddressEnded`.
- A stored `LaunchProfile` of 15.0.0 no longer reads as 16.0.0; a consumer
  that keeps launch attempts must carry or set aside its old rows.

Repin signal-flow 16.0.0 in every consumer, adapt each `LaunchProfile`
construction and each `aspect` read to `address`, handle the two new
rejections, and rebuild and restart the Flow Nexus and its clients together.

## 15.0.0: one shared Aspect type

`LaunchProfile` and `Caller` now use the Library's `Aspect` directly;
`FlowAspect` is removed. Regenerate and pin this release with the matching
`flow-ethos` source, then rebuild every SignalFlow consumer together. The
contract digest changes, so an older peer is refused at the greeting.

## 14.0.0: Metaflows and store refusals

The ordinary contract returns address-indexed Metaflow records from
`Query::Metaflows`; the old native-flow `List` response is removed. Any
ordinary query whose store operation fails now answers `Refused.Store` with
the store's error text, rather than an empty list or an unrelated rejection.
Consumers must regenerate and repin signal-flow 14.0.0 with the matching Flow
Nexus before using this contract.

## 13.0.0: Metaflows replaces List

The ordinary wire contract changes. Rebuild and restart every signal-flow
consumer and its CLI together; the contract digest changes and an older peer
is refused at the greeting.

- `Query::List(ListRequest)` is removed. `Query::Metaflows` is the typed query
  for Flow's durable metaflow records.
- `Response::Listed` now carries `Vec<flow_ethos::Metaflow>`, not
  `Vec<FlowNode>`. Native Flow seat rows remain internal to Flow's launch and
  lifecycle implementation; they are not the Metaflows response.
- `FlowList` and `ListRequest` are removed from the generated contract.

Repin signal-flow 13.0.0 and regenerate from this release's `ethos/signal.ethos`
in every consumer. Adapt each former `List` caller to `Metaflows` only when it
needs metaflow records; code that needs native Flow seat rows must use its
owned internal store path. Rebuild and restart the Nexus and clients together.

## 11.0.0: typed binding refusals and LaunchInspection

The wire changes. Rebuild and restart each signal-flow consumer and
its CLI against this release together; an older peer is refused at
the contract greeting.

- `StartRejection::BindingRefused` now carries a `BindingRefusal`
  reason. The former unit variant no longer composes or archives as
  the same value.
- `LaunchStatus` now answers a known request with `LaunchInspected`
  and a `LaunchInspection`: the durable attempt, its optional
  settled outcome, and the actually recorded `FlowNode`, if any.
  Unknown requests remain rejected. An outcome without an attempt
  is `InconsistentRecords`; a read spanning database markers is
  `ReadViewChanged`. This check does not claim one transaction.
- `Observe.Launch` sends the inspection on open and for each phase
  change, then ends with the inspection carrying the outcome.
- This release uses signal 8.0.0 at e0e3c055, which adds archived
  value identities without changing Signal's wire contract.

Consumers must pin signal-flow 11.0.0 and signal 8.0.0 e0e3c055
together, handle the new response and rejection variants, then
rebuild and restart their Nexus and CLI as one contract upgrade.

## 10.0.0: signal 8.0.0

What breaks:

- signal-flow depends on signal 8.0.0 (f35460de), where it depended on
  7.0.0. The re-exported `Signal`, `Signalizable`, `ByteViewable` and
  `Restorable`, and the `signal::Contracted` that `Query` implements, are
  signal 8.0.0's. A crate holding signal 7.0.0 alongside this release holds
  two signals whose traits do not meet.
- The `datom` feature enables `signal/datom` again: signal 8.0.0's `datom`
  binds datom-codec and protos 0.32.2, the same codec this contract binds, so
  the graph holds one codec (`cargo tree -d`).
- The build reads `ethos/signal.ethos` with ethos-zero 16.0.0 at c2653dd8,
  the rev signal 8.0.0 builds with. The generated module is byte-identical
  (the build script asserts it), so `ETHOS`, the contract digest, the datom
  text and the rkyv archive of every 9.0.0 value are unchanged.

To deploy, in each consumer (meta-signal-flow, signal-message,
meta-signal-message, flow, message): repin signal-flow to this release and
signal to 8.0.0 in the same change. The wire is unchanged, so a 9.0.0 peer
still greets this one; a running Nexus need not restart for this release
alone.

## 9.0.0: Report, Reported, Refused.UnknownFlow

What breaks:

- `Query` gains `Report.{ FlowId Event }` (`Query::Report(Report_Data)`), a
  harness event of one flow. `Response` gains `Reported` and
  `Refused.[ UnknownFlow.FlowId ]` (`Response::Refused(Refused_Data)`). The
  new type `Event.[ Started ToolUsed.String Stopped ]` names the event;
  `ToolUsed` carries the tool's name. Every match on `Query` or `Response`
  must handle the new variants.
- The reply payload type `Started.{ FlowId SessionId OriginClue }` is
  renamed `Launched`, because a bare `Started` in `Event` would otherwise
  name it and carry its fields. `Response::Started(Launched)` and
  `Replaced { flow_id, launched }` replace `Response::Started(Started)` and
  `Replaced { flow_id, started }`. Only Rust names change: the datom text and
  the rkyv archive of every 8.0.0 value are unchanged.
- `ethos/signal.ethos` is reprinted in the canonical vertical print
  (ethos-zero 16.0.0's `Printable`); its declarations other than the three
  above are unchanged. `ETHOS` changes, and with it the contract digest: a
  peer built from 8.0.0 is refused at the greeting with `ContractMismatch`.

What flow must implement:

1. Store each flow's events in its Memory, in the order received, on the
   record of the flow the FlowId names.
2. Answer a stored `Report` with `Reported`.
3. Refuse a `Report` whose FlowId Flow does not hold with
   `Refused.UnknownFlow.<FlowId>`; never adopt such a flow.

The hook: the harness's hook (SessionStart, PostToolUse, Stop) calls the
Flow CLI with one inline datom, `Report.{ <FlowId> Started }`,
`Report.{ <FlowId> ToolUsed.<tool_name> }` or `Report.{ <FlowId> Stopped }`.
It carries the FlowId from the environment Flow launched the flow with
(`FLOW_ID`), never from the harness's own session id.

To deploy, in each consumer (meta-signal-flow, flow):

1. Repin signal-flow to this release.
2. Handle the new variants; in flow, the three steps above.
3. Rebuild and restart the Nexus and both CLIs together; the digest
   changed, so an 8.0.0 client is refused.

## 8.0.0: ethos-zero 16.0.0, signal 7.0.0, protos and datom-codec 0.32.2

What breaks:

- signal-flow depends on signal 7.0.0 (the exchange layer), where it
  depended on 5.0.0. The re-exported `Signal`, `Signalizable`,
  `ByteViewable` and `Restorable` are signal 7.0.0's. `Query` implements
  `signal::Contracted` over `ETHOS`, so the contract's wire identity is the
  digest of `ethos/signal.ethos`; a peer greets with `Query::greeting()`.
- The `datom` feature pins protos 0.32.2 (15b41da8) and datom-codec 0.32.2
  (4dff16b4), where it pinned 0.31.0. It no longer enables `signal/datom`:
  nothing in this contract holds a signal type, and signal 7.0.0's `datom`
  feature pins datom-codec 0.31.0.
- protos 0.32's `textualize` prints vertically; the one-line inline datom
  a CLI reads and writes is `Compactable::compact`.
- The generated Rust is byte-identical to 7.1.0's (ethos-zero 16.0.0 changed
  only Library, Operation and Memory output). The ethos source is unchanged,
  and the vocabulary is 7.1.0's.

To deploy, in each consumer (meta-signal-flow, flow):

1. Repin signal-flow to this release and signal to 66e7b153; repin protos
   and datom-codec to 0.32.2 wherever the consumer names them directly.
2. Replace `.textualize()` with `.compact()` where one-line text is meant.
3. Rebuild and run the tests. The rkyv archive of every value is unchanged,
   so a Nexus and its clients need no store migration; rebuild and restart
   the Nexus and both CLIs together.
