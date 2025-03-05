#include "film_control.h"
#include <cameras/film/servo_9g.h>
#include <definitions/pins.h>
#include <sense/current_sense.h>

int reload_center = 180;

int trigger_press_angle = 0;

Servo9G reload_servo(CamTrigServo_Reload, reload_center);
Servo9G trigger_servo(CamTrigServo_Trigger, reload_center);


void setup_servos() {
    reload_servo.setup();
    trigger_servo.setup();
}

int reload_started = 0;
int curr_reload_direction = Stationary;

int testing_stalled = 0;

void update_servos() {
    if (reload_started == 0) {
        return;
    }

    int timeout = 10000;
    if (reload_started + timeout < millis()) {
        reload_started = 0;
        Serial.println("\n######## ERR ########");
        Serial.print("Reload timeout, after");
        Serial.print(reload_started - millis());
        Serial.println("ms");
        Serial.println("####################\n");
        return;
    }

    if (curr_reload_direction == Stationary) {
        reload_set_direction(Forward);\
        curr_reload_direction = Forward;
        return;
    }

    // Check if stalled
    float reload_ma = read_reload_miliamps();
    int round_time = millis();
    
    Serial.print("Reload MA:");
    Serial.println(reload_ma);

    if (testing_stalled > 0) {
        

    }

    if (reload_ma > 450) {
        // Stalled

    }
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


