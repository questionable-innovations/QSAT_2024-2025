#pragma once

#include <TinyGPSPlus.h>
#include <SoftwareSerial.h>
#include <definitions/pins.h>

void setupGps();
void gpsLoop();