#include "film_control.h"
#include <cameras/film/servo_9g.h>
#include <definitions/pins.h>

int reload_center = 0;

int trigger_press_angle = 90;

Servo9G reload_servo(CamTrigServo_Reload, reload_center);
Servo9G trigger_servo(CamTrigServo_Trigger, reload_center);


void setup_servos() {
    reload_servo.setup();
    trigger_servo.setup();
}

int reload_started = 0;
int curr_reload_direction = Stationary;

bool testing_stalled = false;

void update_servos() {
    if (reload_started == 0) {
        return;
    }

    if (curr_reload_direction == Stationary) {
        reload_set_direction(Forward);
    }
    // Check if stalled
}


void trigger_press(long duration) {
    trigger_servo.angle_set(trigger_press_angle);
    delay(1000);
    trigger_servo.angle_set(reload_center);
}

void request_reload() {
    reload_started = millis();
}

bool reload_complete() {
    return reload_started == 0;
}

void reload_set_direction(Direction direction) {
    if (direction == Forward) {
        reload_servo.angle_set(180);
    } else if (direction == Backward) {
        reload_servo.angle_set(0);
    } else {
        reload_servo.angle_set(reload_center);
    }
}


