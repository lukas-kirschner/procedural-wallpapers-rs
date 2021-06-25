#ifndef PROCEDURAL_WALLPAPERS_BYTESIMAGE_H
#define PROCEDURAL_WALLPAPERS_BYTESIMAGE_H

#include <cstdint>
#include <stdexcept>
#include <limits>

template<typename BitType>
class BytesImage {
public:
    BytesImage(uint32_t width, uint32_t height) : width(width), height(height) {
        this->bytes = new uint8_t[width * height * 3];
    }


    /**
     * Get the Red value for the given pixel
     * @param x X coordinate of the pixel
     * @param y Y Coordinate of the pixel
     * @returns the Red value of the specified pixel
     */
    [[nodiscard]] BitType getR(uint32_t x, uint32_t y) const {
        this->enforce_bounds(x, y);
        return bytes[(this->width * y + x) * 3];
    }

    /**
     * Get the Green value for the given pixel
     * @param x X coordinate of the pixel
     * @param y Y Coordinate of the pixel
     * @returns the Green value of the specified pixel
     */
    [[nodiscard]] BitType getG(uint32_t x, uint32_t y) const {
        this->enforce_bounds(x, y);
        return bytes[(this->width * y + x) * 3 + 1];
    }

    /**
     * Get the Blue value for the given pixel
     * @param x X coordinate of the pixel
     * @param y Y Coordinate of the pixel
     * @returns the Blue value of the specified pixel
     */
    [[nodiscard]] BitType getB(uint32_t x, uint32_t y) const {
        this->enforce_bounds(x, y);
        return bytes[(this->width * y + x) * 3 + 2];
    }

    /**
     * Set the Red value for the given pixel
     * @param x X coordinate of the pixel
     * @param y Y Coordinate of the pixel
     * @param value Value to set the pixel's Red value to
     */
    void setR(uint32_t x, uint32_t y, BitType value) {
        this->enforce_bounds(x, y);
        bytes[(this->width * y + x) * 3] = value;
    }

    /**
     * Set the Green value for the given pixel
     * @param x X coordinate of the pixel
     * @param y Y Coordinate of the pixel
     * @param value Value to set the pixel's Green value to
     */
    void setG(uint32_t x, uint32_t y, BitType value) {
        this->enforce_bounds(x, y);
        bytes[(this->width * y + x) * 3 + 1] = value;
    }

    /**
     * Set the Blue value for the given pixel
     * @param x X coordinate of the pixel
     * @param y Y Coordinate of the pixel
     * @param value Value to set the pixel's Blue value to
     */
    void setB(uint32_t x, uint32_t y, BitType value) {
        this->enforce_bounds(x, y);
        bytes[(this->width * y + x) * 3 + 2] = value;
    }

    [[nodiscard]] BitType const *getBuffer() const {
        return this->bytes;
    }

    /**
     * Set all colors of all pixels to the specified grey value
     * @param value Grey value
     */
    void memset(BitType value) {
        for (uint32_t i = 0; i < this->width * this->height * 3; ++i) {
            this->bytes[i] = value;
        }
    }

    /**
     * Get the maximum allowed value for a pixel value
     * @return the maximum value
     */
    [[nodiscard]] constexpr auto maxPixelValue() const {
        return std::numeric_limits<BitType>::max();
    }

    ~BytesImage() {
        delete[] this->bytes;
    }

    uint32_t const width;
    uint32_t const height;
private:
    void enforce_bounds(uint32_t x, uint32_t y) const {
        if (x >= this->width) {
            throw std::runtime_error(
                    "X coordinate out of bounds: " + std::to_string(x) + " in " + std::to_string(this->width) + " x " +
                    std::to_string(this->height) + " image");
        }
        if (y >= this->height) {
            throw std::runtime_error(
                    "Y coordinate out of bounds: " + std::to_string(x) + " in " + std::to_string(this->width) + " x " +
                    std::to_string(this->height) + " image");
        }
    }

    BitType *bytes;
};

using EightBitImage = BytesImage<uint8_t>;


#endif //PROCEDURAL_WALLPAPERS_BYTESIMAGE_H
