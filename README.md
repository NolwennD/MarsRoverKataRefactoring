# Mars rover kata refactoring

Rules are based on Codurance description (https://katalyst.codurance.com/mars-rover)

## Short version of rules

You have to move a rover on a grid (10x10 by default). The rover have a
starting position and face to a direction (North, East, South, West). It understands three commands:
 * turn right
 * turn left
 * move forward
 
 If the rover reach grid limits, it wraps around the end of the grid.
 
### Inputs and outputs

Given a rover starting from (0,0) and facing North.

Input `MR` gives `0:1:E`

With the same rover and with an obstacle on (0,1).

Input `MR` gives `O:0:0:N` we get the last possible position and facing.
