use rand::Rng;

use crate::{
    services::distance::distance_service::DistanceService,
    solvers::genetic::individual::{Chromosome, GeneAddress, Individual},
};

use super::{crossover_operator::CrossoverOperator, parent_slice::ParentSlice};

#[derive(Clone)]
pub struct OrderCrossover {
    max_of_tries: u8,
}

impl<R: Rng + ?Sized> CrossoverOperator<R> for OrderCrossover {
    fn run(
        &self,
        parent1: Individual,
        parent2: Individual,
        rng: &mut R,
        distance_service: &DistanceService,
    ) -> Option<Individual> {
        let parent_slice = ParentSlice::from((&parent1, &mut *rng, distance_service));

        let mut offspring_chromosomes: Vec<Chromosome> = Vec::new();

        for chromosome in parent2.chromosomes {
            let merged_chromosome = parent_slice.merge_into(chromosome, distance_service)?;
            offspring_chromosomes.push(merged_chromosome);
        }

        let mut offspring = Individual::new(offspring_chromosomes);
        let insertion_point: GeneAddress = offspring.choose_random_gene(rng);

        parent_slice.insert_at_individual(&mut offspring, insertion_point, distance_service);

        Some(offspring)
    }

    fn max_of_tries(&self) -> u8 {
        self.max_of_tries
    }
}

impl OrderCrossover {
    pub fn new(max_of_tries: u8) -> Self {
        Self { max_of_tries }
    }
}
