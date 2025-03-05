#include "shutter.h"
#include <Arduino.h>
#include <ESP32Servo.h>

int ServoPin = 17;
Servo pwm;
int freq = 50;


// DEFINITIONS
#define LDR_PIN 7 // GPIO2, pin 7
#define PWR_ENABLE 8 // TO ENABLE 5V
#define SHUTTER_PIN 9 // GPIO3, pin 9

long int last_fired_time = 0;
uint8_t ldrThreshold = 128;

bool high_power_enabled = false;

int high_power_timeout = 30000; // 30 seconds

void setup_shutter() {
    Serial.println("Setting up shutter");
    pinMode(LDR_PIN, INPUT);
    pinMode(PWR_ENABLE, OUTPUT);
    pinMode(SHUTTER_PIN, OUTPUT);

    digitalWrite(PWR_ENABLE, LOW);

    ESP32PWM::allocateTimer(0);
    pwm.setPeriodHertz(freq);
	pwm.attach(ServoPin, 1000, 2000); // 1KHz 10 bits

}

void loop_shutter() {
    if (high_power_enabled && millis() - last_fired_time > high_power_timeout) {
        // Go back to sleep
        digitalWrite(PWR_ENABLE, LOW);
        high_power_enabled = false;
    }
}

void enable_high_power() {
    Serial.println("Enabling high power");

    digitalWrite(PWR_ENABLE, HIGH);
    high_power_enabled = true;
    delay(100);
}

void fire_shutter() {
    enable_high_power();

    Serial.println("Firing Shutter");
    // analogWrite(SHUTTER_PIN, (2*255)/20);
    delay(1000);
    // analogWrite(SHUTTER_PIN, (1*255)/20);
    // pwm.write(180);
    delay(1000);

    pwm.write(0);
    delay(1000);
    pwm.write(180);
    delay(1000);
    pwm.write(0);
    last_fired_time = millis();
}

bool detectLight() {
    return (analogRead(LDR_PIN) > ldrThreshold);
  }
  
