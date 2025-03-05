#include "hr.h"
#include "state.h"
#include "connection.h"


int draw_hr(int start_height) {


    tft.fillRect(0, start_height, width, standard_margin, createRGB(5,10,5));

    return start_height + standard_margin;
}