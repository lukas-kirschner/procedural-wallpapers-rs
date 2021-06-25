// Basic drawing utilities
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include "main.h"

uint8_t r, g, b;

void fgcolor(uint8_t red, uint8_t green, uint8_t blue) {
    r = red;
    g = green;
    b = blue;
}

void draw_point(int x, int y) {
    if (x > 0 && x < bytes->width && y > 0 && y < HEI) {
        bytes->setR(x, y, r);
        bytes->setG(x, y, g);
        bytes->setB(x, y, b);
    }
}
