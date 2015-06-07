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

##### Notes regarding the source code

I learned quite a bit about Rust as this was my first significant project in Rust. Besides the usual learning curve associated with borrowing and ownership, I had a couple of challenges to overcome that weren't immediately solved by reading through the Rust documentation. Many thanks to the people on the #rust IRC channel.

city.rs defines the basic city struct, which is just a x, y coordinate. I learned about the derive attribute, and how to give a struct Copy behavior on assignment.

```rust
#[derive(Copy, Clone)]
pub struct City {
    pub x: i32,
    pub y: i32,
}

impl Default for City {
    #[inline]
    fn default() -> City {
        City { x: -1, y: -1 }
    }
}
```
I could have added the Default attribute as well, but chose to implement the trait manually to assign each city a -1, -1 location by default. The really nice thing about implementing Default for a struct comes later when initializing an array. In tour.rs we need to initialize an array of cities, and we can do that in one line of code as follows:
```rust
impl Tour {
    pub fn new() -> Tour {
        Tour { 
            //...
            tour: [City::default(); CITY_COUNT],
        }
    }
```

So a Tour is simply a list of cities in which the order determines the route. The function generate_individual can setup an initial, random tour:
```rust
pub fn generate_individual(&mut self, rng: &mut rand::ThreadRng, city_list: &Vec<City>)
```
Note that the function takes a ThreadRng as a parameter so that it doesn't have to create its own ThreadRng. I've read that it helps with performance to cache the ThreadRng instance this way and not create many instances of ThreadRng in a loop.

The function set_relative_fitness is used to initialize the Tour's relative_fitness field. This is used later in our selection function for the genetic algorithm.
```rust
pub fn set_relative_fitness(&mut self, total_fitness: f32, average_fitness: f32)
```
Besides relative_fitness, there is amplified_fitness as well. This is similar to relative_fitness, however its value is either magnified or shrunk by a factor, AMPLIFY_FACTOR, relative to the average fitness of each tour in the  population. 
```rust
    self.amplified_fitness = (self.fitness + ((self.fitness - average_fitness) * AMPLIFY_FACTOR)) / total_fitness;
```
The selection function in population.rs uses amplified_fitness instead of relative_fitness because I found that each tour had essentially the same relative_fitness, and the roulette style selection function was not doing a good enough job of selecting out the highest individual tours for the next generation. The selection function is called get_random_tour:

```rust
pub fn get_random_tour(&self, random_value: f32) -> usize {
    let mut relative_total: f32 = 0.0;
    let mut result: usize = 0;
    if self.total_fitness > 0.0 {
        // randomValue is a number between 0 and 1.0
        // relativeTotal increments from 0 to 1.0
        for i in 0..POP_COUNT {
            if random_value < relative_total + self.tours[i].amplified_fitness {
                result = i;
                break;
            } else {
                relative_total = relative_total + self.tours[i].amplified_fitness;
            }
        }
    }
    result
}
```
In this function we loop through each tour in the population. For each tour we increment the variable, relative_total by the current tour's amplified_fitness. So the next time through the loop, we're able to know the range of fitness between the prior tour and the current tour -- this represents the "slot" in our roulette wheel. Each slot varies in size according to the amplified_fitness of each tour. And the supplied random_value will naturally fall on the bigger slots, and the more fit individuals.
I like this kind of selection because unfit individuals still have a chance of making it to the next generation. Different selection strategies have their own advantages, and they're interesting to read about on [wikipedia](http://en.wikipedia.org/wiki/Selection_%28genetic_algorithm%29).

If you replace amplified_fitness with relative_fitness in the selection function, you may find that the program does not do a good job of finding an optimal route. You may need to modify main.rs to iterate over many more generations to return a better route. Coming up with amplified_fitness was my solution, and I can see why Lee chose to use a tournament selection strategy.

In ga.rs the classic algorithm is implemented in a static struct called GA, since this struct does not keep any state on its own. The function evolve_population takes a population as a parameter and returns a new population by using population.rs's get_random_tour function to find individuals, then combine those individuals in GA's crossover function and finally mutations are applied. The crossover function is best explained on Lee's blog post.

With my roulette style fitness function, I found I had similar results to Lee's Java implementation using a tournament style selection. I found that the resulting executable was very fast, which is encouraging me to continue to use Rust. Next I want to explore Rust's foolproof concurrency features to improve performance further. Hope this was helpful to anyone getting started with Rust like me ;)

Discussion regarding this project can be found on reddit:
https://www.reddit.com/r/rust/comments/38vdg8/my_first_rust_program_traveling_salesman_solved/
