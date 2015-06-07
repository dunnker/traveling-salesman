extern crate rand;
use rand::Rng;
use population::*;
use tour::*;
use city::*;

pub struct GA;

const MUTATION_RATE: f32 = 0.015;

impl GA {
    pub fn evolve_population(rng: &mut rand::ThreadRng, pop: Population) -> Population {
        let mut new_population = Population::new();
        let mut tours = Vec::new();

        // Crossover population
        // Loop over the new population's size and create individuals from
        // Current population
        for _ in 0..POP_COUNT {
            let random_value1: f32 = rng.gen::<f32>();
            let random_value2: f32 = rng.gen::<f32>();

            let parent1 = pop.get_tour(pop.get_random_tour(random_value1));
            let parent2 = pop.get_tour(pop.get_random_tour(random_value2));

            // Crossover parents
            let child: Tour = GA::crossover(rng, &parent1, &parent2);
            tours.push(child);
        }
        new_population.initialize(tours);

        // Mutate the new population a bit to add some new genetic material
        for i in 0..POP_COUNT {
            GA::mutate(rng, new_population.get_tour_mut(i));
        }

        new_population.initialize_fitness();

        new_population
    }

    fn crossover(rng: &mut rand::ThreadRng, parent1: &Tour, parent2: &Tour) -> Tour {
        let mut child: Tour = Tour::new();

        // Get start and end sub tour positions for parent1's tour
        let start_pos: usize = rng.gen_range(0, CITY_COUNT);
        let end_pos: usize = rng.gen_range(0, CITY_COUNT);

        // Loop and add the sub tour from parent1 to our child
        for i in 0..CITY_COUNT {
            // If our start position is less than the end position
            if start_pos < end_pos && i > start_pos && i < end_pos {
                child.set_city(i, parent1.get_city(i));

              // If our start position is larger
            } else if start_pos > end_pos { 
                if !(i < start_pos && i > end_pos) {
                    child.set_city(i, parent1.get_city(i));
                }
            }
        }

        // Loop through parent2's city tour
        for i in 0..CITY_COUNT {
            // If child doesn't have the city add it
            if !child.contains_city(parent2.get_city(i)) {
                // Loop to find a spare position in the child's tour
                for j in 0..CITY_COUNT {
                    // Spare position found, add city
                    if child.get_city(j).x == -1 {
                        child.set_city(j, parent2.get_city(i));
                        break;
                    }
                }
            }
        }

        child
    }

    fn mutate(rng: &mut rand::ThreadRng, tour: &mut Tour) {
        // Loop through tour cities
        for tour_pos1 in 0..CITY_COUNT {
            // Apply mutation rate
            if rng.gen::<f32>() < MUTATION_RATE {
                // Get a second random position in the tour
                let tour_pos2: usize = 0; //random.Next(CityList.CCityCount);

                if tour_pos2 != tour_pos1 {
                    // Get the cities at target position in tour
                    let city1 = tour.get_city(tour_pos1);
                    let city2 = tour.get_city(tour_pos2);

                    // Swap them around
                    tour.set_city(tour_pos2, city1);
                    tour.set_city(tour_pos1, city2);
                }
            }
        }
    }
}