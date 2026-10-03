# Upgrades

How to deploy each breaking change of signal-flow.

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
