# AetherOS — Perceptual Runtime Refinement Prototype

This codebase assumes core OS fundamentals exist and focuses on **perceptual performance**, **micro-interaction consistency**, **cognitive load control**, and **trustworthy intelligence behavior**.

## What is implemented in this iteration

### 1) Interaction latency policy enforcement
- Central rulebook with explicit thresholds:
  - input acknowledgment target: 8–16ms
  - perceived response target: 50ms
  - progressive feedback trigger: 120ms
  - cancellation trigger: 400ms
- Kernel exposes `interaction_status` to evaluate operation behavior against these targets.

### 2) Motion, grid, and integrity governance
- One global rulebook for motion duration range (120–240ms), grid unit (8pt), and typography limits.
- Motion complexity scaling is reduced under load (`full`, `moderate`, `reduced`) to prioritize smoothness.
- Kernel exposes `design_integrity` summary to explain these constraints transparently.

### 3) System intelligence layer with restraint
- Context engine tracks foreground app, recent actions, and friction hotspots.
- Suggestions are suppressed below confidence threshold or after being ignored twice.
- Assistant is capability-transparent and avoids pretending unsupported behavior.

### 4) Notification discipline
- Low-priority alerts are batched into digest queues.
- Digest can be flushed on demand.
- Notification history auto-expires non-critical backlog.

### 5) Assistant personality constraints
- Default tone remains neutral and concise.
- Optional dry humor appears only in low-risk, high-confidence contexts.
- Humor is suppressed automatically for critical/uncertain states.

## Existing functional runtime surfaces retained
- App install/list/launch with compatibility adapters
- File list/read/write/search
- Terminal bridge
- Desktop primitives (dock, windows, workspaces, mission control)
- Settings controls
- Performance profiles
- Continuity primitives (AirShare/Handoff/Dynamic Orb)

## Run

```bash
cargo run
```

## Scope honesty
This remains a high-fidelity prototype runtime, not a production-ready replacement for Windows/macOS. The implemented layer focuses on interaction quality, clarity, and behavior policy enforcement rather than kernel/driver replacement.


## ISO status (important)

You can now generate an **installer ISO artifact** via GitHub Actions (`Build Installer ISO Artifact`) or locally with:

```bash
cargo build --release
bash scripts/make_installer_iso.sh
```

This ISO currently packages the AetherOS prototype binary + installer helper script for an existing Linux system.
It is **not yet a bare-metal bootable replacement OS image** with its own kernel/bootloader/driver stack.


## Bootloader + kernel + driver images (new)

Added missing build components for a bootable prototype image:

- `boot/grub/grub.cfg` — GRUB entry for AetherOS kernel + initramfs
- `initramfs/init` — early userspace init that launches `aether_os`
- `scripts/build_kernel_bundle.sh` — collects Linux kernel + builds initramfs with `aether_os`
- `scripts/build_driver_image.sh` — packages `/lib/modules/<release>` as compressed driver image
- `scripts/build_bootable_iso.sh` — assembles GRUB bootable ISO (`dist/AetherOS-bootable.iso`)
- `.github/workflows/bootable-iso.yml` — CI workflow to build and upload bootable ISO artifact

Build locally (when host has required tooling):

```bash
cargo build --release
bash scripts/build_bootable_iso.sh
```

Note: this is now a **bootable Linux-kernel-based prototype image path**. It is still not a fully independent custom kernel+driver OS distribution yet.


## Bare-metal readiness status

Can it run directly on hardware now?
- **Prototype-level yes**: a GRUB + Linux-kernel + initramfs bootable path is present for direct boot testing.
- **Production-level no**: it is not yet a full independent OS distribution with a custom kernel, full installer partitioning flow, hardware certification, and broad driver QA.

## BootGuard recovery/debug companion

A parallel boot helper now runs from initramfs:
- `initramfs/boot_guard.sh` runs during boot
- attempts automatic fixes for common early-boot issues (missing shell link, init executable bit, missing mount points)
- writes logs to:
  - `/var/log/aetheros/bootguard.log`
  - `/var/log/aetheros/bootguard-history.log`

Runtime diagnostics are also recorded in `/tmp/aetheros-diagnostics.log` and exposed through assistant command `diagnostics`.
