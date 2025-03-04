#include <Arduino.h>
#include <leds.h>
#include "cameras/film/film_control.h"
#include "sense/light.h"

#include <SPI.h>
#include <LoRa.h>
#include "sense/current_sense.h"

bool curr_state;

void setup()
{
  Serial.begin(9600);
  Serial.println("Starting up");
  led_setup();
  setup_servos();
  setup_current_sense();
  // setupGps();
  curr_state = sense_light();
}


void loop() {
  led_loop();

  bool new_state = sense_light();
  Serial.println(new_state);

  if (new_state != curr_state) {
    curr_state = new_state;
    if (!curr_state) {
      Serial.println("Pressing trigger");
      trigger_press(1000);
    }
  }


}