#include <iostream>
#include <vector>
#include "allpass_filter.hpp"

int main() {
    // インスタンス生成
    AllPassFilter apf(1000, 10.5f, 0.5f);

    std::cout << "--- Single Sample Processing ---" << std::endl;

    // 1サンプル処理
    for (int i = 0; i < 5; ++i) {
        float input = (i == 0) ? 1.0f : 0.0f;
        float output = apf.process(input);
        std::cout << "[" << i << "] " << output << std::endl;
    }

    std::cout << "\n--- Block Processing ---" << std::endl;

    // ブロック処理
    std::vector<float> input_block = {1.0f, 0.0f, 0.0f, 0.0f, 0.0f};
    std::vector<float> output_block(5);

    apf.process_block(input_block, output_block);

    for (size_t i = 0; i < output_block.size(); ++i) {
        std::cout << "[" << i << "] " << output_block[i] << std::endl;
    }

    // 4. パラメータ変更
    apf.set_delay(20.0f);
    std::cout << "\nDelay changed to 20.0" << std::endl;

    return 0;
}