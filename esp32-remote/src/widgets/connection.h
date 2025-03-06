#pragma once

#include "freefonts.h"

#include <SPI.h>
#include <TFT_eSPI.h>
#include <XPT2046_Touchscreen.h>

#define XPT2046_IRQ 36
#define XPT2046_MOSI 32
#define XPT2046_MISO 39
#define XPT2046_CLK 25
#define XPT2046_CS 33

extern const int height;
extern const int width;
extern const int standard_margin;
extern const uint16_t background_color;

extern SPIClass mySpi;
extern XPT2046_Touchscreen ts;

extern TFT_eSPI tft;



