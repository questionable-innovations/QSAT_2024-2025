# Psat PSU PCB v2
The PSU PCB provides 1.8V, 3.3V, 5V rails and a raw battery voltage rail. The various supply rails can be always on, or controlled by software (see Functionality). USB-C and micro-USB connectors are present to charge the battery using a standard USB wall charger.
The supply rail voltages can be adjusted by changing the feedback resistor values. See the schematic for details.

## Functionality
### Controlling the Supply Rails
The PSU board offers two different control modes for each supply rail, controlled by the switches the S1 component: 
In the “ON” mode the relevant supply is always on. This is the simplest operating mode and no interaction with the enable pins are necessary. In this mode the enable pin is connected to Vraw via a 1.5k current-limiting resistor.

In the “OFF / SW control” mode, the supply defaults to off (pulled down by a 1Megohm resistor) but a microcontroller can enable the supply by pulling the enable pin high. If you want to programmatically enable a supply during operation you need to use this mode and connect your microcontroller to the enable pins on the header. See ‘Connections’ for the exact voltage(s) required to enable each supply.


*Note: If you configure S1 into the “ON” mode and have also connected the enable pin to a microcontroller, make sure to leave that GPIO pin as an input pin, as otherwise you will waste power through the 1.5k resistor.*

### USB charging
The board is capable of charging a lithium-ion battery by connecting a USB charger.
The two USB connectors (USB-C and micro-USB) are provided for convenience. DO NOT CHARGE USING BOTH CONNECTORS AT ONCE.
The battery will charge up to 800mA (USB power supply willing). This rate can be decreased by changing the value of R6, according to the datasheet of the battery charger.

### Disable while charging
The final S1 switch (labelled '*' on the PCB) is the 'supply disabled while charging' switch. It determines whether all the supply rails (including Vraw) are available while a USB cable is plugged in. The battery will of course charge faster if the load is disabled during charging. The BQ21040 battery charger IC states that it can also power loads during charging, so long as the charging time does not exceed 10 hours as a result, but this is not universally true for all charging ICs. When this switch is set to "ON" the load is disabled during charging. When it is set to "SW control" the supplies are enabled even while plugged in.

### 'Attach Before Flight’ Connector
ABF is a connector that controls whether the battery is connected to the PSU and stack. When the jumper is removed the entire stack has no power. When a jumper is placed across ABF (shorting the two pins) the battery is connected to the PSU and hence the stack. It is placed at a right-angle so that the jumper may be inserted prior to launch through a hole in the exterior PSat shell.

## Connections
Header H1:

 - 1V8 -  1.8V step-down supply rail, capable of 2A output, battery not withstanding.
 - 3V3 -  3.3V step-down supply rail, capable of 2A output, battery not withstanding. *
 - 5V  -  5V step-up supply rail, capable of 2A output, battery not withstanding.
 - GND -  Common ground for all supply rails.
 - Vraw - Raw battery voltage. For a Li-ion battery this is between ~4.2V and ~3V.
 - 1V8_PG - Power good indicator for the 1.8V rail. Open-drain output: While the output voltage is more than 20% away from the regulation level this pin is pulled low, otherwise it is floating - you should attach this to a voltage source via a pullup resistor of your choosing.
 - 3V3_PG - Power good indicator for the 3.3V rail. Open-drain output: While the output voltage is more than 20% away from the regulation level this pin is pulled low,  otherwise it is floating - you should attach this to a voltage source via a pullup resistor of your choosing.
 - EN_1V8 - This pin may be pulled up above 1.5V to enable the 1.8V supply. The pin has a 1 Megohm pulldown resistor attached. When the equivalent S1 switch is in the "ON" position a 1.5k pullup resistor pulls this pin up to Vraw.
 - EN_3V3 - This pin may be pulled up above 1.5V to enable the 3.3V supply. The pin has a 1 Megohm pulldown resistor attached. When the equivalent S1 switch is in the "ON" position a 1.5k pullup resistor pulls this pin up to Vraw.
 - EN_5V - This pin may be pulled up above 1.2V to enable the 5V supply. The pin has a 1 Megohm pulldown resistor attached. When the equivalent S1 switch is in the "ON" position a 1.5k pullup resistor pulls this pin up to Vraw.

 \* *Note: This voltage is only stepped-down from the battery voltage, so if that goes below 3.3V this supply rail will as well. This means that across a full battery discharge this rail is nominally between 3.3V - 3.0V.*

## PCB Stacking
The PSat PSU is designed to be used in conjunction with either a single payload PCB (i.e. ‘2 PCB stack’) or in a stack of multiple PCBs (‘Multi-PCB stack’).

There are three types of 2.54mm pitch headers available in the lab, and which the PCB was designed with in mind:
Male headers with a 6mm mating length, 2.54mm insulation and a 3mm post.
Female headers with 8.5mm insulation and a 3mm post.
'Arduino female stacking headers' with 8.5mm of insulation, and 10.5mm posts.
### 2 PCB Stack
The simplest stack uses the supplied female header on the payload PSU and regular male headers on the payload board, pointed down. This produces a stack height of about 5mm.

The PSU is designed to go at the bottom of the stack (hence why the example payload has a cutout for the battery), but it can be moved up in the stack provided the battery can fit in the remaining space.

If a larger stack height is needed the standard female headers can be used to grant either ~8.5mm or ~11mm stack height (depending on whether the male pins are soldered with the plastic insulation between the PCBs or on the other side).

The stack height can be eliminated entirely if the male header is soldered such that the insulation is not between the two PCBs. Note that this would require the PSU PCB to be flipped (reversing the order of the header H1), as the underside of the PSU PCB must be used to mate against the payload (as no components on this side to interfere).

### Multi-PCB stack
The simplest stack uses the female stacking headers for each layer, except for the bottom which can use a standard female header, and the top which can use a standard male header (pointing down). This enforces a board-to-board stack height of 11mm between payload boards, and a stack height of around 5mm between the PSU board and the first payload board.

## Future changes
Maybe add a resettable fuse between the two USB connector’s bus voltages?

Maybe add a polyfuse to battery and each rail’s outputs

## Version History + Changelog
Version 2 - Not yet printed.
Move away from a centralised, fixed-size stack header to a flexible ‘only-what-is-necessary’ header: Rather than using a single 30-pin header with many unused pins, we use only a 10-pin header because the PSU PCB only needs 10 pins. Removed unused header pins as a result. PCBs further above in the stack can add additional headers adjacent to ours if necessary. 
Also switched to 2.54mm header due to a lack of 1.27mm ‘stacking’ headers. 2.54mm headers are also easier to route around, which actually makes it easier to add more pins to other PCBs in the stack if necessary, even though they’re bigger. 
Fixed Q1 by swapping pins 2 and 3. The mosfet body diode allowed Vbat to power the load even when the mosfet was (supposed to be) acting as an open circuit.
Added a resistor between Vraw and the EN_XYZ pins to prevent possible short circuit with microcontroller if switch is improperly configured
