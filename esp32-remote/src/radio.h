#pragma once

void setup_radio();

void loop_radio();

void printMac();

int getSendingState(); // 0 = disarmed, 1 = armed, 2 = do not change
int getLastReceivedState(); // 0 = disarmed, 1 = armed
int getTimeSinceLastPing(); // recieved ping in milliseconds