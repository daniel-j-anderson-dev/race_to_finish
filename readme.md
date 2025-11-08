# Race to finish MVP

## Summary


## Definitions
1) `Race`
    1) Starts by spawning a `Player` and a `Goal` entity
    2) Ends when there is a `Collision` between the `Player` and the `Goal`

2) `Round`
    1) consists of
        1) certain number of `Race`s and a time limit to complete them in
        2) a time limit to complete all the `Race`s in

3) `Collision`, `Collide`
    1) When the rectangle's of any two `Hitbox`es overlap

4) `Hitbox`
    1) a rectangle used to detect `Collision`s

5) `Entity`, `Entities`
    1) Has a `Hitbox`
        1) must be square
        2) the side length of must be proportional to the smallest screen dimension
    2) spawns in a random position at the start of a `Race`
        1) must spawn entirely on screen
        2) must not `Collide` with any other `Entities`

6) `Player`
    1) An `Entity` whose 2D position can be controlled with keyboard input

7) `Goal`
    1) An `Entity` that will end a `Race` when it `Collides` with a `Player`
