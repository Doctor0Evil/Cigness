#include "cigness/adapters/chat_platform_adapter.hpp"
#include <iostream>

namespace cigness::adapters {

class ConsoleChatAdapter : public ChatPlatformAdapter {
public:
    void send_message(const std::string& user_id,
                      const std::string& message) override {
        std::cout << "[Cigness][" << user_id << "] " << message << "\n";
    }

    void send_eco_milestone(const std::string& user_id,
                            double eco_score,
                            double co2_kg,
                            double butt_kg) override {
        std::cout << "[Cigness][" << user_id
                  << "] Eco score: " << eco_score
                  << ", CO2 avoided (kg): " << co2_kg
                  << ", Butt litter avoided (kg): " << butt_kg << "\n";
    }

    void send_craving_alert(const std::string& user_id,
                            double craving_risk) override {
        std::cout << "[Cigness][" << user_id
                  << "] Craving risk: " << craving_risk << "\n";
    }
};

} // namespace cigness::adapters
