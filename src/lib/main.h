#ifndef MAIN_H
#define MAIN_H

#include <cstdint>
#include "BytesImage.h"

// Default values for width and height
#define WID 1920
#define HEI 1080

extern EightBitImage *bytes; //TODO Refactor to use class?

int main(int argc, char *argv[]);

void draw();

#endif
