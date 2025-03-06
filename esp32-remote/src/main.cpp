#include <Arduino.h>
#include "display.h"
#include "radio.h"
#include "widgets/state.h"

void setup() {
  Serial.begin(115200);
  Serial.println("Booting display...");
  display_init();
  setup_radio();
  startup_animation();
  display_update({
    .localState = Disarmed,
    .remoteState = Armed,
    .lastUpdateMillis = millis(),
    .lightFlux = 172,
    .localRssi = -52.2
  });
  Serial.println("Display booted.");
}

// Don't block/delay on main thread, it'll break touch-responsiveness 
void loop() {
  display_loop();
  loop_radio(); 
}