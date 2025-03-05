#include <Arduino.h>
#include <WiFi.h>
#include "ESPNowW.h"

// DEFINITIONS
#define LDR_PIN 7 // GPIO2, pin 7
#define PWR_ENABLE 8 // TO ENABLE 5V

#ifdef sender
uint8_t mac[] = {0x36, 0x33, 0x33, 0x33, 0x33, 0x33};
uint8_t receiver_mac[] = {0x36, 0x33, 0x33, 0x33, 0x33, 0x32};
#else
uint8_t mac[] = {0x36, 0x33, 0x33, 0x33, 0x33, 0x32};
uint8_t receiver_mac[] = {0x36, 0x33, 0x33, 0x33, 0x33, 0x33};
#endif

unsigned long lastPing = millis();
unsigned long disconnectTime = 10000;
uint8_t ldrThreshold = 128;


// FUNCTION DEFINITIONS
bool detectLight(); 
void fireShutter(); // only activate if no pings for 10 seconds
void sendPing(); // both ESPs should send 

void onPingRecieved(); // update lastPing


// SETUP 
void setup() {
  Serial.begin(115200); // forgor serial baud rate maybe that's right lmao

  pinMode(LDR_PIN, INPUT);
  pinMode(PWR_ENABLE, OUTPUT);

  WiFi.mode(WIFI_MODE_STA);
  ESPNow.set_mac(mac);
  WiFi.disconnect();
  ESPNow.init();
  ESPNow.add_peer(receiver_mac);
  ESPNow.reg_recv_cb(onPingRecieved);
}


// LOOP
void loop() {
  delay(1000);

  // send ping
  uint8_t ping = 1;
  ESPNow.send_message(receiver_mac, &ping, 1);

  if (lastPing > millis() + disconnectTime) {
    // no connection probably, explode
  }
}


// FUNCTIONS
bool detectLight() {
  return (analogRead(LDR_PIN) > ldrThreshold);
}

void onPingRecieved(const uint8_t *mac_addr, const uint8_t *data, int data_len) {

  // bs start
  char macStr[18];
  snprintf(macStr, sizeof(macStr), "%02x:%02x:%02x:%02x:%02x:%02x",
           mac_addr[0], mac_addr[1], mac_addr[2], mac_addr[3], mac_addr[4],
           mac_addr[5]);
  Serial.print("Last Packet Recv from: ");
  Serial.println(macStr);
  Serial.print("Last Packet Recv Data: ");
  // if it could be a string, print as one
  if (data[data_len - 1] == 0)
      Serial.printf("%s\n", data);
  // additionally print as hex
  for (int i = 0; i < data_len; i++) {
      Serial.printf("%x ", data[i]);
  }
  Serial.println("");
  // bs finish

  lastPing = millis();
}
