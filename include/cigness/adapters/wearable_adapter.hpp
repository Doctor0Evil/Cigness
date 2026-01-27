#ifndef CIGNESS_WEARABLE_ADAPTER_HPP
#define CIGNESS_WEARABLE_ADAPTER_HPP

#include "cigness/craving_biophysics.hpp"
#include <functional>

namespace cigness::adapters {

class WearableAdapter {
public:
    using SnapshotCallback = std::function<void(BiophysicalSnapshot&)>;

    virtual ~WearableAdapter() = default;

    virtual bool connect() = 0;
    virtual void disconnect() = 0;

    // Enrich BiophysicalSnapshot with HR/HRV/stress data
    virtual void subscribe(SnapshotCallback cb) = 0;
};

} // namespace cigness::adapters

#endif // CIGNESS_WEARABLE_ADAPTER_HPP
