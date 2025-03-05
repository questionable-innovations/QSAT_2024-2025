#pragma once

#include <cstdint>

enum ArmState {
    Armed,
    Disarmed,
    Neutral,
    Disconnected
};

struct DisplayUpdate {
    ArmState localState;
    ArmState remoteState;
    unsigned long lastUpdateMillis;
    int lightFlux;
    float localRssi;
};

uint16_t createRGB(int r, int g, int b);
void convert_state_to_string(ArmState state, char* buffer);
void convert_state_to_color(ArmState state, uint16_t* color);