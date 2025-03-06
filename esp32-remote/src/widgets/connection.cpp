#include "connection.h"
#include "state.h"

const int height = 320;
const int width = 240;

const int standard_margin = 8;
const uint16_t background_color = createRGB(5,10,5);


SPIClass mySpi = SPIClass(VSPI);
XPT2046_Touchscreen ts(XPT2046_CS, XPT2046_IRQ);

TFT_eSPI tft = TFT_eSPI();
