#ifndef CIGNESS_CRAVING_BIOPHYSICS_HPP
#define CIGNESS_CRAVING_BIOPHYSICS_HPP

#include <vector>
#include <cstdint>

namespace cigness {

// Minimal EEG/biophysical feature vector
struct BiophysicalSnapshot {
    double frontal_theta_power;        // normalized theta band power[web:27]
    double theta_coherence_fp;         // fronto-parietal coherence index[web:29]
    double heart_rate_bpm;
    double hrv_index;
    double stress_score;               // 0–1, from wearable vendor SDK
};

class CravingBiophysicsModel {
public:
    // Returns 0–1 craving risk from a single snapshot
    double craving_risk(const BiophysicalSnapshot& snap) const;

    // Simple smoothing over a window
    double craving_risk_smoothed(const std::vector<BiophysicalSnapshot>& window) const;

private:
    double theta_weight_ = 0.4;
    double coherence_weight_ = 0.3;
    double cardio_weight_ = 0.2;
    double stress_weight_ = 0.1;
};

} // namespace cigness

#endif // CIGNESS_CRAVING_BIOPHYSICS_HPP
