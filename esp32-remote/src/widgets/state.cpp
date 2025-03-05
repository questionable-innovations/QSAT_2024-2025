#include "state.h"
#include <string.h>
#include <TFT_eSPI.h>


void convert_state_to_string(ArmState state, char* buffer) {
    switch (state) {
        case Armed:
            strcpy(buffer, "ARMED");
            break;
        case Disarmed:
            strcpy(buffer, "DISARMED");
            break;
        case Neutral:
            strcpy(buffer, "NEUTRAL");
            break;
        case Disconnected:
            strcpy(buffer, "DISCONNECTED");
            break;
    }
}

uint16_t createRGB(int r, int g, int b)
{   
    // 0b1111100000000000 (Red, 255,0,0)
    return ((r & 0x1F) << 11) | ((g & 0x3F) << 5) | (b & 0x1F);
}

// https://rgbcolorpicker.com/565
void convert_state_to_color(ArmState state, uint16_t* color) {
    switch (state) {
        case Armed:
            *color = createRGB(25, 10, 1);
            break;
        case Disarmed:
            *color = createRGB(7, 45, 5);
            // *color = createRGB(255, 0, 0);

            break;
        case Neutral:
            *color = createRGB(11, 11, 11);
            break;
        case Disconnected:
            *color = TFT_WHITE;
            break;
    }
}