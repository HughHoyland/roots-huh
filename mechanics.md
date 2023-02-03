
# Basics

## Resources

The plant consumes:
* water - from soil, quickly replenishable. Comes from creeks, 
  a bit from surface, and certain underground layers. In default scenario, 
  water in those never ends.
* nitro - slowly replenishable. Filters down from surface.
* sunlight - depends on the size of the neighbors, and the climate.

The least of these resources defines the mass a plant grows by each tick.

## Growth and death mechanic

The mass is then distributed into the root (upper part is abstracted away so far),
proportionally to amount of resource that each branch provides.

Implement later: weighted by the particular resource demand. This means - if 
we need water twice as much as nitro, the branch that brings water will grow
twice as much as the branch that brings the same amount of nitro.

A soil is a field with certain concentrations of nitro and water, represented 
by blue triangles and brown rectangles.

Each part of the root pulls water/nitro surrounding it proportionally to its area and 
the concentration (richness) of the soil in this particular resource.

TODO: Each branch consumes a certain amount of cellulose each tick just to 
stay alive. If a branch gets less water/nitro than what it needs, it 
withers for X hours (how many?) and then dies. Don't eat too much!

Dead parts in the soil turn into some amount of nitro and water.

# Controls

The player can control:

## Resource mining. 
    * You can tune water demand from "Camelthorn" to "Water lily".
    * and nitro - from "bamboo" (super fast metabolism and growth) to "baobab".

Make sure the ecosystem can support you, and competitors don't overwhelm you!

## Resource spending.

### Tech tree - Mutations.

We have a good selection to choose from.

TODO: Gather the ideas here.

### Tactical, real-time decisions.

BIG HOLE IN DESIGN HERE!

Maybe have an army of symbiont ants/bugs/woodpeckers? Silly but funny.

- Controlling root growth by controlling the nitro consumption
    * Sounds like too little and too indirect.


# Game design.

TODO: What decisions does the player have, to get risk and reward?
- decisions in tech tree.
- what else?..

Resource spent on growing roots.
Risk: the invested resources.
Reward: potential resources?


# Limitations
* The thickness of a current branch must not be more than parent branch minus all the children.
    - Extension idea: replace it with conductivity limitation.

# Measurement units
milligrams, cm, mg/cm^3, hour, mg/hour.


# Future tech:

* Parasitic roots. At level 3 we can go fully parasitic, and develop a vine!
* Symbiotic insects. Feed them fruit, get them to remove parasites or soften the soil.
* Symbiotic plants: you can increase soil pH until it kills you; they decrease it. You must
  maintain them to survive.
* ...



# References
* http://algorithmicbotany.org/papers/enviro.sig96.pdf
* https://hal.science/hal-01113767/document

