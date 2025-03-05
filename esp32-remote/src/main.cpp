#include <Arduino.h>
#include "display.h"
#include "widgets/state.h"

void setup() {
  Serial.begin(115200);
  Serial.println("Booting display...");
  display_init();
  startup_animation();
  display_update({Armed, Disarmed, 0, 0, 0});
}

// Don't block/delay on main thread, it'll break touch-responsiveness 
void loop() {
  display_loop();
  
}