#ifndef MAIN_H
#define MAIN_H

#include <cstdint>
#include "BytesImage.h"

// Default values for width and height
#define DEFAULT_WIDTH 1920
#define DEFAULT_HEIGHT 1080

extern EightBitImage *bytes;

int main(int argc, char *argv[]);

void draw();

#endif
