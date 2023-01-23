# An self contained evolutionary ecosystem

![image](https://user-images.githubusercontent.com/1324601/214135720-d1641448-44b4-4993-baeb-b05888d5d603.png)

Based off of this amazing youtube video: https://youtu.be/N3tRFayqVtk?t=1433

I wanted to make something that didn't use frame based evolution, instead using evolution in real time.

This is the result -- a small world with life forms, food and danger. The lifeforms reproduce when
they're well enough fed. There's evolutionary pressure to avoid the danger, which is radioactive and hunts down
the faster lifeforms (scary, right?). There's evolutionary benefit to eating a lot of food. The Lifeforms can
also attack each other, but it costs them health.

The UI terminal based. It uses [tui-rs](https://github.com/fdehau/tui-rs).

## Properties of this app

* Neural Nets of arbitrary recursion
* Blazingly fast, written in Rust, with care taken to be efficient
* Makes efficient use of all available computer cores via concurrency using [rayon](https://docs.rs/rayon/latest/rayon/).

## Interesting bits of code

* Recursion-less recursive back propagating neural net calculations. Recursion is elegant and beautiful, but in languages
  that can not guarantee tail optimization, like Rust, loops are faster. But the idea of recursion is great, especially
  because in this case it mimics the real world analog, a brain. So this app puts together an ordered list of genes to be
  followed one by one and have the neural net calculations done on them (which looks like `tanh(sum(inputs))`).
  Find that [here(ish)](https://github.com/Aaronik/evolution/blob/master/src/genome.rs#L98), with the code that walks that
  vector [around here](https://github.com/Aaronik/evolution/blob/a16f256aad4712f59ebc4f77d6e37b05c1a92bc5/src/lifeform.rs#L45).

## Discoveries

* Already I've seen the lifeforms coordinate to keep the danger in the corner by every so often
  sacrificing one of themselves as bait. This is incredible evolutionary behavior and demonstrates the
  power of group wise evolution even if it costs the individual.
* The neural calculation algorithm paired with the Rust programming language worked out splendidly.
  This program performs parallel recursive network calculations very efficiently, doing thousands of
  iterations, for dozens of lifeforms, each with their own neural net, per second. This is the case even
  on Dellbert, the mid grade, many years old laptop I built this on, and with lifeforms with 10 inner neurons
  and a genome of size 75.

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

