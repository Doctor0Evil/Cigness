# Cigness – Architecture Overview (Phoenix v1)

## 1. Core concepts

- **R-Nodes (Relief / XR Calm Hubs)**  
  Provide tobacco-free XR calm, focus, and eco-linked experiences for youth and adults.

- **C-Nodes (Collection & Recycling Hubs)**  
  Collect cigarette butts, plastics, and devices; pre-process materials for circular use.

- **M-Nodes (Micro-Factories)**  
  Turn recycled feedstock into calm-break device shells, reusable crates, and emergency water containers.

- **P-Nodes (Policy Anchors)**  
  Encode zoning, tobacco retail density, and smoke-free zones; prioritize Cigness infrastructure.

All nodes are discoverable via `cigness_node_registry.json` and spatially anchored via KML.

---

## 2. Data + config contracts

- **Per-node profiles**  
  - `r_node_xr_profile.cigness.json`  
  - `c_node_profile.cigness.json`  
  - `m_node_profile.cigness.json`  
  - `p_node_profile.cigness.json`

- **Global registries & schemas**  
  - `cigness_node_registry.json` – IDs + relationships  
  - `xr_content_pack.schema.cigness.json` – XR modules & constraints  
  - `eco_action_scoring.cigness.json` – eco-action → score mapping

These files are **source of truth** for deployment, monitoring, and governance.

---

## 3. Runtime flows (high level)

### 3.1 XR behavior-support flow (R-Node)

1. User arrives at R-Node (e.g., Downtown Calm Hub).  
2. Local XR node loads `r_node_xr_profile.cigness.json` and `xr_content_pack.schema.cigness.json`.  
3. User selects youth/adult mode → allowed modules + session limits enforced.  
4. Session runs (calm, focus, craving-resistance, eco-world restoration).  
5. Telemetry (aggregated only) is logged and pushed to the Cigness API:
   - session type, duration, self-reported craving/stress, smoke-free days, eco-actions.

### 3.2 Circularity flow (C-Node → M-Node → R-Node)

1. C-Node collects butts + plastics + returned devices per `c_node_profile.cigness.json`.  
2. Sorted and pre-processed outputs (pellets, cleaned composites) are shipped to linked M-Nodes.  
3. M-Nodes manufacture device shells and containers per `m_node_profile.cigness.json`.  
4. Devices are distributed back to R-Nodes; containers can be used in emergency water logistics.  
5. All flows are logged as material telemetry and surfaced on public dashboards.

### 3.3 Policy & zoning flow (P-Node)

1. P-Node defines constraints on tobacco retail density and smoke-free zones.  
2. Cigness services query P-Node profiles to:
   - Suggest new R/C/M node placements.  
   - Flag conflicts with tobacco retail expansion.  
3. Policy changes are versioned and auditable.

---

## 4. Telemetry & privacy

- All telemetry is:
  - **Pseudonymous**,  
  - **Aggregated for analytics**,  
  - **Never used for advertising**.  
- Youth data has stricter retention windows.  
- Node operators push telemetry to the Cigness API using the schemas in `eco_action_scoring.cigness.json` and node profiles.

---

## 5. Deployment model

- **Edge:** XR runtimes + local node services (R/C/M).  
- **Core:** Cigness API + registries + dashboards.  
- **Governance:** Municipal + Cigness Consortium, using node profiles as enforceable contracts.
