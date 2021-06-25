// CLOUDS
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include <cstdlib>
#include <cmath>

#include "lib/main.h"
#include "lib/perlin.h"

float sigmoid(float x) {
    return tanh(tanh(2 * x - 0.5) * 2);
}

void draw() {
    Perlin<uint8_t> myPerlin(*bytes);
    myPerlin.generate_noise();
    float freq = 0.002 * (rand() & 0xFF) / 0xFF + 0.003;
    int x, y;
    for (x = 0; x < bytes->width; x++) {
        for (y = 0; y < bytes->height; y++) {
            float val = 0.5 * sigmoid(myPerlin.fractal(x, y, freq, 7)) + 0.5;
            bytes->setR(x, y, val * 230 + 25);
            bytes->setG(x, y, val * 255);
            bytes->setB(x, y, 255);
        }
    }
}
