# Alpine-Antipay-Core
alpine-antipay-core is a sovereign boot system, not a distro. It splits hardware truth from user experience using a minimal Alpine Floor, signed hardware Capsules, and a strict Tenant contract. The Floor verifies reality, stages hardware, then disappears, enabling small, auditable, modular Linux systems across phones, tablets, laptops, and desktops

alpine-antipay-core

A sovereign boot system, not a distribution.

Scope and Intent

alpine-antipay-core exists to solve one specific, chronic failure in modern Linux systems:
the operating system is forced to guess what hardware it is running on.

That single mistake is what causes:

- bloated kernels
- endless device probing
- fragile mobile ports
- vendor lock-in
- un-auditable boot paths

This project fixes that by splitting responsibility into hard layers and enforcing them mechanically.

This repository does not build a distro, UI, or app ecosystem.
It builds the constitutional machinery that makes those things clean, small, and replaceable.

The Core Idea (Why This Works)

Traditional OS design mixes three concerns into one blob:

1. Hardware discovery
2. Policy and security
3. User experience

Antipay separates them permanently.

Hardware truth is established once, early, and cryptographically.
After that, nothing guesses. Nothing probes. Nothing “figures it out later.”

The result is a system that is:

- smaller
- faster to boot
- easier to port
- auditable by humans
- enforceable by machines


The Layer Model (Head → Tail)

This system is composed of five strict layers. Each layer has authority only over its domain.

Layer 0 — The Floor (Alpine Initramfs + Warden)

The Floor is the only layer that touches raw hardware.

It:

- boots first
- mounts the minimum needed to run
- verifies signatures
- applies hardware quirks
- stages firmware
- emits signed hardware facts
- kexecs into the Tenant and disappears

The Floor is built on Alpine because:

- musl enables fully static binaries
- initramfs stays under ~10MB
- attack surface is minimal
- behavior is deterministic

The Floor has no network, no shell, no services, and no persistence.
If verification fails, it halts. Panic is success.


Layer 1 — Capsules (Hardware Modules)

Capsules are signed hardware translation units.

They contain:

- firmware blobs (GPU, NPU, modem, etc.)
- device tree overlays or ACPI quirks
- power and thermal tuning
- kernel module bundles (only if unavoidable)
- a signed facts payload describing the machine
- rollback protection (minimum allowed version)

Capsules are swappable.
If you move from one device to another, you swap Capsules — not the OS.

This is what makes the system future-proof and portable.


Layer 2 — Tenant Base (Slim OS Substrate)

The Tenant Base is the smallest possible “real OS.”

It includes:

- one init system
- minimal userspace
- security policy (LSM, seccomp, cgroups v2)
- atomic update + rollback
- a profile selector that reads Floor facts

It does not:

- probe hardware
- fetch firmware
- guess the platform
- carry drivers for devices it doesn’t own

The Tenant assumes the Floor already made hardware boring.


Layer 3 — Form-Factor UI Profiles (Software Modules)

Phone, tablet, laptop, and desktop differences live here — not in the OS.

Profiles are declarative bundles such as:

- ui-profile-phone
- ui-profile-tablet
- ui-profile-laptop
- ui-profile-desktop

Each profile defines:

- compositor / session
- input expectations (touch, pen, keyboard, trackpad)
- power policy
- windowing behavior
- on-screen keyboard rules

Profiles are selected only from verified Floor facts.
No probing. No heuristics.


Layer 4 — Apps and Workloads (Packages & Containers)

This is where software lives:

- Desktop apps via Flatpak
- Dev tools and AI stacks via OCI containers
- Explicit system services only

Nothing installs into the base OS unless it is infrastructure.


Hardware vs Software (Clear Line)

- Hardware modules → Capsules (Layer 1)
- Operating substrate → Tenant Base (Layer 2)
- Form-factor behavior → UI Profiles (Layer 3)
- User software → Apps & Containers (Layer 4)

This line is enforced by structure, not convention.


Repository Layout (How This Is Implemented)

alpine-antipay-core/
├── README.md
├── LICENSE
│
├── docs/                    # Constitutional documents
│   ├── rfc-001-separation-of-state.md
│   ├── floor-boot-flow.md
│   ├── tenant-contract.md
│   └── threat-model.md
│
├── floor/                   # Layer 0: The Floor
│   ├── initramfs/
│   │   ├── init              # Statically linked Rust PID 1
│   │   └── etc/
│   │
│   ├── warden/               # antipay-floor (Rust)
│   │   └── src/
│   │
│   └── build/
│       ├── mk-initramfs.sh
│       └── mk-uki.sh
│
├── capsule/                  # Layer 1: Hardware Capsules
│   ├── spec/
│   │   ├── capsule.toml.schema
│   │   ├── facts.cbor.schema
│   │   └── rollback-protection.json
│   │
│   └── tools/
│       └── mk-capsule/
│           └── src/
│
├── tenant/                   # Layer 2 contract (not an OS)
│   ├── manifest/
│   │   ├── tenant-manifest.schema.yaml
│   │   └── examples/
│   │
│   └── handoff/
│       ├── cmdline.md
│       ├── efi-vars.md
│       └── initrd-file.md
│
├── exec/                     # Optional secure command runner
│   └── src/
│
└── ci/
    ├── build-floor.yml
    ├── lint-capsules.yml
    └── lint-manifests.yml

Actual Tenants, UI profiles, and hardware Capsules live in separate repositories.
This repo defines the law, not the inhabitants.


What This Project Is Not

- Not a mobile distro
- Not Android replacement
- Not a desktop environment
- Not a driver collection

Those are consumers of this system, not part of it.


Threat Model (Short)

- Assume Tenant compromise is possible
- Assume physical access is possible
- Assume downgrade attacks are attempted
- Trust only signed artifacts and monotonic versioning
- Hardware authority ends with the Floor
  

Wrap-Up

This architecture works because it refuses to be clever.

- The Floor makes hardware boring.
- Capsules isolate vendor chaos.
- Tenants stay small and auditable.
- UI becomes modular.
- Apps stay contained.

Nothing guesses. Nothing sprawls. Nothing rots silently.

The Floor is the slab.
The Capsule is the translator.
The Tenant is the home.

Infrastructure begins here.
