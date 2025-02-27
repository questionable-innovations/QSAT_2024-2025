#pragma once
#include <Energia.h>

class Servo9G {
   public:
    Servo9G(int pin, int inital_angle);
    void setup();
    void angle_set(int angle);

   private:
    int pin;
    int oldDuty;
};