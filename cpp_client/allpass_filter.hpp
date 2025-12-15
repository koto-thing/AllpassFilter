#pragma once
#include <cstddef>
#include <vector>

extern "C" {
    struct CAllPass;

    CAllPass* allpass_create(size_t max_delay, float initial_delay, float gain);
    void allpass_destroy(CAllPass* ptr);
    float allpass_process(CAllPass* ptr, float input);
    void allpass_process_block(CAllPass* ptr, const float* input, float* output, size_t len);
    void allpass_set_delay(CAllPass* ptr, float delay);
    void allpass_set_gain(CAllPass* ptr, float gain);
    void allpass_set_smoothing(CAllPass* ptr, float factor);
}

class AllPassFilter {
private:
    CAllPass* handle;

public:
    AllPassFilter(size_t max_delay, float initial_delay, float gain) {
        handle = allpass_create(max_delay, initial_delay, gain);
    }

    ~AllPassFilter() {
        if (handle) {
            allpass_destroy(handle);
            handle = nullptr;
        }
    }

    AllPassFilter(const AllPassFilter&) = delete;
    AllPassFilter& operator=(const AllPassFilter&) = delete;

    AllPassFilter(AllPassFilter&& other) noexcept : handle(other.handle) {
        other.handle = nullptr;
    }

    float process(float input) {
        return allpass_process(handle, input);
    }

    void process_block(const std::vector<float>& input, std::vector<float>& output) {
        if (output.size() < input.size()) {
            output.resize(input.size());
        }

        allpass_process_block(handle, input.data(), output.data(), input.size());
    }

    void set_delay(float delay) {
        allpass_set_delay(handle, delay);
    }

    void set_gain(float gain) {
        allpass_set_gain(handle, gain);
    }

    void set_smoothing(float factor) {
        allpass_set_smoothing(handle, factor);
    }
};