Floor Build (Layer 0)

This directory documents how the Floor is built, not how it is customized.

The Floor is the immutable governance layer of the Antipay architecture.
Its build process is intentionally narrow, deterministic, and boring.

If you are looking to add features, drivers, or configuration options, you are in the wrong layer.


What the Floor Is

The Floor is a minimal Alpine-based initramfs containing:

- a statically linked Rust PID 1 (the Warden)
- the minimum userspace required to execute it
- no persistent state
- no networking
- no shells or scripting environments

Its lifetime ends the moment it performs a successful "kexec" or EFI handoff.


Build Outputs

A Floor build produces exactly one boot artifact:

- An initramfs image (optionally bundled as a UKI)

This artifact:

- boots first
- owns the hardware temporarily
- verifies Capsules and facts
- hands control to a Tenant
- then ceases to exist


Build Characteristics (Non-Negotiable)

All valid Floor builds must be:

- Statically linked (musl)
- Deterministic (reproducible with identical inputs)
- Minimal (no unused utilities or libraries)
- Offline (no network access during runtime)
- Fail-closed (verification failure halts the system)

If a build step violates any of these, it is not a Floor build.


What This Directory Contains

This directory may contain:

- documentation describing the build process
- reference build scripts
- CI configuration examples

Any scripts here are:

- non-authoritative
- replaceable
- reference-only

They exist to explain how a Floor can be built, not to mandate where or by whom it is built.


What This Directory Does NOT Contain

This directory must never contain:

- device-specific logic
- firmware blobs
- hardware detection code
- Tenant artifacts
- interactive tooling

All hardware specificity lives in Capsules.
All user experience lives in Tenants.


Typical Build Flow (Conceptual)

1. Assemble a minimal Alpine userspace (musl-based)
2. Compile the Rust Warden as a static binary
3. Place the Warden at "/init"
4. Package the initramfs
5. (Optional) Bundle kernel + initramfs as a UKI

The exact host OS, CI system, or toolchain used is irrelevant as long as the output satisfies the constraints above.


Rationale

Keeping the Floor build process small and explicit ensures:

- auditability
- fast boot
- minimal attack surface
- long-term maintainability

The Floor is not meant to evolve quickly.
It is meant to be trusted.


Boundary Reminder

The Floor governs hardware.
It does not serve users.

If a proposed change makes the Floor more convenient, more dynamic, or more flexible, it is probably wrong.

The Floor stands.
