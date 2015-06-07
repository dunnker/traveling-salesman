traveling-salesman

This is a Rust program to determine an optimal solution to the traveling
salesman problem, using a genetic algorithm.

The code is based off a Java implementation by Lee Jacobson. His excellent
blog post is found here:
http://www.theprojectspot.com/tutorial-post/applying-a-genetic-algorithm-to-the-travelling-salesman-problem/5

However, this implementation does not use a tournament selection for each
generation. Rather a [roulette style selection](http://en.wikipedia.org/wiki/Fitness_proportionate_selection) algorithm is used.

To build the project enter the following command:

cargo build

To run the project enter:

cargo run

