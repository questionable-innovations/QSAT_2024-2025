#pragma once

#include "connection.h"
#include "state.h"
#include "line.h"

int draw_line(char* key, char* value, int start_height) {
    int top_gap = 8;
    int box_height = 32;

    uint16_t base_colour;
    convert_state_to_color(Neutral, &base_colour);

    tft.fillRect(0, top_gap+start_height, width, box_height, base_colour);
    tft.setTextColor(TFT_WHITE, base_colour, true);

    int middle_start = ((box_height - 9)/2);

    tft.setFreeFont(FSS9);                 // Select the font
    tft.setTextDatum(TL_DATUM);
    tft.drawCentreString(key, top_gap , middle_start, GFXFF);

    tft.setFreeFont(FSSB9);                 // Select the font

    tft.setTextDatum(TR_DATUM);
    tft.drawCentreString(value, top_gap , middle_start, GFXFF);



    return top_gap+start_height+box_height;
}
