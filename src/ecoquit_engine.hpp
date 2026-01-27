#ifndef ECOQUIT_ENGINE_HPP
#define ECOQUIT_ENGINE_HPP

#include <string>

struct EcoQuitInput {
    int cigarettes_baseline_per_day;      // typical daily cigarettes before quit
    int cigarettes_current_per_day;       // current daily cigarettes
    int days_since_quit_start;            // total days since program start
};

struct EcoQuitOutput {
    double cigarettes_avoided_total;
    double co2_kg_avoided;
    double butt_litter_avoided_kg;
    double eco_impact_score;              // normalized 0–100
};

class EcoQuitEngine {
public:
    EcoQuitOutput compute(const EcoQuitInput& input) const;

    // configurable factors (could later be loaded from ALN/JSON)
    double co2_kg_per_cigarette = 0.014;      // example factor, kg CO2 per cigarette
    double butt_kg_per_cigarette = 0.0003;    // example factor, kg of butt waste
};

#endif // ECOQUIT_ENGINE_HPP
