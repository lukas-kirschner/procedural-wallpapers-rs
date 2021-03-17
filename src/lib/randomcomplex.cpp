// Random complex function generator
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include <complex>
#include <cstdlib>

#include "randomcomplex.h"
#include "main.h"

std::complex<float> poly1[5], poly2[5];

void generate_poly(std::complex<float> *poly) {
    poly[1] = std::complex<float>(((int) (rand() & 255) - 128) / 100., ((int) (rand() & 255) - 128) / 100.);
    poly[2] = std::complex<float>(((int) (rand() & 255) - 128) / 100000., ((int) (rand() & 255) - 128) / 100000.);
    poly[3] = std::complex<float>(((int) (rand() & 255) - 128) / 100000000., ((int) (rand() & 255) - 128) / 100000000.);
    poly[4] = std::complex<float>(((int) (rand() & 255) - 128) / 10000000000.,
                                  ((int) (rand() & 255) - 128) / 10000000000.);
}

void generate_f() {
    generate_poly(poly1);
    generate_poly(poly2);
}

std::complex<float> p1(std::complex<float> z) {
    return poly1[4] * z * z * z * z + poly1[3] * z * z * z + poly1[2] * z * z + poly1[1] * z;
}

std::complex<float> p2(std::complex<float> z) {
    return poly2[4] * z * z * z * z + poly2[3] * z * z * z + poly2[2] * z * z + poly2[1] * z;
}

std::complex<float> f(std::complex<float> z) {
    return std::pow(std::complex<float>(0.0, 2.0), p1(z - std::complex<float>(50, 0)) / std::complex<float>(200, 0)) *
           std::complex<float>(100, 0) + p2(z);
}
