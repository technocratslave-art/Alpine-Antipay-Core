Tenant Build (Documentation Only)

This directory intentionally contains no executable build scripts.

The Antipay core repository does not build Tenant operating systems.
It defines the contract that Tenant builders must follow.

Any scripts that appear here in the future are:

- reference implementations
- non-authoritative
- optional
- replaceable

---

Expected Tenant Artifacts

A Tenant is expected to produce, externally:

- A kernel image ("vmlinuz" or UKI)
- A minimal initramfs
- A root filesystem (immutable or image-based)
- A signed Tenant Manifest

These artifacts are consumed by the Floor, not built by it.

---

Handoff Requirements

A valid Tenant must support boot via:

- "kexec" or
- EFI handoff

The Tenant must not:

- probe hardware to determine platform identity
- fetch firmware at runtime
- modify Floor-owned state

All hardware information is provided via signed Floor facts.

---

Resource Boundaries

Tenants must declare and respect:

- memory budget
- CPU quota (if enforced)
- device access policy

The Tenant assumes it is resource-contained and untrusted.

---

Reference Implementations

Reference Tenants (e.g. Debian Slim, Fedora Slim) live in separate repositories:

- antipay-tenant-debian-slim
- antipay-tenant-fedora-slim

This repository only defines the rules they must obey.

---

Rationale

Keeping Tenant build logic out of the core repository prevents:

- distro creep
- accidental policy violations
- hardware responsibility leakage

The Floor governs.
The Tenant complies.
