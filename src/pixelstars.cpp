/*
 * Pixel Stars, 2020 by Lukas Kirschner
 * The code in this file is Public Domain, you are free to modify and republish it.
 *
 * pixelstars draws a random number of pixels in a random color scheme influenced by perlin noise.
 */

#include <cstdlib>

#include "lib/main.h"
#include "lib/perlin.h"

static float const BASE_LIKELINESS = 0.55f;
static float const NOISE_INFLUENCE = 0.25f;
static uint8_t const PIXEL_BACKGROUND_MAX_GREYVALUE = 20;

static double const CUTOFF = 1.3;

static inline bool pixel_not_black(uint32_t x, uint32_t y) {
    if (bytes->getR(x, y) > PIXEL_BACKGROUND_MAX_GREYVALUE ||
        bytes->getG(x, y) > PIXEL_BACKGROUND_MAX_GREYVALUE ||
        bytes->getB(x, y) > PIXEL_BACKGROUND_MAX_GREYVALUE) {
        return true;
    }
    return false;
}

static void fill_background() {
    for (uint32_t x = 0; x < bytes->width; ++x) {
        for (uint32_t y = 0; y < bytes->height; ++y) {
            uint8_t const greyval =
                    ((fractal(x, y, 0.004f, 5) + 1.0f) / 2.0f) * (double) (PIXEL_BACKGROUND_MAX_GREYVALUE - 1);
            bytes->setR(x, y, greyval);
            bytes->setG(x, y, greyval);
            bytes->setB(x, y, greyval);
        }
    }
}

/*
 * Get the likeliness between 0 and 1 that pixel (x,y) has a star.
 */
static double get_star_likeliness(size_t x, size_t y) {
    float const noise_amount = fractal(x, y, 0.002f, 5);
    bool const has_neighbour =
            (x > 0 && pixel_not_black(x - 1, y)) || (y > 0 && (pixel_not_black(x, y - 1)));
    return (BASE_LIKELINESS + (NOISE_INFLUENCE * noise_amount)) * (has_neighbour ? 0.8 : 1);
}

void draw() {//TODO fill background with noise?
    bytes->memset(0);
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
    for (int x = 0; x < bytes->width; ++x) {
        for (int y = 0; y < bytes->height; ++y) {
            if (!(rand() % 12)) {
                double const likeliness = get_star_likeliness(x, y);
                uint8_t const ran = rand() % bytes->maxPixelValue();
                if ((uint16_t) ran * (likeliness + 1) >= bytes->maxPixelValue() * CUTOFF) { // Draw a star
                    int const ran_rem = ran - (bytes->maxPixelValue() / 2.0);
                    bytes->setR(x, y, r * likeliness);
                    bytes->setG(x, y, g * (1 - likeliness));
                    bytes->setB(x, y, b - 19 + (ran_rem % 20));
                }
//                printf("%.6f\n",likeliness);
            }
        }
    }
}
