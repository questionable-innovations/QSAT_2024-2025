#pragma once
#include "state.h"

enum Direction {
    Left,
    Right
};

int draw_button(ArmState state, bool selected, Direction direction, int start_height);