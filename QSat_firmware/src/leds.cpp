#include "leds.h"
#include "definitions/pins.h"
#include "sense/light.h"

void led_setup() {
    pinMode(StatusLED1, OUTPUT);
    pinMode(StatusLED2, OUTPUT);
    pinMode(StatusLED3, OUTPUT);
    digitalWrite(StatusLED1, HIGH);
    digitalWrite(StatusLED2, HIGH);
    digitalWrite(StatusLED3, HIGH);
}

void led_loop() {
    digitalWrite(StatusLED1, sense_light());
    digitalWrite(StatusLED2, HIGH);
}
