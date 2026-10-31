# SigmaOS-desktop-session

Component boundary for graphical-session lifecycle: compositor selection and
startup, session environment, readiness reporting, orderly shutdown, and
failure recovery. The component manifest assigns canonical ownership to
`SigmaOS-shell` and names the Wayland compositor capability.

## Current state

`COMPONENT.toml` marks this repository as a scaffold. The tracked checkout
currently contains its manifest, license, README, and a metadata validation
workflow; it does not yet implement a session manager or publish a stable API.

## Intended boundary

The session layer should own transitions from session start through compositor
readiness and shutdown. Desktop applications should consume the established
session environment without controlling compositor lifecycle. Device-specific
setup and application policy belong in their respective components. This
repository should expose lifecycle state and failure information through a
documented contract before other components depend on it.

## Readiness evidence

Before describing this component as implemented, add the lifecycle contract,
the executable source, focused transition and failure-path checks, and an
integration example in the SigmaOS shell. The metadata validation workflow
checks repository declarations; it does not establish runtime readiness.
