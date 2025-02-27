#include "film_control.h"
#include <cameras/film/servo_9g.h>
#include <definitions/pins.h>

int reload_center = 0;

Servo9G reload_servo(CamTrigServo_Reload, reload_center);


void setup_servos() {
}

void update_servos() {

}

void trigger_press(long duration) {
}

void reload_set_direction(Direction direction) {
}
