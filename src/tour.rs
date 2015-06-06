extern crate rand;
extern crate num;
use num::*;
use rand::Rng;
use city::*;

pub struct Tour {
	tour: [City; CITY_COUNT],
	pub fitness: f32,
	pub relative_fitness: f32,
	pub amplified_fitness: f32,
}

const AMPLIFY_FACTOR: f32 = 2f32;

impl Tour {
	pub fn new() -> Tour {
		Tour { 
            fitness: 0.0,
            relative_fitness: 0.0,
            amplified_fitness: 0.0,
            tour: [City::default(); CITY_COUNT],
		}
	}

	pub fn generate_individual(&mut self, rng: &mut rand::ThreadRng, city_list: &Vec<City>) {
		assert_eq!(city_list.len(), CITY_COUNT);

		// copy cities in original sequence
		for i in 0..CITY_COUNT {
			self.tour[i] = city_list[i];
		}
		// shuffle to create new sequence
        for _ in 0..100 {
            for j in 0..CITY_COUNT {
	            let random_index: i32 = rng.gen_range(0, CITY_COUNT as i32);
	            if random_index != j as i32
	            {
	            	// swap the city at j with the city at random_index:
	                let save_city = self.get_city(j);
	                let random_city = self.get_city(random_index as usize);
	                self.tour[j] = random_city;
	                self.tour[random_index as usize] = save_city;
	            }
	        }
	    }
	}

	pub fn get_city(&self, index: usize) -> City {
		self.tour[index]
	}

	pub fn set_city(&mut self, index: usize, city: City) {
		self.fitness = 0.0;
		self.relative_fitness = 0.0;
		self.amplified_fitness = 0.0;
		self.tour[index] = city;
	}

    pub fn contains_city(&self, find_city: City) -> bool {
		for tour in &self.tour {
            if tour.x == find_city.x && tour.y == find_city.y {
                return true;
            }
        }
        false
    }

    pub fn set_fitness(&mut self) {
    	let distance: f32 = self.get_distance() as f32;
    	assert!(distance > 0.0001);
        self.fitness = (1f32 / distance) * 100f32;
    }

    pub fn set_relative_fitness(&mut self, total_fitness: f32, average_fitness: f32) {
    	assert!(total_fitness > 0.0001);
	    self.relative_fitness = self.fitness / total_fitness;
	    self.amplified_fitness = (self.fitness + ((self.fitness - average_fitness) * AMPLIFY_FACTOR)) / total_fitness;
    }

    fn distance_to(&self, from_city: City, to_city: City) -> f32 {
        let x_distance: i32 = num::abs(from_city.x - to_city.x);
        let y_distance: i32 = num::abs(from_city.y - to_city.y);
    	// use Pythagorean theorem to calculate distance:
        let sums_squared: i32 = (x_distance * x_distance) + (y_distance * y_distance);
        let distance: f32 = num::Float::sqrt(sums_squared as f32);
        distance
    }

    pub fn get_distance(&self) -> i32 {
        let mut tour_distance: i32 = 0;
		for i in 0..CITY_COUNT {
            let from_city = self.tour[i];
            let destination_city =
	            // check we're not on our tour's last city, if we are set our
	            // tour's final destination city to our starting city
	            if i + 1 < CITY_COUNT {
	                self.tour[i + 1]
	            } else {
	                self.tour[0]
	            };

            tour_distance += self.distance_to(from_city, destination_city) as i32;
        }
        tour_distance
    }
}