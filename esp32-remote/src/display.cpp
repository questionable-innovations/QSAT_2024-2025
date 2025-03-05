#include "display.h"
#include "freefonts.h"

#include <SPI.h>
#include <TFT_eSPI.h>
#include <XPT2046_Touchscreen.h>
#include "widgets/connection.h"
#include "widgets/header.h"

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
    tft.setTextDatum(MC_DATUM);
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

void display_loop() {
  if (ts.tirqTouched() && ts.touched()) {
    TS_Point p = ts.getPoint();
    printTouchToSerial(p);
    delay(100);
  }
}

void display_update(DisplayUpdate update) {
    tft.fillScreen(TFT_BLACK);

    int last_height = draw_header(update.remoteState);    
}
