# Pilot Protocol: Tobacco, Alcohol, and Litter in City District X v1.0
`pilots/city_district_tobacco_alcohol_litter_protocol_v1.0.md`

## 1. Scope

This pilot tests the Neuro‑EcoKarma governance architecture in a single urban district, focusing on three behavior classes:

1. Tobacco consumption (smoking events).
2. Alcohol consumption (standard drinks).
3. Litter vs. proper disposal of common packaging (glass, plastic, cans).

## 2. Cohort and Participation

- Voluntary cohort of residents and businesses in District X.
- Each participant:
  - Receives an Agent ID.
  - Installs a mobile app to log smoking, drinking, and disposal events.
  - Consents to use of environmental sensor data and LCA factors for impact computation.
  - Retains neurorights; no brain data is collected.

## 3. Behavioral Mappings

### 3.1 Tobacco (Smoking)

For each smoking event:

- Action_Type: `Smoke_Cigarette`.
- Use LCA factors for:
  - CO₂e per cigarette.
  - PM₂․₅ and NOₓ per cigarette (combustion).
- Map to:

  \[
  M^{(\text{cig})} = (M_{\text{CO₂e}}, M_{\text{PM2.5}}, M_{\text{NOx}}, M_{\text{butt_waste}}, \dots).
  \]

Context adjustments:

- Indoor vs outdoor.
- Proximity to sensitive receptors (school, clinic).

### 3.2 Alcohol (Standard Drinks)

For each drink:

- Action_Type: `Consume_Standard_Drink`.
- Use LCA factors for:
  - CO₂e per standard drink (production, distribution).
  - Packaging waste (glass or can).

Map to \(M_i\) accordingly.

### 3.3 Litter vs Proper Disposal

For each disposal event:

- Action_Type:
  - `Dispose_Proper` — recognized bin.
  - `Dispose_Litter` — reported or detected litter.

Map:

- Proper disposal: low or zero \(M_{\text{mismanaged}}\).
- Litter: positive \(M_{\text{mismanaged}}\) (e.g., kg plastics, glass).

## 4. Parameters \(\beta_i\) and \(\lambda_i\)

### 4.1 \(\beta_i\)

Example choices:

- \(\beta_{\text{CO₂e}} = 1\) (CO₂e baseline).
- \(\beta_{\text{PM2.5}}\): from health LCIA models (DALY/kg PM₂․₅).
- \(\beta_{\text{mismanaged_plastic}}\): from ecotoxicity/persistence indicators.

Documented in the `Governing_Parameter` table for jurisdiction `CityX_DistrictX`.

### 4.2 \(\lambda_i\)

Define λ‑fields for:

- School zones and clinics: \(\lambda_i > 1\).
- Standard residential: \(\lambda_i = 1\).
- Existing over‑burdened blocks (identified via environmental justice data): \(\lambda_i\) further increased.

A public map of λ‑values MUST be published.

## 5. Ecological Polytope \(P_{\text{eco}}\)

For District X, define stressor vector:

\[
x = (\overline{\text{PM2.5}}, \overline{\text{NOx}}, \text{litter_index}, \dots).
\]

Build \(P_{\text{eco}}\) using:

- Local air‑quality standards (daily mean PM₂․₅, NO₂).
- Target litter index thresholds.

Express as inequalities \(A_{\text{eco}} x \le b_{\text{eco}}\). Store in `Eco_Polytope` tables.

## 6. Governance Application

### 6.1 Actions Subject to Gating

For this pilot:

- High‑impact actions requiring `ActionAllowed` check:
  - Operation of certain outdoor heaters or generators by low‑K agents.
  - Approval to run larger outdoor events with emissions.
- Everyday smoking/drinking/disposal are logged but not blocked.

### 6.2 Role Levels

- Participants start as `FullOperator` for this domain.
- If \(K_{\text{person}} < -0.5 K_{\max}\): may lose eligibility for certain permits (e.g., event permits).
- If \(K_{\text{person}} < -K_{\max}\): may require completion of restorative actions before regaining eligibility.

## 7. Restorative Action Catalogue

Examples:

- `Cleanup_Event`: verified participation in district cleanup.
- `Tree_Planting`: planting and maintaining trees in designated areas.
- `Monitoring_Sponsorship`: funding or hosting air‑quality sensors.

Each has:

- Standard \(\Delta M_i < 0\) (representing removed or avoided impacts).
- Mapped \(\Delta K\) increments.

## 8. Privacy and Fairness Safeguards

- No neural data collection.
- Personal identifiers pseudonymized in research outputs.
- Year‑one focus: evaluate:
  - Change in district PM₂․₅ and litter.
  - Distribution of role downgrades by demographic group.
  - Participant perception of fairness and burden.

## 9. Timeline

- Month 0–3: recruitment, parameter calibration.
- Month 4–15: live pilot with quarterly reviews.
- Month 16–18: analysis and publication.
