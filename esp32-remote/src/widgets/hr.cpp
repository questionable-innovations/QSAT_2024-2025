#include "hr.h"
#include "state.h"
#include "connection.h"


int draw_hr(int start_height) {


    tft.fillRect(0, start_height, width, standard_margin, background_color);

    return start_height + standard_margin;
}