// The common main method for all wallpaper generators in this project
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include <cstdlib>
#include <cstdio>
#include <ctime>
#include <cxxopts.hpp>

#include "main.h"

#define STRINGIFY(x) #x
#define TOSTRING(x) STRINGIFY(x)
#ifndef PRGNAME
#define PRGNAME out
#endif

uint8_t *bytes;

int main(int argc, char *argv[]) {
    bytes = new uint8_t[WID * HEI * 3];
//TODO Seed here
    srand((unsigned) time(NULL));
    draw();
    std::string fname;
    char has_no_filename = (argc <= 1);
    if (has_no_filename) {
        fname = TOSTRING(PRGNAME)".ppm";
    } else {
        fname = argv[1];
    }
    FILE *out = fopen(fname.c_str(), "w");

    fprintf(out, "P6\n");
    fprintf(out, "%d %d\n", WID, HEI);
    fprintf(out, "255\n");

    fwrite(bytes, 1, WID * HEI * 3, out);
    fclose(out);
    delete[] bytes;
    return 0;
}
