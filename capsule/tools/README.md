Capsule Tools (Blacksmith)

This directory documents the host-side tools used to create Capsules.

Capsule tools do not run on the device.
They run during development, manufacturing, or provisioning.

Their purpose is to take hardware-specific chaos and turn it into a sealed, verifiable artifact that the Floor can trust.


What a Capsule Is

A Capsule is a signed hardware translation unit.

It may contain:

- firmware blobs (GPU, NPU, modem, Wi-Fi, etc.)
- device tree overlays or ACPI quirks
- power and thermal tuning data
- kernel module bundles (only when unavoidable)
- a signed hardware facts payload
- rollback protection metadata

A Capsule represents one device or one device family.


Role of Capsule Tools

Capsule tools exist to ensure that nothing malformed ever reaches the Floor.

They are responsible for:

- validating Capsule structure
- enforcing schema compliance
- hashing payload contents
- applying version floors (rollback protection)
- signing the final artifact

By the time a Capsule reaches a device, it must already be a slab of truth.


Trust Boundary

Capsule tools sit outside the trusted runtime.

- They are not security-critical at runtime.
- They may be replaced, rewritten, or reimplemented.
- Their output is what matters, not the tooling itself.

The Floor trusts only:

- cryptographic signatures
- monotonic versioning
- schema correctness


What This Directory Contains

This directory may contain:

- reference implementations of Capsule builders (e.g. "mk-capsule")
- schema validators
- signing helpers
- documentation describing Capsule assembly

These tools are:

- host-only
- offline
- deterministic where possible


What This Directory Must NOT Contain

This directory must never contain:

- runtime code executed on the device
- logic that assumes a specific OS or distro
- hard-coded private keys
- network-dependent build steps
- Capsule payloads themselves

Capsules belong in their own repositories or secure artifact stores.


Capsule Creation (Conceptual Flow)

1. Gather hardware-specific inputs (firmware, quirks, facts)
2. Validate inputs against schemas
3. Compute hashes for all payload components
4. Enforce rollback-protection rules
5. Generate a Capsule manifest
6. Sign the Capsule
7. Emit a sealed artifact

If any step fails, the Capsule is not created.


Rationale

Separating Capsule creation from the Floor ensures:

- hardware complexity never pollutes the boot layer
- future devices require no Floor changes
- downgrade attacks are mechanically prevented
- audits remain simple and finite

The Blacksmith shapes the metal.
The Floor only accepts finished blades.


Boundary Reminder

Capsule tools may evolve quickly.
Capsule formats must not.

If a change requires updating the Floor to accept it, the change is suspect.

Capsules translate hardware.
They do not define policy.
