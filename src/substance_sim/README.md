# Substance Envelope Simulation Framework — `substance_sim`

## Overview

This crate implements the **Substance-EnvelopeSpec**, a formally specified, diagnostic-only biophysical simulation module for educational use within the NewRow-Print!/Tree-of-Life sovereignty stack. It models the neurochemical and behavioral effects of substances (caffeine, nicotine, THC) using pharmacokinetic/pharmacodynamic (PK/PD) and neurotransmitter dynamics, and maps their outputs to the **Tree-of-Life axes**: `DECAY`, `LIFEFORCE`, `POWER`, `FEAR`, `PAIN`.

### ✅ Governance Compliance

- **CapabilityTier**: `CapModelOnly`
- **Actuation**: `none`
- **RoH Bound**: `0.3` (monotonic, never exceeds)
- **Non-Reward**: `true`
- **EducationOnly**: `true`
- **AutodowngradeEnabled**: `false`
- **No CapabilityState / ReversalConditions / Consent Hooks**
- **All outputs are read-only logs or visualizations**
- **No coupling to live biosignals, bio-tokens, or cybernetic actuators**

### 📦 Modules

- `substance_envelope.rs` — Core data structure and mapping logic
- `pkpd_engine.rs` — (Next phase) ODE-based plasma concentration models
- `behavioral_model.rs` — (Next phase) Rule-based craving/impulse models

### 🔬 Usage

```rust
let envelope = SubstanceEnvelopeSpec::new(
    4.2, // caffeine mg/L
    0.2, // nicotine mg/L
    80.0, // THC ng/mL
    0.18, // D2 occupancy
    0.35, // CB1 occupancy
    70.0, // adenosine antagonism %
    1.1,  // ΔDA in NAcc
    -0.05, // Δ5-HT in PFC
    -0.15, // ΔGABA in Amygdala
    0.35, // ΔGlu in Hippocampus
)?;

let snapshot = envelope.to_biophysical_snapshot();
// Render in TreeOfLifeView HUD as virtual axis overlay
