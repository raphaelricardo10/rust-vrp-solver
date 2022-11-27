use std::{cmp, collections::HashSet};

use rand::Rng;

use crate::{
    services::distance::distance_service::DistanceService,
    solvers::genetic::individual::{Chromosome, Gene, GeneAddress, Individual},
};

use super::{crossover_operator::CrossoverOperator, offspring::Offspring};

#[derive(Clone)]
pub(crate) struct OrderCrossover {
    max_of_tries: u8,
}

impl CrossoverOperator<OrderCrossover> for OrderCrossover {
    fn run<R>(
        &self,
        parent1: Individual,
        parent2: Individual,
        rng: &mut R,
        distance_service: &DistanceService,
    ) -> Option<Individual>
    where
        R: Rng + ?Sized,
    {
        Self::make_offspring(parent1, parent2, rng, distance_service)
    }

    fn max_of_tries(&self) -> u8 {
        self.max_of_tries
    }
}

impl OrderCrossover {
    pub(crate) fn new(max_of_tries: u8) -> Self {
        Self { max_of_tries }
    }

    fn generate_range<R>(min: usize, max: usize, rng: &mut R) -> (usize, usize)
    where
        R: Rng + ?Sized,
    {
        let a = rng.gen_range(min..=max);
        let mut b = rng.gen_range(min..=max);

        while a == b {
            b = rng.gen_range(min..=max);
        }

        (cmp::min(a, b), cmp::max(a, b))
    }

    fn choose_random_slice<'b, R>(
        parent: &'b Individual,
        rng: &'b mut R,
    ) -> Option<(GeneAddress, Vec<Gene>)>
    where
        R: Rng + ?Sized,
    {
        let (chromosome_index, chromosome) = parent.choose_random_chromosome(rng, 4)?;

        let max_size = chromosome.stops.len() - 1;

        let (lower_bound, upper_bound) = Self::generate_range(1, max_size, rng);

        let slice_address: GeneAddress = (chromosome_index, lower_bound);

        Some((
            slice_address,
            chromosome.stops[lower_bound..upper_bound].to_vec(),
        ))
    }

    pub(crate) fn drop_gene_duplicates(
        chromosome: &Chromosome,
        compare_set: &HashSet<Gene>,
    ) -> Vec<Gene> {
        chromosome
            .stops
            .iter()
            .filter(|gene| !compare_set.contains(gene))
            .cloned()
            .collect()
    }

    pub(crate) fn calculate_slice_cost(slice: &[Gene], distance_service: &DistanceService) -> f64 {
        slice
            .windows(2)
            .map(|window| {
                distance_service
                    .get_distance(&window[0], &window[1])
                    .unwrap()
            })
            .sum()
    }

    pub(crate) fn make_offspring_chromosome(
        parent1_slice: &HashSet<Gene>,
        parent2_chromosome: Chromosome,
        distance_service: &DistanceService,
    ) -> Chromosome {
        let mut offspring_chromosome = Chromosome::new(parent2_chromosome.vehicle);

        offspring_chromosome
            .add_stop(parent2_chromosome.stops[0], 0.0)
            .unwrap();

        let unrepeated_genes: Vec<Gene> =
            Self::drop_gene_duplicates(&parent2_chromosome, parent1_slice);

        if unrepeated_genes.len() == 2 {
            return offspring_chromosome;
        }

        unrepeated_genes
            .windows(2)
            .map(|window| {
                (
                    window[1],
                    distance_service
                        .get_distance(&window[0], &window[1])
                        .unwrap(),
                )
            })
            .for_each(|(gene, distance)| {
                offspring_chromosome.add_stop(gene, distance).unwrap();
            });

        offspring_chromosome
    }

    pub(crate) fn insert_parent_slice_in_offspring(
        offspring: &mut Individual,
        insertion_point: GeneAddress,
        parent_slice: Vec<Gene>,
        parent_slice_cost: f64,
        distance_service: &DistanceService,
    ) -> Option<()> {
        let distance_before = match offspring.chromosomes[insertion_point.0].stops.len() == 1 {
            true => 0.0,
            false => distance_service.get_distance(
                &offspring.chromosomes[insertion_point.0].stops[insertion_point.1 - 1],
                parent_slice.first()?,
            )?,
        };

        let distance_after = distance_service.get_distance(
            parent_slice.last()?,
            &offspring.chromosomes[insertion_point.0].stops[insertion_point.1],
        )?;

        offspring.chromosomes[insertion_point.0].add_multiple_stops_at(
            parent_slice,
            insertion_point.1,
            parent_slice_cost + distance_before + distance_after,
        );

        offspring.update_fitness();

        Some(())
    }

    pub(crate) fn make_offspring<R>(
        parent1: Individual,
        parent2: Individual,
        rng: &mut R,
        distance_service: &DistanceService,
    ) -> Option<Individual>
    where
        R: Rng + ?Sized,
    {
        let (_, parent_slice): (GeneAddress, Vec<Gene>) = Self::choose_random_slice(&parent1, rng)?;

        let parent_slice_cost = Self::calculate_slice_cost(&parent_slice, distance_service);

        let genes_set: HashSet<Gene> = HashSet::from_iter(parent_slice.iter().cloned());

        let mut offspring_chromosomes: Vec<Chromosome> = Vec::new();

        for chromosome in parent2.chromosomes {
            offspring_chromosomes.push(Self::make_offspring_chromosome(
                &genes_set,
                chromosome,
                distance_service,
            ));
        }

        let mut offspring = Individual::new(offspring_chromosomes);
        let insertion_point: GeneAddress = offspring.choose_random_gene(rng).unwrap();

        Self::insert_parent_slice_in_offspring(
            &mut offspring,
            insertion_point,
            parent_slice,
            parent_slice_cost,
            distance_service,
        )?;

        Some(offspring)
    }
}
