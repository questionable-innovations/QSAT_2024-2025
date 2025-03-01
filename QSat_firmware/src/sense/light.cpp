#include "light.h"
#include <Arduino.h>
#include <definitions/pins.h>

int threshold = 60;

void setup_light_sense() {
    pinMode(LightSense, INPUT);
}

bool sense_light() {
    int read = analogRead(LightSense);
    Serial.print("Light: ");
    Serial.println(read);
    return read > threshold;
}
