#include "servo_9g.h"

const int resolution = 255;

Servo9G::Servo9G(int pin, int inital_angle) {
    this->pin = pin;
    this->oldDuty = inital_angle;
}

void Servo9G::setup() {
    pinMode(this->pin, OUTPUT);
    analogFrequency(50);
    analogResolution(resolution);
}

void Servo9G::angle_set(int angle) {
    angle = max(0, min(180, angle));

    // 1/20 is 0deg, 1/10 is 180deg (theoretically)
    float active_time = 1.0 / 20.0 + (angle / 180.0) * 1.0 / 20.0;
    int duty = (int)(active_time * resolution);

    // Writing the same value to the servo will cause it to jitter
    if (duty == this->oldDuty) {
        return;
    }

    this->oldDuty = duty;
    analogWrite(pin, (int)(duty));
}
