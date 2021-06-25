// ZEBRA
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include <cstring>
#include <complex>

#include "lib/main.h"
#include "lib/basicdrawing.h"
#include "lib/randomcomplex.h"

void draw() {
    bytes->memset(230);
    fgcolor(150, 150, 150);
    generate_f();
    int x, y;
    for (x = 0; x < bytes->width; x++) {
        for (y = 0; y < bytes->height; y++) {
            std::complex<float> z = std::complex<float>((float) ((float) x - (float) bytes->width / 2),
                                                        (float) ((float) y - (float) bytes->height / 2));
            z /= ((float) bytes->width) / 1000;
            char re = (char) f(z).real();
            if (re < 0) {
                draw_point(x, y);
            }
        }
    }
}
