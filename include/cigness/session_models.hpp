#ifndef CIGNESS_SESSION_MODELS_HPP
#define CIGNESS_SESSION_MODELS_HPP

#include <string>
#include <chrono>
#include <vector>

namespace cigness {

struct UserProfile {
    std::string user_id;
    int baseline_cigarettes_per_day;
    std::string ecosystem_region;  // e.g. "phoenix_urban"
};

enum class EventType {
    CravingDetected,
    CravingResolved,
    Slip,          // smoked when intended not to
    CigaretteFreeDay,
    EcoMilestone
};

struct SessionEvent {
    EventType type;
    std::chrono::system_clock::time_point timestamp;
    double craving_score;  // 0–1 normalized, from biophysical model
    int cigarettes_today;
};

struct SessionState {
    UserProfile profile;
    int days_since_quit_start;
    int cigarettes_current_per_day;
    std::vector<SessionEvent> events;
};

} // namespace cigness

#endif // CIGNESS_SESSION_MODELS_HPP
