# A self contained evolutionary ecosystem

![image](https://user-images.githubusercontent.com/1324601/214135720-d1641448-44b4-4993-baeb-b05888d5d603.png)

Based off of this amazing youtube video: https://youtu.be/N3tRFayqVtk?t=1433

## What is it?

I wanted to make some kind of evolving ecosystem with lifeforms with
neural nets. I didn't want it to use frame based evolution, where all the
lifeforms do their thing for a certain number of iterations, then the app
stops, measures each one's fitness, and reproduces the most successful ones.
Instead, I wanted it to be more like life we see here in our
universe, where time continues on without these breaks, and generations of
lifeforms can overlap.

This is the result -- a small world with lifeforms, food, danger. The
lifeforms reproduce when they eat enough. The danger is radioactive and
hunts down lifeforms (scary, right?). The Lifeforms can attack each other,
costing them both health.

The UI is terminal based. It uses [tui-rs](https://github.com/fdehau/tui-rs).

Here's a video of it in action, but note that this demonstrates mostly the UI. Because every time you run it and let it get to 500,000
or 1,000,000 iterations, different behaviors start to appear. One video cannot do it justice :)

https://user-images.githubusercontent.com/1324601/214379339-e6b794b2-08a9-4a16-815d-8086b882fcf5.mp4

## Properties of this app

* Cyclic neural nets
* Blazingly fast, written in Rust, with care taken to be efficient
* Parallelized using [rayon](https://docs.rs/rayon/latest/rayon/)

## Discoveries I made running it

* In one evolution I saw the lifeforms coordinate to keep the danger in the corner by every so often sacrificing one of themselves as bait.
  This is incredible evolutionary behavior and demonstrates the power of groupwise evolution even if it costs the individual.
* The neural calculation algorithm paired with the Rust programming language worked out splendidly.
  This program performs parallel recursive network calculations very efficiently, doing thousands of
  iterations, for dozens of lifeforms, each with their own neural net, per second. This is the case even
  on Dellbert, the mid grade, many years old laptop I built this on, and with lifeforms with 10 inner neurons
  and a genome of size 75.

## A little about the neural nets

There are three groups of neurons:
* **Input Neurons**, which get their values from the environment,
* **Output Neurons**, which, once their values are computed, determine what actions the lifeform is going to take, and
* **Inner Neurons**, the interesting ones. Running the app from the CLI, you can choose how many inner neurons there are going to be.
  The inner neurons take their inputs from either the input neurons _or other inner neurons_, and they output to either output
  neurons _or other inner neurons_. They are fully free to output to themselves as well. So the neuron graph becomes cyclic.
  There aren't layers like in a deep learning net, there's just one big blob of inner neurons that are free to interconnect how they want.
  I thought this was more representative of how it works in biology.
* Initially all of the lifeforms have the same set of neurons, which aren't connected to each other. They all have the same input neurons,
  output neurons and number of inner neurons. It's the **Genome** that represents the connections between neurons. The genome is comprised
  of an unordered list of **genes**, which each is `{ from: <neuron_id>, to: <neuron_id>, weight: f32 }`.
* As time goes on, it's the **Genome** that gets selected for under the evolutionary pressures.
* Lifeforms aren't chosen for their fitness after a certain period of time passes. Whether they reproduce and pass on their genome is determined
  _by whether they reproduce and pass on their genome_. There's no grade or score that determines whether the lifeforms reproduce - if they
  eat enough and naturally reproduce, then they pass on their genes.

## Interesting bits of code

* Recursion-less recursive neural net calculations. Recursion is elegant and beautiful, but in languages
  that can not guarantee tail optimization, like Rust, loops are faster. But the idea of recursion is great, especially
  because in this case it mimics the real world analog, a brain. So this app puts together an ordered list of genes to be
  followed one by one and have the neural net calculations done on them (which looks like `tanh(sum(inputs))`).
  Find that [here(ish)](https://github.com/Aaronik/evolution/blob/master/src/genome.rs#L98), with the code that walks that
  vector [around here](https://github.com/Aaronik/evolution/blob/a16f256aad4712f59ebc4f77d6e37b05c1a92bc5/src/lifeform.rs#L45).
* Within that neural net calculation function is this data structure I'm calling [NeuronGraph](https://github.com/Aaronik/evolution/blob/f8f31aceb834619b3f4342fd77fe4820088f9791/src/genome.rs#L31).
  I think this is cool because it's an infinitely recursive graph

## Things left undone

* Letting the lifeforms evolve the number of genes and inner neurons they have. Right now those values are fixed,
  but it'd be really cool to see if there were some ideal values, or at least local maxima/minima.
* Separating the main thread from the UI drawing thread. Instead of doing this, I'm leaving a less ideal
  solution of being able to pause the drawing from within the app itself.
* Ability to save the evolutions. They can evolve thousands of generations in only a few minutes, so it hasn't really
  been that important. But nonetheless, it'd be interesting to see how they'd be after a million generations!

## For Next Time

* The console UI is fun, but I'd definitely like some visual medium that is more expressive.
* I'd love to have a richer set of output actions. Maybe some that can facilitate more social
  behaviors.

