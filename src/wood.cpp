// WOOD
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include <cstdlib>

#include "lib/main.h"
#include "lib/perlin.h"

void draw() {
    generate_noise();
    int detail = rand() % (500000 / HEI) + (2000000 / HEI);
    int x, y;
    for (x = 0; x < bytes->width; x++) {
        for (y = 0; y < HEI; y++) {
            unsigned char depth = fractal(x / 10.0, y, 0.005, 4) * detail;
            unsigned char noise = rand() & 0xFF;
            bytes->setR(x, y, 160 + depth / 4 + noise / 8);
            bytes->setG(x, y, 82 + depth / 4 + noise / 8);
            bytes->setB(x, y, 45 + depth / 8 + noise / 16);
        }
    }
}
