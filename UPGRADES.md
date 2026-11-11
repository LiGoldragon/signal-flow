# Upgrades

## 60.0.0: Living, routed answers, Remind, Answer, a Report's birth

Built on 59.0.0 (d2a4ca5), for Flow 0.54.3, on flow-ethos 0.54.3
(`062ef9e1`).

- The living's words are `Living.{ Context Verbatim }`: `SendableRequest`
  and `VisibleRequest` carry `Living.{ Context Verbatim }` and
  `DeliveredRequest` `Living.{ RequestId Context Verbatim }`, in Psyche's
  place; `Psyches` is gone from all three, so the variants after it move
  up one. `Said` is gone; `Context` and `Verbatim` are re-exported from
  flow-ethos. `Send.{ { Psyche core Fable } Soft.Living.{ «the drain»
  «Fix it today.» } }`.
- `Report.{ FlowId Option<Process> Event }`: the birth of the harness that
  ran the hook, `None` when it found none. A hook built on 59.0.0 is
  answered FrameRefused.
- `Queue.{ Address Vector<VisibleNativeInput> }`: each queued message with
  its sender and priority, as it would land in the prompt.
- `Query` gains, last, `Remind.{ Address Minutes String }` and
  `Answer.RequestId`; `Response` gains, last, `Reminded.RequestId`,
  `Answered.VisibleNativeInput`, `AnswerRejected.AnswerRejection` and
  `RestartQueued.FlowId`, with `AnswerRejection.[ UnknownRequest Open
  Routed StoreRefused ]`. `RestartRefusal` gains `Asleep.FlowId`, last.
  The names are tentative.
- A Result or Done is routed by the Nexus to the sender of the request it
  names; the Address its Send writes decides nothing. The notes on
  Refresh, Restart and Launch say what Flow 0.54.3 does.
- A consumer repins flow-ethos to `062ef9e1`.

## 59.0.0: every caller is answered alike; queue and request operations

Built on 58.0.2 (15790be), for Flow 0.54.2. Every query gives the same
answer to every caller; the calling process only fills in a message's
sender.

- `VisibleSender` gains `Unbound`, last: a sender in no flow. A
  consumer reading a snapshot of such a send needs this revision.
- `BindSeat.{ Address SeatPane }` and `Adopt.{ Address SeatPane
  NativeSessionId }` name the Herdr pane whose seat they bind, as
  `BindSeat.{ { Field core Sol } { main w1:p3 } }`; `BindSeat.Address` no
  longer reads.
- `Protected` leaves `RefreshRejection`, `StopRejection`,
  `RestartRefusal` and `ReplaceRejection`; `Caller` leaves
  `ArchiveRejection`; `UnknownCaller` leaves `HandoverRefusal`. Each was
  the last variant but `Caller` and `UnknownCaller`, whose types are
  answers only, never stored.
- `Query` gains, last: `Cancel.RequestId`, `Silence.RequestId`,
  `Clear.Address`, `ClearAll`, `Push.{ Address Priority }`, `Queues`,
  `Hot`. `Response` gains, last: `Cancelled.RequestId`,
  `CancelRejected.RequestRejection`, `Silenced.RequestId`,
  `SilenceRejected.RequestRejection`, `Cleared.Vector<RequestId>`,
  `Pushed.Vector<RequestId>`, `Queues.Vector<Queue>`, `Hot.Vector<Warmth>`,
  with `RequestRejection.[ UnknownRequest NotYet Closed NotTask
  StoreRefused ]`, `Priority.[ HardAbrupt MiddleAbrupt Soft ]`,
  `Queue.{ Address Vector<DeliveredRequest> }` and
  `Warmth.{ Address Option<Minutes> }`. The names are tentative.
