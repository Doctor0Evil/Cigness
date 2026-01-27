#include "cigness/craving_biophysics.hpp"
#include <algorithm>

namespace cigness {

double CravingBiophysicsModel::craving_risk(const BiophysicalSnapshot& snap) const {
    double theta_component = std::clamp(snap.frontal_theta_power, 0.0, 1.0);
    double coherence_component = std::clamp(snap.theta_coherence_fp, 0.0, 1.0);
    double cardio_component = 0.0;
    if (snap.heart_rate_bpm > 60.0) {
        cardio_component = std::min((snap.heart_rate_bpm - 60.0) / 60.0, 1.0);
    }
    double stress_component = std::clamp(snap.stress_score, 0.0, 1.0);

    double score =
        theta_weight_ * theta_component +
        coherence_weight_ * coherence_component +
        cardio_weight_ * cardio_component +
        stress_weight_ * stress_component;

    return std::clamp(score, 0.0, 1.0);
}

double CravingBiophysicsModel::craving_risk_smoothed(
    const std::vector<BiophysicalSnapshot>& window) const
{
    if (window.empty()) {
        return 0.0;
    }

    double total = 0.0;
    for (const auto& snap : window) {
        total += craving_risk(snap);
    }
    return total / static_cast<double>(window.size());
}

} // namespace cigness
