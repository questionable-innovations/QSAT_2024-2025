#include <Arduino.h>
#include "radio.h"

// DEFINITIONS
#define LDR_PIN 7 // GPIO2, pin 7
#define PWR_ENABLE 8 // TO ENABLE 5V


unsigned long lastPing = millis();
uint8_t ldrThreshold = 128;


// FUNCTION DEFINITIONS
bool detectLight(); 
void fireShutter(); // only activate if no pings for 10 seconds
void sendPing(); // both ESPs should send 



// SETUP 
void setup() {
  Serial.begin(115200); // forgor serial baud rate maybe that's right lmao

  pinMode(LDR_PIN, INPUT);
  pinMode(PWR_ENABLE, OUTPUT);

  setup_radio();

}


// LOOP
void loop() {
  loop_radio();
}


// FUNCTIONS
bool detectLight() {
  return (analogRead(LDR_PIN) > ldrThreshold);
}

