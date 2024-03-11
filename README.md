Versioning notes:
ver X.Y.Z means version X, revision Y, patch Z.

The version X relates to both electrical and physical compatibility. Boards that share a major version X shall be physically and electrically compatible with eachother. If a change prevents a board from either physically mating (e.g. changing stack header location) or electrically interoperating (e.g. changing the location of the stack's ground pin) with other boards then this must be associated with an increase in version number.

The revision number Y relates to software-level changes. If a change does not affect physical or electrical interoperability but does require changing software (e.g. the location of two control pins have been swapped) then this must be associated with an increase in the revision number.

The patch number Z relates to internal bug fixes or additions to functionality. An increase in a board's patch number must be completely transparent to the wider stack, only adding additional functionality or solving issues/bugs internal to the board. Any already working stack shall continue to function (physically, electrically, and programmatically) even if one board receives a patch.

To summarise: Only boards that share a version number shall be connected together. If they also share their revision number then no changes to existing software shall be required. Boards with the same version but different revision are compatible, but shall require changes to software. Patches shall have no effect on compatibility.

Examples:

PSU v1.0.0 <=/=> MCU v2.0.0

MCU v2.0.0 <=> Beacon v2.1.0, with software changes

MCU v2.2.0 <=> Beacon v2.2.37, with no software changes
