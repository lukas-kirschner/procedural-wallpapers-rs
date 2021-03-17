// ISLANDS
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include "lib/main.h"
#include "lib/perlin.h"

#define GRID_MARGINS 10
#define GRID_SIZE_IN_PX ((((WID > HEI) ? HEI : WID) - 2 * GRID_MARGINS) / 6)

#define THRESHOLDVAL1 195
#define THRESHOLDVAL2 190

unsigned char const background_base_color[] = {198, 151, 63};
unsigned char const border_color[] = {154, 115, 82};
unsigned char const foreground_base_color[] = {202, 168, 131};

unsigned char compute_threshold(unsigned char value, unsigned char replacement_1, unsigned char replacement_2,
                                unsigned char replacement_3) {
    if (value > THRESHOLDVAL1) {
        return replacement_1;
    } else {
        if (value > THRESHOLDVAL2) {
            return replacement_2;
        } else {
            return replacement_3;
        }
    }
}

void draw_horiz_dashed(int y) {
    int x;
    for (x = 0; x < WID; x++) {
        if (x % 20 < 10) {
            bytes[(WID * y + x) * 3] = 100;
            bytes[(WID * y + x) * 3 + 1] = 96;
            bytes[(WID * y + x) * 3 + 2] = 82;
        }
    }
}

void draw_vert_dashed(int x) {
    int y;
    for (y = 0; y < HEI; y++) {
        if (y % 20 < 10) {
            bytes[(WID * y + x) * 3] = 100;
            bytes[(WID * y + x) * 3 + 1] = 96;
            bytes[(WID * y + x) * 3 + 2] = 82;
        }
    }
}

void draw() {
    generate_noise();
    int x, y;
    for (x = 0; x < WID; x++) {
        for (y = 0; y < HEI; y++) {
            unsigned char val = 185 + (char) (fractal(x, y, 0.004, 8) * 70);
            bytes[(WID * y + x) * 3] = compute_threshold(val, foreground_base_color[0] * val / 255, border_color[0],
                                                         (background_base_color[0] + val) / 2);
            bytes[(WID * y + x) * 3 + 1] = compute_threshold(val, foreground_base_color[1] * val / 255, border_color[1],
                                                             (background_base_color[1] + val) / 2);
            bytes[(WID * y + x) * 3 + 2] = compute_threshold(val, foreground_base_color[2] * val / 255, border_color[2],
                                                             (background_base_color[2] + val) / 2);
        }
    }
    for (int y = GRID_MARGINS; y < HEI; y += GRID_SIZE_IN_PX) {
        draw_horiz_dashed(y);
    }
    for (int x = GRID_MARGINS; x < WID; x += GRID_SIZE_IN_PX) {
        draw_vert_dashed(x);
    }
}
