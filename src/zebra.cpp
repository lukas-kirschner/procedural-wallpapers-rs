// ZEBRA
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include <cstring>
#include <complex>

#include "lib/main.h"
#include "lib/basicdrawing.h"
#include "lib/randomcomplex.h"

void draw() {
    memset(bytes, 230, WID * HEI * 3);
    fgcolor(150, 150, 150);
    generate_f();
    int x, y;
    for (x = 0; x < WID; x++) {
        for (y = 0; y < HEI; y++) {
            std::complex<float> z = std::complex<float>(x - WID / 2, (y - HEI / 2));
            z /= ((float) WID) / 1000;
            char re = (char) f(z).real();
            if (re < 0) {
                draw_point(x, y);
            }
        }
    }
}
