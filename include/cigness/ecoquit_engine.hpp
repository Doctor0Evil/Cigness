#ifndef CIGNESS_ECOQUIT_ENGINE_HPP
#define CIGNESS_ECOQUIT_ENGINE_HPP

#include "session_models.hpp"

namespace cigness {

struct EcoQuitOutput {
    double cigarettes_avoided_total;
    double co2_kg_avoided;
    double butt_litter_avoided_kg;
    double eco_impact_score;      // 0–100
};

class EcoQuitEngine {
public:
    EcoQuitOutput compute(const SessionState& state) const;

    double co2_kg_per_cigarette = 0.014;   // approximate emissions per cigarette[web:39]
    double butt_kg_per_cigarette = 0.0003; // approximate butt mass
};

} // namespace cigness

#endif // CIGNESS_ECOQUIT_ENGINE_HPP
