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

void convert_state_to_color(ArmState state, uint16_t* color) {
    switch (state) {
        case Armed:
            *color = TFT_RED;
            break;
        case Disarmed:
            *color = TFT_GREEN;
            break;
        case Neutral:
            *color = TFT_YELLOW;
            break;
        case Disconnected:
            *color = TFT_WHITE;
            break;
    }
}