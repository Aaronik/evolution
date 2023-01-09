# WHAT WILL THIS BE?

Ecosystem Simulation

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

### ON HOW TO ENSURE THERE ARE ALWAYS SOME LIFEFORMS

An LF can self replicate?
It can self replicate at a rate that is inversely relative to how many LFs there are on the board?
Maybe if there's a minimum threshold of LFs, then the fittest ones self replicate with some higher
than usual mutation rate!??
Maybe there's limited food and water?
Maybe they can kill each other, but they lose health when they do? That way it's balanced when it
gets overcrowded it becomes advantageous to kill?

## PHYSICAL LIMITS ON EACH LIFEFORM

There have to be physical limits based on each lifeform's stats, too.

* The stronger, the slower?
-- OR --
* Being more well fed makes them slower
* Being thirsty makes them slower

Do we even have speed? It'd be easier if they could just move one space per turn

* When health hits 0, LF dies (leaving food behind?)
* Health degrades at inverse square from distance to danger plus a constant rate
* Mating increases health at a rate proportional to the health of the other LF
* Hunger increases rate of health drop
* Thirst sharply increases rate of health drop, -- or -- does it inhibit clarity of input neurons?

## INPUT NEURONS
* Direction to food
* Direction to water
* Direction to healthiest LF
* Direction to closest LF
* Direction to danger
* Healthiest LF's health
* Closest LF's health
* LF's health
* LF's hunger
* LF's thirst
* Total number of LFs

## OUTPUT NEURONS
* Move up
* Move down
* Move right
* Move left
*
