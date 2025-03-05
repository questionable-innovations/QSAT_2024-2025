#include "radio.h"

#include <WiFi.h>

#include "ESPNowW.h"

#ifdef rocket
uint8_t mac[] = {0x36, 0x33, 0x33, 0x33, 0x33, 0x33};
uint8_t receiver_mac[] = {0x36, 0x33, 0x33, 0x33, 0x33, 0x32};
#else
uint8_t mac[] = {0x36, 0x33, 0x33, 0x33, 0x33, 0x32};
uint8_t receiver_mac[] = {0x36, 0x33, 0x33, 0x33, 0x33, 0x33};
#endif

unsigned long last_ping_sent = 0;
unsigned long last_ping_received = 0;

uint8_t isArmed = 0;

unsigned long disconnectTime = 10000;


void onPingReceived(const uint8_t *mac_addr, const uint8_t *data, int data_len);

void setup_radio() {
    WiFi.mode(WIFI_MODE_STA);
    ESPNow.set_mac(mac);
    WiFi.disconnect();
    ESPNow.init();
    ESPNow.add_peer(receiver_mac);
    ESPNow.reg_recv_cb(onPingReceived);
}

void send_ping() {
    Serial.println("Sending ping");
    // send ping
    ESPNow.send_message(receiver_mac, &isArmed, 1);  // 1 for armed, 0 for disarmed
}

void loop_radio() {
    if (millis() - last_ping_sent > 1000) {
        send_ping();
        last_ping_sent = millis();
    }

    if (last_ping_received > millis() + disconnectTime) {
        // no connection probably, arm
        Serial.println("No connection, exploding");
        isArmed = 1;
    }
}

void onPingReceived(const uint8_t *mac_addr, const uint8_t *data, int data_len) {
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

    // set isArmed
    isArmed = data[0];
    if (isArmed) {
        Serial.println("RECIEVED 1: System is armed.");
    } else {
        Serial.println("DID NOT RECIEVE 1: System is disarmed.");
    }

    last_ping_received = millis();
}
