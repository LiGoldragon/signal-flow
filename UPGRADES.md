# Upgrades

How to deploy each breaking change of signal-flow.

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
