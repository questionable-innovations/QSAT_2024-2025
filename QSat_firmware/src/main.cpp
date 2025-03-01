#include <Arduino.h>
#include <leds.h>

#include <SPI.h>
#include <LoRa.h>

long unsigned int currentValue = 0;

long resolution = 255;

void setup()
{
  Serial.begin(9600);

  pinMode(P2_0, OUTPUT);
  analogFrequency(50);
  analogResolution(resolution);
  led_setup();
}

int old_duty = 0;

void angle_set(int pin, int angle)
{
  angle = max(0, min(180, angle));
  float active_time = 1.0 / 20.0 + (angle / 180.0) * 1.0 / 20.0;
  // float active_time = (angle / 180.0);
  int duty = (int)(active_time * resolution);

  if (duty == old_duty)
  {
    return;
  }
  old_duty = duty;
  analogWrite(pin, (int)(duty));
}

void loop()
{
  led_loop();
  // put your main code here, to run repeatedly:
  int angle = currentValue % 180;
  Serial.print("Angle: ");
  // Serial.print(digitalPinToTimer(P2_0));
  Serial.println(angle);
  // angle_set(P1_6, angle);
  angle_set(P2_0, angle);

  currentValue += 1;
  delay(10);
}