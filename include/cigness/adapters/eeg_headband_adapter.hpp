#ifndef CIGNESS_EEG_HEADBAND_ADAPTER_HPP
#define CIGNESS_EEG_HEADBAND_ADAPTER_HPP

#include "cigness/craving_biophysics.hpp"
#include <functional>
#include <string>

namespace cigness::adapters {

class EEGHeadbandAdapter {
public:
    using SnapshotCallback = std::function<void(const BiophysicalSnapshot&)>;

    virtual ~EEGHeadbandAdapter() = default;

    virtual bool connect(const std::string& device_id) = 0;
    virtual void disconnect() = 0;

    virtual void start_streaming(SnapshotCallback cb) = 0;
    virtual void stop_streaming() = 0;
};

} // namespace cigness::adapters

#endif // CIGNESS_EEG_HEADBAND_ADAPTER_HPP
