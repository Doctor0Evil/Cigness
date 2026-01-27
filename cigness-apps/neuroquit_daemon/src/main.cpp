#include "cigness/session_models.hpp"
#include "cigness/ecoquit_engine.hpp"
#include "cigness/craving_biophysics.hpp"
#include "cigness/adapters/chat_platform_adapter.hpp"
#include <memory>
#include <thread>
#include <chrono>

// Forward-declare a console adapter factory (implementation as shown earlier)
namespace cigness::adapters {
    std::unique_ptr<ChatPlatformAdapter> make_console_chat();
}

int main() {
    using namespace cigness;

    UserProfile profile;
    profile.user_id = "daemon_user";
    profile.baseline_cigarettes_per_day = 15;
    profile.ecosystem_region = "phoenix_urban";

    SessionState state;
    state.profile = profile;
    state.days_since_quit_start = 30;
    state.cigarettes_current_per_day = 3;

    EcoQuitEngine eco_engine;
    CravingBiophysicsModel bio_model;

    auto chat = cigness::adapters::make_console_chat();

    for (int i = 0; i < 10; ++i) {
        BiophysicalSnapshot snap{};
        snap.frontal_theta_power = 0.5 + 0.05 * i;
        snap.theta_coherence_fp = 0.4 + 0.04 * i;
        snap.heart_rate_bpm = 75.0 + i;
        snap.hrv_index = 0.5;
        snap.stress_score = 0.4 + 0.03 * i;

        double risk = bio_model.craving_risk(snap);

        if (risk > 0.6) {
            chat->send_craving_alert(profile.user_id, risk);
        }

        EcoQuitOutput eco_out = eco_engine.compute(state);
        if (eco_out.eco_impact_score >= 50.0 && i == 0) {
            chat->send_eco_milestone(profile.user_id,
                                     eco_out.eco_impact_score,
                                     eco_out.co2_kg_avoided,
                                     eco_out.butt_litter_avoided_kg);
        }

        std::this_thread::sleep_for(std::chrono::seconds(1));
    }

    return 0;
}
