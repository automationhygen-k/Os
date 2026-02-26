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
