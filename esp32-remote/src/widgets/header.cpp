#include "header.h"
#include "freefonts.h"
#include "connection.h"

int draw_header(ArmState state) {
    // Red Box, with white text "Current State" small, and "Disarmed" big
    const int box_height = 100;

    uint16_t base_colour;
    convert_state_to_color(state, &base_colour);

    char state_str[24];
    convert_state_to_string(state, state_str);

    tft.fillRect(0, 0, width, box_height, base_colour);
    tft.setTextColor(TFT_WHITE, base_colour, true);
    tft.setTextDatum(MC_DATUM);

    int big_text_start = ((box_height)/2);

    tft.setFreeFont(FSS9);                 // Select the font
    tft.drawCentreString("REMOTE STATE", width/2 , (big_text_start-9)/2, GFXFF);


    tft.setFreeFont(FSSB18);                 // Select the font
    tft.drawCentreString(state_str, width/2 , big_text_start, GFXFF);

    return box_height;
}