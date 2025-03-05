#include <Arduino.h>
#include "display.h"


void setup() {
  Serial.begin(115200);
  display_init();
  Serial.println("Booting display...");
}

// Don't block/delay on main thread, it'll break touch-responsiveness 
void loop() {
  // put your main code here, to run repeatedly:
  Serial.println("Display loop");
  
  display_loop();
}