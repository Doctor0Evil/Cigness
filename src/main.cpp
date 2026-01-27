#include "ecoquit_engine.hpp"
#include <iostream>

int main() {
    EcoQuitEngine engine;

    EcoQuitInput input{};
    input.cigarettes_baseline_per_day = 20;
    input.cigarettes_current_per_day  = 5;
    input.days_since_quit_start       = 90;

    EcoQuitOutput out = engine.compute(input);

    std::cout << "Cigarettes avoided: " << out.cigarettes_avoided_total << "\n";
    std::cout << "CO2 avoided (kg): " << out.co2_kg_avoided << "\n";
    std::cout << "Butt litter avoided (kg): " << out.butt_litter_avoided_kg << "\n";
    std::cout << "Eco impact score (0-100): " << out.eco_impact_score << "\n";

    return 0;
}
