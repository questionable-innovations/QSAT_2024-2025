#pragma once

void setup_servos();

void trigger_press(long duration);


enum Direction {
  Backward,
  Stationary,
  Forward
};


void reload_set_direction(Direction direction);

bool reload_complete();