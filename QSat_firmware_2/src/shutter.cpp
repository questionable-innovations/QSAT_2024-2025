#include "shutter.h"
#include <Arduino.h>
#include <ESP32Servo.h>

int ServoPin = 13;
ESP32PWM pwm;
int freq = 1000;


// DEFINITIONS
#define LDR_PIN 7 // GPIO2, pin 7
#define PWR_ENABLE 8 // TO ENABLE 5V
#define SHUTTER_PIN 9 // GPIO3, pin 9

long int last_fired_time = 0;
uint8_t ldrThreshold = 128;

void setup_shutter() {
    pinMode(LDR_PIN, INPUT);
    pinMode(PWR_ENABLE, OUTPUT);
    pinMode(SHUTTER_PIN, OUTPUT);

    ESP32PWM::allocateTimer(0);
	pwm.attachPin(ServoPin, freq, 10); // 1KHz 10 bits

}

void loop_shutter() {
}

void fire_shutter() {
    pwm.writeScaled(0);
    delay(1000);
    pwm.writeScaled(1);
    delay(1000);
    pwm.writeScaled(0);
    last_fired_time = millis();
}

bool detectLight() {
    return (analogRead(LDR_PIN) > ldrThreshold);
  }
  
