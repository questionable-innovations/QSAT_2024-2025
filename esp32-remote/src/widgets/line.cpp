#pragma once

#include "connection.h"
#include "state.h"
#include "line.h"

int draw_line(char* key, char* value, int start_height, ArmState colour) {
    int box_height = 32;

    uint16_t rgb_colour;

    convert_state_to_color(colour, &rgb_colour);

    tft.fillRect(0, start_height, width, box_height, rgb_colour);
    tft.setTextColor(TFT_WHITE, rgb_colour, true);

    int middle_start = ((box_height)/2) + start_height;

    tft.setFreeFont(FSS9);                 // Select the font
    tft.setTextDatum(ML_DATUM);
    tft.drawString(key, standard_margin , middle_start, GFXFF);

    tft.setFreeFont(FSSB9);                 // Select the font

    tft.setTextDatum(MR_DATUM);
    tft.drawString(value, width-standard_margin , middle_start, GFXFF);



    return start_height+box_height;
}
