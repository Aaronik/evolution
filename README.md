# WHAT WILL THIS BE?

Ecosystem Simulation

Based off of this amazing youtube video: https://youtu.be/N3tRFayqVtk?t=1433

I want to make a more complex evolutionary thing, using individual little entities that each
have a neural net that evolves to create their behavior

I'd love for it to not be frame based evolution, but evolution in real time.

So there's a 2d world, and I'm envisioning it has maybe 10 lifeforms.
There's food, which is in one part of the board, and there's water, which is in another.
There should also be a hazard, like a sink hole, or a radioactive thing.

Maybe lifeforms grow stronger as they eat and drink, and weaker as time passes
Maybe lifeforms can consume each other?

Maybe lifeforms get a boost of some kind when they mate, and they can only mate when they are
well fed and well hydrated. Maybe it's a health boost, and they lose health when they come in
contact with the danger. Or -- the danger is radioactive and extends across the whole board,
and sucks their health down.

### ON HOW TO ENSURE A BOUNDED QUANTITY OF LIFEFORMS

* If there's a minimum threshold of LFs, the LFs self replicate with some higher degree of mutation
    (So in the beginning, it's a lot of different random LFs, until some start actually working
    with the physics)
* There's limited food and water (So if there's too much, the LFs won't all be able to survive)
* LF's can attack other LFs (Ensuring it's not random survival based on who's closest to food and
  water)

### PHYSICS

* When health hits 0, LF dies (leaving food behind?)
* Health degrades at inverse square from distance to danger plus a constant rate
* Mating increases health at a rate proportional to the health of the other LF?
    * Mating increases hunger
* Hunger increases rate of health drop
* Thirst inhibits clarity of input neurons
* Attacking halves both LFs health or something

* LF can only eat if adjacent to food
* LF can only drink if adjacent to water
* LF can only attack if adjacent to LF
* LF can only mate if adjacent to LF

* LF can only mate so often (to prevent mating clumps around the food)
* If LF tries to mate next to multiple LFs, the LF with the lowest health will be mated with (To prevent orgy clumps (local minima))

* ~~Moving increases the LF's hunger and thirst a small amount~~ (Don't want to optimize for boring)

### INPUT NEURONS
* Direction to food
* Distance to food
* Direction to water
* Distance to water
* Direction to danger
* Distance to danger
* Direction to healthiest LF
* Direction to closest LF
* Healthiest LF's health
* Closest LF's health
* LF's health
* LF's hunger
* LF's thirst
* Total number of LFs
* Number of LFs adjacent to LF
* Random
* Oscillator

### OUTPUT NEURONS
* Move up
* Move down
* Move right
* Move left
* Attack
* Mate
* Eat
* Drink


## Program Features

* Have another SQLite DB that allows for a single board to pause and continue its evolution
* Have a visual representation of the board
* Have a graph next to it that shows stats for specific LFs?
* It'd be nice to have multiple threads working together to render each frame, rather than each working on their own board

## Organization

At every frame, we go through each LF's input neurons and give them their values. Then we do the calculation phase, which is as follows:

Each internal neuron sums up the weights of the neurons connecting to it, then runs tanh (hyporbolic tangent) on that sum to get it between -1.0 and 1.0.
Each output neuron does the same

If the output neuron ends up being b/t 0 and 1, use that as a likelihood of firing (so 0.2 would be a 20% chance of firing), where firing means performing the effect.

