// FLOW
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include <cstdlib>
#include <cmath>

#include "lib/main.h"
#include "lib/perlin.h"

uint16_t const NUM_PARTICLES = 5000;
uint16_t const PATH_LEN = 300;

double flow[WID * HEI]; // TODO refactor!!!
double curvature;
double frequency;
int8_t signum;

void make_single_path() {
    double x = rand() % bytes->width;
    double y = rand() % bytes->height;
    int i = 0;
    while (i < PATH_LEN && x > 0 && x < bytes->width && y > 0 && y < HEI) {
        int xi = x;
        int yi = y;
        flow[(bytes->width * yi + xi)] += ((double) (PATH_LEN - i)) / PATH_LEN;
        double angle = 2 * M_PI * (fractal(x, y, frequency, 6) - 0.5) * curvature;
        x += cos(angle);
        y += sin(angle);
        i++;
    }
}

void draw() {
    generate_noise();
    signum = (rand() & 0x1) ? 1 : -1;
    curvature = 0.3 + 0.1 * ((float) (rand() & 0xFF)) / 0xFF;
    frequency = 0.003 + 0.003 * ((float) (rand() & 0xFF)) / 0xFF;
    int i;
    for (i = 0; i < NUM_PARTICLES; i++) {
        make_single_path();
    }
    double max;
    for (i = 0; i < bytes->width * bytes->height; i++) {
        max = flow[i] > max ? flow[i] : max;
    }
    for (int x = 0; x < bytes->width; x++) {
        for (int y = 0; y < bytes->height; y++) {
            auto val = (uint8_t) (256.0 + (double) signum * (55.0 + 200.0 * flow[y * bytes->width + x] / (double) max));
            bytes->setR(x, y, val);
            bytes->setG(x, y, val);
            bytes->setB(x, y, val);
        }
    }
}
