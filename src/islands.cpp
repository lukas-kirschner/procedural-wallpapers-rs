// ISLANDS
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include "lib/main.h"
#include "lib/perlin.h"

#define GRID_MARGINS 10//TODO refactor:
#define GRID_SIZE_IN_PX ((((bytes->width > bytes->height) ? bytes->height : bytes->width) - 2 * GRID_MARGINS) / 6)

uint8_t const THRESHOLDVAL1 = 195;
uint8_t const THRESHOLDVAL2 = 190;

uint8_t const background_base_color[] = {198, 151, 63};
uint8_t const border_color[] = {154, 115, 82};
uint8_t const foreground_base_color[] = {202, 168, 131};
uint8_t const dashed_grid_color[] = {100, 96, 82};

unsigned char compute_threshold(uint8_t value, uint8_t replacement_1, uint8_t replacement_2,
                                uint8_t replacement_3) {
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
    for (x = 0; x < bytes->width; x++) {
        if (x % 20 < 10) {
            bytes->setR(x, y, dashed_grid_color[0]);
            bytes->setG(x, y, dashed_grid_color[1]);
            bytes->setB(x, y, dashed_grid_color[2]);
        }
    }
}

void draw_vert_dashed(int x) {
    int y;
    for (y = 0; y < bytes->height; y++) {
        if (y % 20 < 10) {
            bytes->setR(x, y, dashed_grid_color[0]);
            bytes->setG(x, y, dashed_grid_color[1]);
            bytes->setB(x, y, dashed_grid_color[2]);
        }
    }
}

void draw() {
    Perlin myPerlin = Perlin(*bytes);
    myPerlin.generate_noise();
    int x, y;
    for (x = 0; x < bytes->width; x++) {
        for (y = 0; y < bytes->height; y++) {
            unsigned char val = 185 + (char) (myPerlin.fractal(x, y, 0.004, 8) * 70);
            bytes->setR(x, y, compute_threshold(val, foreground_base_color[0] * val / 255, border_color[0],
                                                (background_base_color[0] + val) / 2));
            bytes->setG(x, y, compute_threshold(val, foreground_base_color[1] * val / 255, border_color[1],
                                                (background_base_color[1] + val) / 2));
            bytes->setB(x, y, compute_threshold(val, foreground_base_color[2] * val / 255, border_color[2],
                                                (background_base_color[2] + val) / 2));
        }
    }
    for (int y = GRID_MARGINS; y < bytes->height; y += GRID_SIZE_IN_PX) {
        draw_horiz_dashed(y);
    }
    for (int x = GRID_MARGINS; x < bytes->width; x += GRID_SIZE_IN_PX) {
        draw_vert_dashed(x);
    }
}
