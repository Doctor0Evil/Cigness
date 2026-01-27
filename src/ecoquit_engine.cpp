#include "cigness/ecoquit_engine.hpp"
#include <algorithm>

namespace cigness {

EcoQuitOutput EcoQuitEngine::compute(const SessionState& state) const {
    EcoQuitOutput out{};

    if (state.days_since_quit_start <= 0 ||
        state.profile.baseline_cigarettes_per_day < 0 ||
        state.cigarettes_current_per_day < 0) {
        return out;
    }

    int baseline = std::max(state.profile.baseline_cigarettes_per_day, 0);
    int current  = std::max(state.cigarettes_current_per_day, 0);

    int avoided_per_day = std::max(baseline - current, 0);
    out.cigarettes_avoided_total =
        static_cast<double>(avoided_per_day) *
        static_cast<double>(state.days_since_quit_start);

    out.co2_kg_avoided = out.cigarettes_avoided_total * co2_kg_per_cigarette;
    out.butt_litter_avoided_kg = out.cigarettes_avoided_total * butt_kg_per_cigarette;

    const double max_cigs_for_full_score = 2000.0;
    double ratio = std::min(out.cigarettes_avoided_total / max_cigs_for_full_score, 1.0);
    out.eco_impact_score = ratio * 100.0;

    return out;
}

} // namespace cigness
