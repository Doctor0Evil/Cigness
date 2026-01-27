#include "ecoquit_engine.hpp"
#include <algorithm>

EcoQuitOutput EcoQuitEngine::compute(const EcoQuitInput& input) const {
    EcoQuitOutput out{};

    if (input.days_since_quit_start <= 0 ||
        input.cigarettes_baseline_per_day < 0 ||
        input.cigarettes_current_per_day < 0) {
        return out;
    }

    int baseline = input.cigarettes_baseline_per_day;
    int current  = input.cigarettes_current_per_day;

    baseline = std::max(baseline, 0);
    current  = std::max(current, 0);

    int avoided_per_day = std::max(baseline - current, 0);
    out.cigarettes_avoided_total = static_cast<double>(avoided_per_day) *
                                   static_cast<double>(input.days_since_quit_start);

    out.co2_kg_avoided = out.cigarettes_avoided_total * co2_kg_per_cigarette;
    out.butt_litter_avoided_kg = out.cigarettes_avoided_total * butt_kg_per_cigarette;

    // Simple eco-impact score: saturating curve, 0–100
    double max_cigs_for_full_score = 2000.0;
    double ratio = std::min(out.cigarettes_avoided_total / max_cigs_for_full_score, 1.0);
    out.eco_impact_score = ratio * 100.0;

    return out;
}
