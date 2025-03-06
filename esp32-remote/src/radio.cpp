#include "radio.h"
#include "ESPNowW.h"

#include <WiFi.h>
#include <ArduinoJson.h>

uint8_t mac[6];
// uint8_t receiver_mac[] = {0xcc, 0x8d, 0xa2, 0x8d, 0x28, 0x2c}; // TAINE'S TEST ESP 1
uint8_t receiver_mac[] = {0xcc, 0x8d, 0xa2, 0x8d, 0x07, 0xa8}; // TAINE'S TEST ESP 2
// uint8_t receiver_mac[] = {} // CYD

unsigned long last_ping_sent = 0;
unsigned long last_ping_received = 0;

uint8_t sendingState = 2; // 0 = disarmed, 1 = armed, 2 = do not change
uint8_t lastReceivedState = 0; // 0 = disarmed, 1 = armed

unsigned long disconnectTime = 30000; // time before auto-arm

void onMsgReceived(const uint8_t *mac_addr, const uint8_t *data, int data_len);

void setup_radio() {
    WiFi.mode(WIFI_MODE_STA);

    // get mac address
    esp_read_mac(mac, ESP_MAC_WIFI_STA);
    ESPNow.set_mac(mac);

    WiFi.disconnect();
    ESPNow.init();
    ESPNow.add_peer(receiver_mac);
    ESPNow.reg_recv_cb(onMsgReceived);
}

void send_ping() {
    Serial.println("Sending ping");

    printMac();

    JsonDocument jsonDoc;
    if (sendingState != 2) {
        jsonDoc["isArmed"] = sendingState;
    } else {
        jsonDoc["ping"] = true;
    }
    char json[200];
    serializeJson(jsonDoc, json);

    ESPNow.send_message(receiver_mac, (uint8_t *)json, strlen(json));
}

void printMac()
{
    Serial.print("MAC Address: ");
    for (int i = 0; i < 6; i++)
    {
        Serial.printf("%02x", mac[i]);
        if (i < 5)
        {
            Serial.print(":");
        }
    }
}

void loop_radio() {
    if (millis() - last_ping_sent > 1000) {
        send_ping();
        last_ping_sent = millis();
    }    
}

void onMsgReceived(const uint8_t *mac_addr, const uint8_t *data, int data_len) {
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
    // ==========================
    
    // real code
    JsonDocument jsonDoc;
    DeserializationError error = deserializeJson(jsonDoc, data, data_len);
    if (error) {
        Serial.print("deserializeJson() failed: ");
        Serial.println(error.c_str());
        return;
    }

    // print out the json
    serializeJsonPretty(jsonDoc, Serial);
    Serial.println("");

    if (jsonDoc.containsKey("isArmed")) {
        lastReceivedState = jsonDoc["isArmed"];
    }

    last_ping_received = millis();
}

int getSendingState() {
    return sendingState;
}

int getTimeSinceLastPing() {
    return millis() - last_ping_received;
}

int getLastReceivedState() {
    return lastReceivedState;
}