- Every other variant keeps its place, so a peer on 58.0.2 that sends
  none of the changed queries (Message's Bind, Identify, IdentifyThread,
  Lock, Deliver, Release, Acknowledge and its observations; the Energy
  Nexus's `Observe.TurnEnds`) still reads and is read.
- `NotMessage` and `NotSendServer` stay in `Refusal`, answered no longer,
  for the outcomes archived with them. The flow-ethos pin stays at
  5c61d666; Flow 0.54.2 builds this contract against its own flow-ethos,
  whose `SentBy` gains `Unbound`.

## 58.0.2: flow-ethos 5c61d666 (0.54.0)

Only the flow-ethos pin moves, from d1effd4b (0.53.3) to flow main 5c61d666
(0.54.0), whose flow-ethos source is unchanged. The contract and every datom
form are unchanged. A consumer repins flow-ethos to 5c61d666.

## 58.0.1: signal 6d4eb74 (11.0.0), on build-base d5ad91d

Only the signal pin moves, from d77169d (10.4.1) to 6d4eb74 (11.0.0): signal
carries no transport feature and no tokio, and datom is unchanged. The
contract and every datom form are unchanged. A consumer repins signal to
6d4eb74.

## 58.0.0: ethos-zero 1976e45, flow-ethos d1effd4b, datom-codec 09a0526, protos c93b598, signal d77169d

Built on 57.0.0. Every datom text form is unchanged.

- ethos-zero 22 makes each one-type head a newtype: FlowId, Title,
  NativeSessionId and the other scalar heads are tuple structs, so a consumer
  wraps a value with the constructor and reads it with `.0`.
- Recipient and Sender are gone; a Lock holds two Addresses and Lock.{ Address
  Address } is the Lock input; SendRequest holds an Address. Caller is renamed
  CallerIdentity. VisibleSendReceipt, StopRequest, RecipientResolutionRequest
  and the other heads over a single named type are that type.
- The pins are datom-codec 09a0526, protos c93b598, ethos-zero 1976e45,
  signal d77169d and flow-ethos d1effd4b, so one source of each sits in a
  consumer's graph.
- Built on build-base d7ce205: one derivation per crate from the committed
  Cargo.nix.

How to deploy each breaking change of signal-flow.

## 57.0.0: a Stop carries its background work; a refresh waits for rest

Built on 56.0.0 (be85c63).

- `NativeStop` gains `BackgroundWork.Vector<BackgroundTask>`, last:
  `BackgroundTask.{ BackgroundTaskId BackgroundTaskKind BackgroundTaskStatus
  Description }`, `BackgroundTaskKind.[ Shell Subagent Monitor Workflow
  Teammate CloudSession McpTask Other.String ]`, the work Claude Code's Stop
  hook names in `background_tasks`, as in `Stopped.{ Some.s1 Some./p/s1.jsonl
  None [ { m1 Monitor running watch } ] }`. Empty: nothing in flight.
- A Stop without it (`Stopped.{ None None None }`, 56.0.0's shape) no longer
  reads.
- `Response` gains `RefreshQueued.RefreshQueued`, last, with
  `RefreshQueued.{ Address FlowId }`: a Refresh of a working flow, queued
  for its idle event. A Launch or Replace of a working flow answers
  `LaunchPending` and finishes through `Observe.Launch`.
- A frame that decodes as no Query, an earlier hook's Report among them,
  is answered `FrameRefused.UndecodableFrame` and recorded, with
  `UndecodableFrame.{ Option<FlowId> SeatEvidence }`: the flow bound to the
  caller's seat, if any, and the caller's seat and process.
  `ObserveSelection` gains `Undecodable`, last: `Observe.Undecodable` answers
  `UndecodableObserved.Vector<UndecodableFrame>`, every record on open, then
  each new one. `Response` gains both, after `RefreshQueued`.
- `ObserveSelection` gains `TurnEnds`, after `Undecodable`:
  `Observe.TurnEnds` answers `TurnEndsObserved.Vector<TurnEnd>` on open, the
  last turn end of each harness, then `TurnEnded.TurnEnd` for each turn end,
  with `TurnEnd.{ Address HarnessKind FlowId ObservedAt }`, as in
  `TurnEnded.{ { Psyche core Opus } Claude 7d41e0 1761800000 }`. `Response`
  gains both, after `UndecodableObserved`.
- `ObserveSelection` gains `Metaflows`, after `TurnEnds`:
  `Observe.Metaflows` answers `MetaflowsObserved.Vector<Metaflow>` on open,
  the metaflows as `Metaflows` lists them, then at each change
  `MetaflowChanged.Metaflow` for a metaflow as it now stands and
  `MetaflowGone.Address` for one no longer listed, as in
  `MetaflowChanged.{ { Psyche flow Fable } Asleep [ f1c841 ] [] }`.
  `Response` gains all three, after `TurnEnded`. `Metaflows` stays the
  one-shot read.
- Every producer and consumer moves to 57.0.0 together: the hook that
  writes a Stop, the Nexus that stores it, and a match on `Response`.

## 52.0.0: a handover that cannot be read refuses the launch

Built on 51.0.0.

- `CompositionRefusal` gains `HandoverUnreadable.{ FlowId HandoverFault }`,
  last: the launch names a predecessor whose handover could not be read
  whole, as in `StartRejected.CompositionRefused.HandoverUnreadable.{ abc123
  NotUtf8 }`.
- `HandoverFault.[ InvalidPredecessor RootUnresolved NotUtf8
  Unreadable.String ]`: the predecessor names no flow, the launch root is
  unresolved, the file is not UTF-8, or it exists and could not be read,
  with the reader's text.
- A rejection 51.0.0 archived reads the same: the variant is appended, and
  a `StartRejection` holding it is no larger (`tests/archive.rs`).
- Every producer and consumer moves to 52.0.0 together: a match on
  `CompositionRefusal` names the new variant.

## 51.0.0: a composer's rejection carries its text

Built on 50.0.0.

- `ContextRefusal.ComposerRejected` is `ComposerRejected.String`: the text
  Curriculum answered `Rejected` with, kept whole, as in
  `StartRejected.ContextRefused.ComposerRejected.«missing module
  knowledge:absent»`.
- Every producer and consumer moves to 51.0.0 together: a match on
  `ComposerRejected` binds its text.

## 50.0.0: protos 0.33, datom-codec 0.33, signal 10.3

Built on 49.0.0. The contract is unchanged; its stack moves.

- `signal` 10.3.0 (7afb68b), `datom-codec` 0.33.0 and `protos` 0.33.0, the
  last two pinned by the revs signal names (`eabead4`, `3c7ade3`) so a build
  holds one of each; `flow-ethos` at 5ea2aa4, on the same stack. The
  generator stays ethos-zero 17.0.0 (07714b0).
- A guillemet string is read by balance: `«Title «The Belief Nexus made
  small»»` is one string, its inner pair content; an unbalanced guillemet
  inside one is escaped, `«an unbalanced \» inside»`.
- `ReaderBudget` is built by `ReaderBudget::try_from(n)`.
- A bare String prints bare: a brief `«Begin.»` reads as before and prints
  `Begin.`.
- Every producer and consumer moves to 50.0.0 together, on this stack.

## 49.0.0: a protected Address answers only its own seat

Built on 46.0.0; 47.0.0 and 48.0.0 are other lines.

- `RestartRefusal` gains `Protected.FlowId`; `RefreshRejection`,
  `ReplaceRejection` and `StopRejection` gain `Protected`. A Restart,
  Refresh, Replace or Launch of an Address the meta `ProtectedAddresses`
  names is answered `Protected` unless its own seat asks; its Stop also
  from the owner.
- Every producer and consumer moves to 49.0.0 together: an exhaustive match
  on any of the four gains the variant.

## 46.0.0: a live flow restarts as its own session

- `Restart.FlowId` closes a Claude flow's harness, Awake and resting, and
  resumes its own native session in a new pane: `Restarted.FlowId`, or
  `RestartRefused.RestartRefusal`, one of `UnknownFlow`, `NoHeldSession`,
  `TranscriptAbsent`, `Busy`, `CloseRefused`, `ResumeRefused.{ FlowId
  String }`, `Interrupted` and `Unsupported`, each naming the flow.
- `StopRejection` gains `Restarting` (a restart step runs for the flow) and
  `SpawnUnproven` (an interrupted restart may have started a harness Flow
  cannot show stopped).
- Every producer and consumer moves to 46.0.0 together.

## 45.0.0: a recipient is an Address; no model ranks above another

- A `Recipient` is the `Address` it names: `Send.{ { Mind core Sol }
  Soft.Order.«…» }`. `Up` leaves the contract; no route is inferred from the
  sender's model.
- `PowerLevel` leaves the contract, and with it the power field of `Caller`:
  `Caller.{ FlowId Aspect ModelName }`.
- `NoSecondary`, `NoneAbove` and `NoLayer` leave the contract: no Start waits
  for another model's seat, and no Send is refused for having nothing above
  its sender.
- A task's last reminder sends its failure to the task's sender.
- Pins flow-ethos 9644e01. Every producer and consumer moves to 45.0.0
  together.

## 44.0.0: an address names its model

- `Address` is `{ Aspect Topic Model }`, `Model.[ Fable Opus Sonnet Haiku
  Astra Sol Luna Terra ]`, from flow-ethos; `Layer` leaves the contract. Any
  model stands in any aspect.
- `Up` resolves to the model before the sender's in its family: Opus to
  Fable, Sonnet to Opus, Haiku to Sonnet, Sol to Astra, Luna to Sol, Terra
  to Luna. Fable and Astra have none above: `NoneAbove`.
- `NoSecondary` now refuses a Fable or Astra whose Opus or Sol of its aspect
  and topic is not Awake. `NoLayer` is kept for archived refusals.
- Every producer and consumer moves to 44.0.0 together.

## 42.0.0: an existing running seat adopts itself

- `Adopt.{ Address NativeSessionId }` binds the authenticated calling seat to
  the address only when its observed native session is the supplied session.
  It answers `Adopted.FlowId`; refusals use the existing typed `Refused` reply.
- The request carries no FlowId. Flow derives it from the verified native
  session and retains every `BindSeat` ownership and duplicate-binding check.
- `BindSeat` and every 41.0.0 operation remain unchanged.

## 41.0.0: message priority is the delivery policy

- `Send` is `Send.{ Recipient Message }`. The sender supplies the recipient
  and typed message; the message priority determines immediate delivery,
  interruption, queueing and waking.
- The private `Deliver` query is `Deliver.{ Lock Message }` for the same
  reason. The sender-selected `Delivery.[ Queue Wake ]` policy leaves both
  wires and the current signal-flow API.
- This changes the ordinary Query archive. Every producer and consumer moves
  to 41.0.0 together; there is no compatibility path.

## 38.0.0: lifecycle replies retain their run identities

- Lifecycle replies again carry their useful Flow ids. Version 37 removed
  them too broadly while projecting message-ledger ids away.
- A recipient-visible `VisibleNativeInput` carries its sender, priority and
  request. Its recipient is routing state inside the Nexus, not envelope
  data repeated to that recipient.

## 37.0.0: lifecycle replies projected run identities away

- This release removed Flow ids from lifecycle replies. Version 38 restores
  them while retaining the message and delivery projections introduced by
  version 36.

## 36.0.0: public replies and messages carry metaflows, not ledger keys

- `Send` takes the typed public `VisibleMessage`; `Result` and `Done` name
  the complete typed `Order` or `Task` value they answer. The Nexus derives
  the sender from the authenticated caller and keeps delivery and run ids in
  its private ledger.
- Send admission, observation, drain, receipt and failure responses retain
  their state and typed reason without exposing the private logical-delivery
  key.
- `VisibleNativeInput` carries the sender and recipient metaflow addresses
  and the typed message, without a delivery id. The private `QueueEntry`,
  `NativeInput`, response and delivery types remain available to the Nexus
  for its existing stored rows; this contract change does not rewrite them.
- Major: ordinary Query and Response archives change shape. Consumers pin
  signal-flow 36.0.0 and update every producer and consumer together.

## 35.0.0: Unidentified carries the raw evidence of the seat

- `Refusal.Unidentified` carries `SeatEvidence` where it carried `Process`:
  `SeatEvidence.{ Option<SeatPane> Option<PaneLabel> Option<NativeSessionId>
  ObservedAt Process }`, with `SeatPane.{ HerdrSessionName HerdrPaneId }`,
  `PaneLabel.String` and `ObservedAt.Integer` (seconds since the Unix
  epoch). A caller Flow cannot name stays unidentified; the refusal reports
  the pane it ran in, the label Herdr shows for it, the native session read
  there (if any), when it was read, and its process, and invents no identity.
  The answer is `Refused.Unidentified.{ Some.{ herdr-1 w1-2 } Some.«Claude
  research» Some.01a0fdcc-c1a1 1761800000 { 4242 987654 } }`; a caller in no
  pane Flow reads is `Refused.Unidentified.{ None None None 1761800000 { 4242
  987654 } }`.
- Major: the payload changes the shape of every `Refusal`, in its datom and
  in its rkyv archive, so an archive of a `DeliveryState`, `SendAcceptance`,
  `SendFailure`, `Response` or any record holding a `Refusal` written by
  34.0.0 reads differently. A consumer holding such archives reads them as
  34.0.0 wrote them and upgrades `Unidentified.Process` to a
  `SeatEvidence` with no pane, no label, no session, observed at 0 and that
  process. Consumers pin signal-flow 35.0.0; no compatibility path.

## 34.0.0: variants leave; the version marks the break

- Variants were removed from the contract; this release carries the
  version that marks it. Consumers pin signal-flow 34.0.0; no
  compatibility path.

## 33.3.0: Remind leaves the ordinary socket; a Task with its Minutes replaces it

- `Query` loses `Remind.{ Instant Body }`, `Response` loses `Reminded`,
  and `Refusal` loses `InstantUnreadable.Instant`; the `Instant` and `Body`
  types go with them. A reminder is a `Task` (flow-ethos `Request`) with
  its Minutes: its sender is reminded when the Minutes lapse with no Done.
  `NoOpenTask` and `MinutesRefused` move up one discriminant, so an archive
  holding a `Refusal` written by 33.1 or 33.2 reads differently; no
  released Flow stores one. Consumers drop the arms; no compatibility path.

## 33.2.0: Flow refuses a Task's Minutes at admission; flow-ethos carries the ruled message contract

- `Refusal` gains `MinutesRefused.MinutesRefusal`, appended, with
  `MinutesRefusal.[ Zero Negative Unreachable ]`: a `Task` (flow-ethos
  `Request`) whose Minutes is zero, below zero, or gives a deadline the
  clock cannot hold is refused at admission. Minutes 0 no longer means
  "no reminder". Existing archives keep their discriminants; a consumer
  matching `Refusal` exhaustively adds the arm.
- flow-ethos is pinned at flow 51e01e9, which adds the ruled message
  contract as `flow_ethos::message`. Every type this contract imports is
  unchanged.

## 33.1.0: Remind on the ordinary socket; tasks close by Done

- `Query` gains `Remind.{ Instant Body }`: at `Instant`, an RFC 3339
  date-time, Flow queues `Soft.Notice.Body` for the caller's metaflow with
  `Wake`, held in Flow's memory until then. `Response` gains `Reminded`;
  `Refusal` gains `InstantUnreadable.Instant` and `NoOpenTask.LogicalDeliveryId`,
  the refusal of a `Done` (flow-ethos `Request`) naming no open task.
  Every variant is appended,
  so existing archives keep their discriminants; a consumer matching
  `Query`, `Response` or `Refusal` exhaustively adds the arms.

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
# 40.0.0: launch rehearsal and missing-module identity compose

The launch rehearsal contract and typed missing-module context refusal are one contract. Every consumer moves to this version together.

# 39.0.0: one declared launch request can be rehearsed

- `LaunchRequest.{ Address Remembering InstructionPrompt }` is the named
  request carried by both `Launch` and the new `Rehearse` query.
- `Rehearsed.Command.String` answers the fully shell-quoted final native
  invocation. `RehearseRejected.StartRejection` changes nothing.
- malformed configured harness flags are a typed
  `StartRejection.MalformedHarnessFlags.HarnessFlagsRefusal`.

All ordinary clients and Flow must move together because Query, Response and
the contract digest changed.
# 55.0.0: observe handover retirement

`Observe.Retirement.Address` opens the ordinary Observe subscription with a
snapshot of the Address's closed handover generation identities. Each later
close, verified archive commit and exact removal is pushed in FIFO order.
`RetirementObservationFailed` ends an observation whose evidence became
incomplete. Archive evidence names the actual Git commit, document path and
blob. This observation is ephemeral and adds no durable history.
