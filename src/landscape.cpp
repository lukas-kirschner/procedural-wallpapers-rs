//Code stolen from https://tyrellrummage.github.io/landscape/

#include <cstdlib>
#include <cstring>
#include <cmath>
#include <vector>
#include <memory>

#include "lib/main.h"

using vec = std::vector<double>;
using vecptr = std::unique_ptr<vec>;
vecptr points;

void generate_points(double steepness) {

    //first create a new array and set the initial point
    size_t arrlen = points->size() * 2 - 1;
    double newArray[arrlen];
    newArray[0] = points->at(0);

    //then, loop through the original points array starting from the first one (to correct off by one error)
    for (int i = 1; i < points->size(); i++) {

        //get the average between the two points
        double avg = (points->at(i) + points->at(i - 1)) / 2.0;

        //get an offset
        double offsetAm = (double) HEI / ((double) (points->size() - 1) * 20 / steepness);

        //make the offset random and able to be negative
        int rnd = rand() % ((int) offsetAm * 2 + 1) - (int) offsetAm;

        //add the randomness to the average (displace the point)
        avg += rnd;

        //push the displaced point as well as the original point
        newArray[2 * i - 1] = avg;
        newArray[2 * i] = points->at(i);
    }
    //finally, copy the new array into points
    points->resize(arrlen);
    for (int i = 0; i < arrlen; ++i) {
        (*points)[i] = newArray[i];
    }
}

void generate_layer(double level, double steepness, uint8_t r, uint8_t g, uint8_t b) {
    points = std::make_unique<vec>(2);
    (*points)[0] = level;
    (*points)[1] = level;
    int iters = (int) (log(bytes->width) / log(2.0)) + 1;
    for (int i = 0; i < iters; i++) {
        generate_points(steepness);
    }
    for (int i = 0; i < points->size(); i++) {
        int pt = (int) points->at(i);
        for (int ii = pt; ii < bytes->height; ii++) {
            if (ii >= 0 && i < bytes->width) {
                bytes->setR(i, ii, r);
                bytes->setG(i, ii, g);
                bytes->setB(i, ii, b);
            }
        }
    }
}

void draw() {
    points = std::make_unique<vec>();
    bytes->memset(240);
    uint8_t r = 128 + (rand() % 128);
    uint8_t g = 128 + (rand() % 128);
    uint8_t b = 128 + (rand() % 128);
    uint8_t m = 240 + (rand() % 16);
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
        default:
            break;
    }
    generate_layer(HEI * 2.3 / 5, 7, r, g, b);
    generate_layer(HEI * 2.5 / 5, 7, r - 20, g - 20, b - 20);
    generate_layer(HEI * 3.0 / 5, 5, r - 50, g - 50, b - 50);
    generate_layer(HEI * 4.0 / 5, 5, r - 100, g - 100, b - 100);
}
