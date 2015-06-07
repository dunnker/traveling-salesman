extern crate rand;
use tour::*;
use city::*;

pub struct Population {
    tours: Vec<Tour>,
    total_fitness: f32,
}

pub const POP_COUNT: usize = 50;

impl Population {
    pub fn new() -> Population {
        Population {
            total_fitness: 0.0,
            tours: Vec::new(),
        }
    }

    pub fn initialize_from_cities(&mut self, rng: &mut rand::ThreadRng, city_list: &Vec<City>) {
        assert_eq!(self.tours.len(), 0);
        for _ in 0..POP_COUNT {
            let mut new_tour = Tour::new();
            new_tour.generate_individual(rng, &city_list);
            self.tours.push(new_tour);
        }
        self.initialize_fitness();
    }

    pub fn initialize(&mut self, tours: Vec<Tour>) {
        assert_eq!(tours.len(), POP_COUNT);
        assert_eq!(self.tours.len(), 0);
        for tour in tours {
            self.tours.push(tour);
        }
    }

    pub fn initialize_fitness(&mut self) {
        self.total_fitness = 0.0;
        for i in 0..POP_COUNT {
            self.tours[i].set_fitness();
            self.total_fitness += self.tours[i].fitness;
        }
        for i in 0..POP_COUNT {
            self.tours[i].set_relative_fitness(self.total_fitness, self.total_fitness / (POP_COUNT as f32));
        }
    }

    pub fn get_tour(&self, index: usize) -> &Tour {
        &self.tours[index]
    }

    pub fn get_tour_mut(&mut self, index: usize) -> &mut Tour {
        &mut self.tours[index]
    }

    pub fn get_fittest(&mut self) -> &Tour {
        let mut max_tour = &self.tours[0];
        for i in 0..POP_COUNT {
            if self.tours[i].fitness > max_tour.fitness {
                max_tour = &self.tours[i];
            }
        }
        max_tour
    }

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
}