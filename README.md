# Rusty Boids

## A pure rust implementation of the boids flocking algorithm

This project was undertaken as a means to teach myself rust, a language I'm begining to be quite the fan of. 

This was initially built using the [Piston](https://github.com/PistonDevelopers/piston) graphics framework but due to some performance issues it was rewritten to use [ggez](https://github.com/ggez/ggez) to take advantage of draw call batching.

![boids](boids_function.gif)
