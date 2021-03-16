#ifndef MAIN_H
#define MAIN_H

#include <stdint.h>

// replace the 0s with the width and height of your display respectively
#define WID 1920
#define HEI 1080

#if WID == 0 || HEI == 0
	#error "You haven't set your screen resolution. Please go to lib/main.h and change the WID and HEI macros."
#endif

extern uint8_t bytes[]; //TODO Refactor to use class?

int main(int argc, char *argv[]);
void draw();

#endif
