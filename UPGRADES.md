# UPGRADES

## 3.0.0 → 4.0.0 — ordinary delivery escalation contract

### What breaks

`Query` adds `ResolveDelivery` and `Response` adds `DeliveryResolved`, so the
rkyv enum discriminants and the closed Datom vocabularies change. Version 3
peers must not exchange `Query` or `Response` archives with version 4 peers.

The v3 `ResolveRecipient` query, `RecipientDisposition` response, and refresh
contract remain present and unchanged. Version 4 adds a delivery-resolution
request with a caller-supplied hop limit and an opaque stable delivery
idempotency key. Its response carries an ordered routing trace and exactly one
of these caller dispositions:

- a selected Ready recipient in the requested Flow's aspect;
- a retryable hold for lifecycle transition or a crucial Flow start already
  pending;
- a terminal refusal for a cross-aspect candidate, no eligible same-aspect
  recipient, exhausted hop limit, refused start, or detected cycle.

A Medium or High Flow start is represented by Flow's existing
`LaunchRequestId`. Consumers must reuse that identifier when retrying and must
not create Message-owned launch state. The wire does not supply a default hop
limit; the caller must obtain policy and put the chosen value in every request.

### Deploying

This repository is a contract and has no runtime to deploy. Repin the Flow
producer and every direct decoder together, rebuild them against version 4,
then switch them in one attended cutover. Producers must record trace entries
in evaluation order: the requested Flow first, higher powers in the same
aspect before lower powers, and no selected cross-aspect candidate. A
`StartPending` or lifecycle hold is retryable; `NoEligibleSameAspect` is
terminal.

Do not activate a producer that emits version 4 until all live consumers can
decode it. Roll back by restoring the complete version 3 producer/consumer
set pinned to `fa326ac00b47e3e35fb590852c33c672d9e65bb6`; mixed v3/v4 operation is
unsupported. Because this contract-only change cannot affect network or
remote access by itself, no countdown rollback is armed here. Any repository
that deploys a runtime consumer must record its own rollback timeout,
cancellation authority, and network/remote-access witness before activation.
