#include <Arduino.h>
#include "radio.h"
#include "shutter.h"





// FUNCTION DEFINITIONS
bool detectLight(); 
void fireShutter(); // only activate if no pings for 10 seconds



// SETUP 
void setup() {
  Serial.begin(115200); // forgor serial baud rate maybe that's right lmao
  Serial.println("Starting up");
  setup_shutter();
  setup_radio();

}


// LOOP
void loop() {
  Serial.println("e :)");

  loop_radio();
}
