#ifndef PERLIN_H
#define PERLIN_H

#include <vector>
#include <cmath>
#include "BytesImage.h"

/**
 * Perlin Noise Generator Class, refactored from the old perlin.c noise generator by Attila Bagyoni, 2018
 */
template<typename ImageBitDepth>
class Perlin {
public:
    Perlin(BytesImage<ImageBitDepth> const &img) :
            gradient(img.width, std::vector<std::pair<double, double>>(img.height, {0.0, 0.0})) {
    }

    Perlin() = delete;

    ~Perlin() = default;

    void generate_noise() {
        int x, y;
        for (x = 0; x < gradient.size(); x++) {
            for (y = 0; y < gradient[0].size(); y++) {
                int rnd = rand() & 0xFFF;
                gradient[x][y].first = sin(rnd);
                gradient[x][y].second = cos(rnd);
            }
        }
    }


    [[nodiscard]] double distance_along_gradient(double x, double y, int gridx, int gridy) const {
        return (x - gridx) * gradient[gridx][gridy].first
               + (y - gridy) * gradient[gridx][gridy].second;
    }

    [[nodiscard]] double perlin(double x, double y) const {
        double d1 = distance_along_gradient(x, y, x, y);
        double d2 = distance_along_gradient(x, y, x + 1, y);
        double d3 = distance_along_gradient(x, y, x, y + 1);
        double d4 = distance_along_gradient(x, y, x + 1, y + 1);
        double i1 = inter(d1, d2, x - (int) x);
        double i2 = inter(d3, d4, x - (int) x);
        return inter(i1, i2, y - (int) y);
    }

    [[nodiscard]] double fractal(double x, double y, double freq, int depth) const {
        if (depth == 0) {
            return 0;
        }
        return perlin(x * freq, y * freq) + fractal(x, y, freq * 2, depth - 1) / 2;
    }


private:
    [[nodiscard]] static float inter(float x, float y, float weight) {
        float yweight = weight * weight * (2 - weight);
        return x * (1 - yweight) + y * yweight;
    }

/**
 * Perlin Noise Gradient with values -1 <= x <= 1
 */
    std::vector<std::vector<std::pair<double, double>>> gradient;
};

#endif
