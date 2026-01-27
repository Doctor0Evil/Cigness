#include "cigness/session_models.hpp"
#include "cigness/ecoquit_engine.hpp"
#include "cigness/craving_biophysics.hpp"
#include <iostream>

using namespace cigness;

int main() {
    UserProfile profile;
    profile.user_id = "demo_user";
    profile.baseline_cigarettes_per_day = 20;
    profile.ecosystem_region = "phoenix_urban";

    SessionState state;
    state.profile = profile;
    state.days_since_quit_start = 60;
    state.cigarettes_current_per_day = 5;

    EcoQuitEngine eco_engine;
    EcoQuitOutput eco_out = eco_engine.compute(state);

    std::cout << "User: " << state.profile.user_id << "\n";
    std::cout << "Cigarettes avoided: " << eco_out.cigarettes_avoided_total << "\n";
    std::cout << "CO2 avoided (kg): " << eco_out.co2_kg_avoided << "\n";
    std::cout << "Butt litter avoided (kg): " << eco_out.butt_litter_avoided_kg << "\n";
    std::cout << "Eco impact score (0–100): " << eco_out.eco_impact_score << "\n";

    CravingBiophysicsModel model;
    BiophysicalSnapshot snap{};
    snap.frontal_theta_power = 0.7;
    snap.theta_coherence_fp = 0.8;
    snap.heart_rate_bpm = 90.0;
    snap.hrv_index = 0.5;
    snap.stress_score = 0.6;

    double risk = model.craving_risk(snap);
    std::cout << "Sample craving risk: " << risk << "\n";

    return 0;
}
