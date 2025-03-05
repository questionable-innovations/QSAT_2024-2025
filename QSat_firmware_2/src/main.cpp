#include <Arduino.h>
#include "radio.h"
#include "shutter.h"


// SETUP 
void setup() {
  Serial.begin(115200); // forgor serial baud rate maybe that's right lmao
  Serial.println("Starting up");
  setup_shutter();
  setup_radio();
}


// LOOP
void loop() {
  fire_shutter();
  loop_shutter();
  
  
  loop_radio();
}
