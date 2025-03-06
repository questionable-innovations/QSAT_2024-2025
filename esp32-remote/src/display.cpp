#include "display.h"
#include "freefonts.h"

#include <SPI.h>
#include <TFT_eSPI.h>
#include <XPT2046_Touchscreen.h>

#include "widgets/connection.h"
#include "widgets/header.h"
#include "widgets/line.h"
#include "widgets/hr.h"
#include "widgets/arm_button.h"

void (*armCallback)(void);
void (*disarmCallback)(void);

int buttonStart = 0;
int buttonEnd = 0;


void part_update_seconds();

void display_init() {
    mySpi.begin(XPT2046_CLK, XPT2046_MISO, XPT2046_MOSI, XPT2046_CS);
    ts.begin(mySpi);
    ts.setRotation(1);
    

    // Start the tft display and set it to black
    tft.init();
    tft.setRotation(0);	 // This is the display in landscape

    // Clear the screen before writing to it
    tft.fillScreen(TFT_BLACK);

    int x = 320 / 2;  // center of display
    int y = 100;
    int fontSize = 2;
}

void startup_animation() {
    Serial.println("Starting up display...");

    tft.fillScreen(TFT_BLACK);
    tft.setTextColor(TFT_WHITE, TFT_BLACK, true);
    tft.setTextDatum(CC_DATUM);
    tft.setFreeFont(FSSB24);                 // Select the font
    tft.drawCentreString("QSAT", width/2 , height/2, GFXFF);
    delay(2000);
    Serial.println("Ending...");


}

void printTouchToSerial(TS_Point p) {
  Serial.print("Pressure = ");
  Serial.print(p.z);
  Serial.print(", x = ");
  Serial.print(p.x);
  Serial.print(", y = ");
  Serial.print(p.y);
  Serial.println();
}

long int last_update_ms = 0;
DisplayUpdate last_update = {Neutral, Neutral, 0, 0, 0};

int last_update_seconds = 0;

void display_loop() {
  if (ts.tirqTouched() && ts.touched()) {
    TS_Point p = ts.getPoint();
    printTouchToSerial(p);

    if (p.y > buttonStart && p.y < buttonEnd) {
      if (p.x < width/2) {
        // Disarm
        if (disarmCallback == 0) {
          Serial.println("Disarm callback is null, skipping");
        } else {
          Serial.println("Disarm callback pressed, running");
          disarmCallback();
        }
      } else {
        // Arm
        if (armCallback == 0) {
          Serial.println("Arm callback is null, skipping");
        } else {
          Serial.println("Arm callback pressed, running");
          armCallback();
        }

      }
    }
  }

  if (millis() - last_update_ms > 100) {
    part_update_seconds();
  }
}

void part_update_seconds() {
  if (last_update.lastUpdateMillis == 0) {
    return;
  }

  int seconds_since_last_ping = (millis() - last_update.lastUpdateMillis) / 1000;
  if (seconds_since_last_ping != last_update_seconds) {
    Serial.print("Seconds since last ping: ");
    Serial.println(seconds_since_last_ping);
    last_update_seconds = seconds_since_last_ping;
    display_update(last_update);
  }
}

void display_update(DisplayUpdate update) {
    last_update = update;
    last_update_ms = millis();
    
    // tft.fillScreen(TFT_BLACK);

    int last_height = draw_header(update.remoteState);
    
    // Local State
    char localState[24];
    convert_state_to_string(update.localState, (localState));
    last_height = draw_hr(last_height);
    last_height = draw_line("Local State", localState, last_height, update.localState);


    // Calculate last ping time
    last_height = draw_hr(last_height);
    if (update.lastUpdateMillis == 0) {
      last_height = draw_line("Last Ping", "N/A", last_height, Disconnected);
    } else {
      char last_ping[24];
      int seconds_since_last_ping = (millis() - last_update.lastUpdateMillis) / 1000;
      sprintf(last_ping, "%ds", seconds_since_last_ping);
      last_height = draw_line("Last Ping", last_ping, last_height, Neutral);
    }

    // LIGHT FLUX
    char lightFlux[24];
    sprintf(lightFlux, "%d", update.lightFlux);

    last_height = draw_hr(last_height);
    last_height = draw_line("Light Flux", lightFlux, last_height, Neutral);
    
    // Signal Strength
    char signalStrength[24];
    sprintf(signalStrength, "%.1f", update.localRssi);

    last_height = draw_hr(last_height);
    last_height = draw_line("Signal Strength", signalStrength, last_height, Neutral);
    
    
    last_height = draw_hr(last_height);

    buttonStart = last_height - standard_margin;
    draw_button(update.localState, update.localState == Disarmed, Left, last_height);
    last_height = draw_button(update.remoteState, update.localState == Armed, Right, last_height);
    buttonEnd = last_height + standard_margin;

}
