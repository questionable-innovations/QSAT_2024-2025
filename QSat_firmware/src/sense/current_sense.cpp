#include "current_sense.h"
#include <Energia.h>
#include <definitions/pins.h>

void setup_current_sense() {
    pinMode(CurrentSense, INPUT);
}

float read_reload_miliamps() {
    int rawRead = analogRead(CurrentSense);
    int analog_input_mV = (3300 * (rawRead)) >> 12;

    // Calculate the current in amps, based on the opamp config
    float circuit_gain = 14.358;
    float sense_resistor = 0.4;
    float est_amps = analog_input_mV / (circuit_gain*sense_resistor);

    return est_amps;
}