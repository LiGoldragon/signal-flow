# Upgrades

How to deploy each breaking change of signal-flow.

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
