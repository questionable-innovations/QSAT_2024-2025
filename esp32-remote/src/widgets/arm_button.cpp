#include "arm_button.h"
#include "connection.h"


int draw_button(ArmState state, bool selected, Direction direction, int start_height) {
    int button_height = 32;

    uint16_t color;
    convert_state_to_color(state, &color);

    const char* state_string;
    if (state == Disarmed) {
        state_string = "DISARM";
    } else {
        state_string = "ARM";
    }

    int base_left = 0;
    if (direction == Right) {
        base_left = width/2;
    }

    tft.fillRect(base_left, start_height, width/2, button_height, background_color);

    tft.fillRect(base_left+standard_margin, start_height, width/2-standard_margin, button_height, color);
    if (selected) {
        tft.drawRect(base_left+standard_margin, start_height, width/2-standard_margin, button_height, TFT_WHITE);
    }

    tft.setTextDatum(CC_DATUM);
    tft.setTextColor(TFT_WHITE, color, true);
    tft.setFreeFont(FSSB9);
    tft.drawString(state_string, base_left + width/4, start_height + button_height/2);
    
    return start_height+standard_margin;
}