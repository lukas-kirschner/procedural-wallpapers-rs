/** Nearest Points
 Lukas Kirschner, 2021
 This software is public domain. Do whatever you want with it.
**/

#include <limits>
#include <cmath>
#include <array>
#include <unordered_set>

#include "lib/main.h"
#include "lib/perlin.h"

#define _NEARESTPOINT_DEBUG false

using coord = uint32_t;
using col_t = uint8_t;
using color = std::array<col_t, 3>;

class point {
public:
    coord const x;
    coord const y;
    color const col;

    point(coord x, coord y, color col) : x(x), y(y), col(col) {}

    [[nodiscard]] constexpr double distanceTo(coord x, coord y) const {
        return std::sqrt(std::pow((int32_t) x - (int32_t) this->x, 2) + std::pow((int32_t) y - (int32_t) this->y, 2));
    }

    bool operator==(const point &other) const {
        return (x == other.x && y == other.y && col == other.col);
    };

    /**
     * Draw the pixel at the given position in the color specified by this point
     * @tparam colordepth Color depth of the image
     * @param x X coordinate
     * @param y Y coordinate
     * @param image Image to draw
     */
    template<typename colordepth>
    void draw_pixel(coord x, coord y, BytesImage<colordepth> &image) const {
        image.setR(x, y, this->col[0]);
        image.setG(x, y, this->col[1]);
        image.setB(x, y, this->col[2]);
        if constexpr (_NEARESTPOINT_DEBUG) { // Draw points in red
            if (x == this->x && y == this->y) {
                image.setR(x, y, 255);
                image.setG(x, y, 0);
                image.setB(x, y, 0);
            }
        }
    }
};
namespace std {
    template<>
    struct hash<point> {
        std::size_t operator()(const point &p) const noexcept {
            //Analog to boost::hash_combine
            std::hash<coord> coord_hasher;
            std::hash<col_t> col_hasher;
            std::size_t seed = coord_hasher(p.x);
            seed ^= coord_hasher(p.y) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
            seed ^= col_hasher(p.col[0]) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
            seed ^= col_hasher(p.col[1]) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
            seed ^= col_hasher(p.col[2]) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
            return seed;
        }
    };
}

void populate_points(size_t num_points, std::unordered_set<point> &points) {
    for (size_t i = 0; i < num_points; ++i) {
        col_t r = 128 + (rand() % 128);
        col_t g = 128 + (rand() % 128);
        col_t b = 128 + (rand() % 128);
        coord x = rand() % bytes->width;
        coord y = rand() % bytes->height;
        color col = {r, g, b};
        points.emplace(x, y, col);
    }
}

void color_image(std::unordered_set<point> &points) {
    for (coord x = 0; x < bytes->width; ++x) {
        for (coord y = 0; y < bytes->height; ++y) {
            double mindist = std::numeric_limits<double>::max();
            point const *selected;
            std::for_each(points.begin(), points.end(), [&](point const &p) {
                if (p.distanceTo(x, y) < mindist) {
                    selected = &p;
                    mindist = p.distanceTo(x, y);
                }
            });
            selected->draw_pixel(x, y, *bytes);
        }
    }
}

void draw() {
    size_t const num_points = std::ceil(
            (size_t) (bytes->height * bytes->width / 20000)); // Point on every 20000th pixel
    std::unordered_set<point> points;

    populate_points(num_points, points);

    color_image(points);
}
