// The common main method for all wallpaper generators in this project
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include <cstdlib>
#include <cstdio>
#include <ctime>
#include <cxxopts.hpp>
#include <Magick++.h>

#include "main.h"

#define STRINGIFY(x) #x
#define TOSTRING(x) STRINGIFY(x)
#ifndef PRGNAME
#define PRGNAME out
#endif

EightBitImage *bytes;

int main(int argc, char *argv[]) {
    cxxopts::Options options(TOSTRING(PRGNAME), "Procedural Wallpaper Generator");
    options.add_options()
            ("h,height", "Image Height", cxxopts::value<uint32_t>()->default_value(TOSTRING(HEI)))
            ("w,width", "Image Width", cxxopts::value<uint32_t>()->default_value(TOSTRING(WID)))
            ("s,random-seed", "Random Seed",
             cxxopts::value<uint32_t>()->default_value(std::to_string((unsigned) time(NULL))))
            ("o,out", "Output file", cxxopts::value<std::string>()->default_value(TOSTRING(PRGNAME)".png"))
            ("v,verbose", "Verbose output", cxxopts::value<bool>()->default_value("false"));
    auto cli = options.parse(argc, argv);
    uint32_t const h = cli["height"].as<uint32_t>();
    uint32_t const w = cli["width"].as<uint32_t>();
    uint32_t const seed = cli["random-seed"].as<uint32_t>();
    std::string outfilename = cli["out"].as<std::string>();

    bytes = new EightBitImage(w, h);

    srand(seed);
    draw();
    Magick::Image image;
    image.read(w, h, "rgb", Magick::CharPixel, bytes->getBuffer());
    image.write(outfilename);
//    FILE *out = fopen(fname.c_str(), "w");
//
//    fprintf(out, "P6\n");
//    fprintf(out, "%d %d\n", bytes->width, HEI);
//    fprintf(out, "255\n");
//
//    fwrite(bytes, 1, bytes->width * HEI * 3, out);
//    fclose(out);
    delete bytes;
    return 0;
}
