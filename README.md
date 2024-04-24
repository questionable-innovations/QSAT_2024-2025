# APSS Reference PCBs
This is a collection of reference PCBs and Altium libraries for use within APSS.
The PSat folder includes a bunch of reference PCBs for use in PSat payloads.
The shared libraries folder includes a list of standardised parts for use within APSS. Many of these parts are available in the lab for breadboarding. You may of course use other components, but you will have to wait until they arrive before testing. 

## PSat Reference PCBs
A small collection of compatible PCBs and schematics for making a PSat payload with. See the separate readme in the folder for more details.

## APSS Altium Libraries
The libraries are split into flight-ready parts and prototyping parts. Prototyping parts are generally good parts that are cost-optimised, may not be automotive/military rated, and are not screened for CMOS latchups. These parts are useful for any non-critical design projects like PSat or ElevatorSat. 

Flight-ready parts are generally the highest performance parts that can be obtained for the role. They are automotive rated (or at least rated to automotive-like temperatures), checked for non-CMOS alternatives, etc. These should be used only for satellite-rated hardware like cubesat payloads, as some of these parts cost up to $20 each.

A General Components library also contains components like generic resistors, test points and pads, solder jumpers, etc.

## APSS Breadboard Adapters
A collection of small PCBs to use SMT parts on a breadboard.
