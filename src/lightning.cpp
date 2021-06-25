// LIGHTNING
// Attila Bagyoni, 2018
// This software is public domain. Do whatever you want with it.

#include <stdlib.h>
#include <vector>

#include "lib/main.h"
#include "lib/basicdrawing.h"

int step;
int distribution;

void generate_distribution() {
    distribution = rand() % 4;
}

void place_charge(int *x, int *y) {
    switch (distribution) {
        case 0:
            *x = rand() % bytes->width;
            *y = bytes->height - (rand() % (rand() % (bytes->height - 1) + 1)) - 2;
            break;
        case 1:
            *x = rand() % bytes->width;
            *y = rand() % (rand() % (bytes->height - 1) + 1);
            break;
        case 2:
            *x = bytes->width - (rand() % (rand() % bytes->width + 1)) - 1;
            *y = rand() % (bytes->height - 1);
            break;
        case 3:
            *x = rand() % (rand() % bytes->width + 1);
            *y = rand() % (bytes->height - 1);
            break;

    }
}

void next_step(std::vector<std::pair<int, int>> &particles) {
    int x, y;
    place_charge(&x, &y);
    int nearest;
    int nearest_d_sq = bytes->width * bytes->width + bytes->height * bytes->height;
    int i;
    for (i = 0; i < step; i++) {
        int dist_sq = (particles[i].first - x) * (particles[i].first - x)
                      + (particles[i].second - y) * (particles[i].second - y);
        if (dist_sq < nearest_d_sq) {
            nearest_d_sq = dist_sq;
            nearest = i;
        }
    }
    int dx = particles[nearest].first < x ? 1 : -1;
    int dy = particles[nearest].second < y ? 1 : -1;
    if (dx * (x - particles[nearest].first) > 3 * dy * (y - particles[nearest].second)) {
        dy = 0;
    } else if (dy * (y - particles[nearest].second) > 3 * dx * (x - particles[nearest].first)) {
        dx = 0;
    }
    particles[step].first = particles[nearest].first + dx;
    particles[step].second = particles[nearest].second + dy;
    draw_point(particles[step].first, particles[step].second);
    draw_point(particles[step].first, particles[step].second + 1);
    step++;
}

void draw() {
    uint32_t const PNUM = ((int) bytes->width * 10);
    std::vector<std::pair<int, int>> particles(PNUM, {0, 0});
    generate_distribution();
    particles[0].first = rand() % bytes->width;
    particles[0].second = 0;
    int red = rand() % 120;
    int green = rand() % 120;
    step = 1;
    while (step < PNUM) {
        if (step % 2000 == 0) {
            generate_distribution();
        }
        fgcolor(
                red + (200 - red) * (PNUM - step) / PNUM,
                green + (200 - green) * (PNUM - step) / PNUM,
                255);
        next_step(particles);
    }
}
