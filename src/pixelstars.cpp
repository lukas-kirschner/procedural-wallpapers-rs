/*
 * Pixel Stars, 2020 by Lukas Kirschner
 * The code in this file is Public Domain, you are free to modify and republish it.
 *
 * pixelstars draws a random number of pixels in a random color scheme influenced by perlin noise.
 */

#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>

#include "lib/main.h"
#include "lib/perlin.h"

static float const BASE_LIKELINESS = 0.55f;
static float const NOISE_INFLUENCE = 0.25f;
static uint8_t const PIXEL_BACKGROUND_MAX_GREYVALUE = 20;

static double const CUTOFF = 1.3;

/**
 * Get the pixel value at the specified coordinates.
 * Colors can be accessed and set by using brackets []
 * e.g.
 * pixel_at(x,y)[0] = R
 * pixel_at(x,y)[1] = G
 * pixel_at(x,y)[2] = B
 * @param x X coordinate
 * @param y Y coordinate
 * @return a pointer to the pixel
 */
static inline uint8_t *pixel_at(int x, int y) {
    x = abs(x);
    y = abs(y);
    return bytes + ((WID * y + x) * 3);
}

static inline bool pixel_not_black(size_t x, size_t y) {
    for (size_t i = 0; i < 3; i++) {
        if (pixel_at(x, y)[i] > PIXEL_BACKGROUND_MAX_GREYVALUE) {
            return true;
        }
    }
    return false;
}

static void fill_background() {
    for (int x = 0; x < WID; x++) {
        for (int y = 0; y < HEI; y++) {
            uint8_t const greyval =
                    ((fractal(x, y, 0.004f, 5) + 1.0f) / 2.0f) * (double) (PIXEL_BACKGROUND_MAX_GREYVALUE - 1);
            for (size_t i = 0; i < 3; i++) {
                pixel_at(x, y)[i] = greyval;
            }
        }
    }
}

/*
 * Get the likeliness between 0 and 1 that pixel (x,y) has a star.
 */
static double get_star_likeliness(size_t x, size_t y) {
    float const noise_amount = fractal(x, y, 0.002f, 5);
    bool const has_neighbour =
            (x > 0 && pixel_not_black(x - 1, y)) || (pixel_not_black(x, y - 1));
    return (BASE_LIKELINESS + (NOISE_INFLUENCE * noise_amount)) * (has_neighbour ? 0.8 : 1);
}

void draw() {//TODO fill background with noise?
    memset(bytes, 0, WID * HEI * 3);
    int r = 64 + (rand() % 128);
    int g = 128 + (rand() % 128);
    int b = 250 + (rand() % 5);
    int m = 240 + (rand() % 16);
    int rnd = rand() % 3;
    switch (rnd) {
        case 0:
            r = m;
            break;
        case 1:
            g = m;
            break;
        case 2:
            b = m;
            break;
    }
    generate_noise();
    fill_background();
    generate_noise();
    for (int x = 0; x < WID; ++x) {
        for (int y = 0; y < HEI; ++y) {
            if (!(rand() % 12)) {
                double const likeliness = get_star_likeliness(x, y);
                uint8_t const ran = rand() % UINT8_MAX;
                if ((uint16_t) ran * (likeliness + 1) >= UINT8_MAX * CUTOFF) { // Draw a star
                    int const ran_rem = ran - (UINT8_MAX / 2.0);
                    bytes[(WID * y + x) * 3 + 0] = r * likeliness;
                    bytes[(WID * y + x) * 3 + 1] = g * (1 - likeliness);
                    bytes[(WID * y + x) * 3 + 2] = b - 19 + (ran_rem % 20);
                }
//                printf("%.6f\n",likeliness);
            }
        }
    }
}
