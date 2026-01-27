#ifndef CIGNESS_CHAT_PLATFORM_ADAPTER_HPP
#define CIGNESS_CHAT_PLATFORM_ADAPTER_HPP

#include <string>

namespace cigness::adapters {

class ChatPlatformAdapter {
public:
    virtual ~ChatPlatformAdapter() = default;

    virtual void send_message(const std::string& user_id,
                              const std::string& message) = 0;

    virtual void send_eco_milestone(const std::string& user_id,
                                    double eco_score,
                                    double co2_kg,
                                    double butt_kg) = 0;

    virtual void send_craving_alert(const std::string& user_id,
                                    double craving_risk) = 0;
};

} // namespace cigness::adapters

#endif // CIGNESS_CHAT_PLATFORM_ADAPTER_HPP
