# ERRATA
## 'Unknown device' when using 2-wire programming
Despite being the value recommended in the datasheet, C5 being 1nF is sometimes too high to allow programming when using 2-wire mode. Reducing C5 to 470pF solved the issue.

Affects: All versions

Fixed in version: TBD

## Reset pullup resistor value
Pullup resistor R1 is wrongly valued at 4.7k, the datasheet recommends 47k. No adverse affects noted. 

Affects v2.0.0 - v2.0.2

Fixed in: v2.0.3


