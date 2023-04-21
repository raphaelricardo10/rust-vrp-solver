use std::{cmp, collections::HashSet};

use rand::Rng;

use crate::{
    services::distance::distance_service::DistanceService,
    solvers::genetic::individual::{Chromosome, Gene, Individual},
};

pub(super) type GeneSet = HashSet<Gene>;

pub(crate) struct ParentSlice {
    pub(super) cost: f32,
    pub(super) slice: Vec<Gene>,
    pub(super) gene_set: HashSet<Gene>,
}

pub(crate) type RandomParentSliceGeneratorParams<'a, 'b, 'c, R> =
    (&'a Individual, &'b mut R, &'c DistanceService);

impl<'a, 'b, 'c, R: Rng + ?Sized> From<RandomParentSliceGeneratorParams<'a, 'b, 'c, R>>
    for ParentSlice
{
    fn from((parent, rng, distance_service): RandomParentSliceGeneratorParams<R>) -> Self {
        let (_, chromosome) = parent.choose_random_chromosome(rng, 4);

        let max_size = chromosome.stops.len() - 1;

        let (lower_bound, upper_bound) = Self::generate_range(1, max_size, rng);

        Self::new(
            chromosome.stops[lower_bound..upper_bound].to_vec(),
            distance_service,
        )
    }
}

impl ParentSlice {
    pub(super) fn new(slice: Vec<Gene>, distance_service: &DistanceService) -> Self {
        let cost = Self::calculate_slice_cost(&slice, distance_service);
        let gene_set: GeneSet = HashSet::from_iter(slice.iter().cloned());

        Self {
            cost,
            slice,
            gene_set,
        }
    }

    pub(super) fn calculate_slice_cost(slice: &[Gene], distance_service: &DistanceService) -> f32 {
        slice
            .windows(2)
            .map(|window| distance_service.get_distance(&window[0], &window[1]))
            .sum()
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

    pub(super) fn merge_into(
        &self,
        chromosome: Chromosome,
        distance_service: &DistanceService,
    ) -> Option<Chromosome> {
        let mut offspring_chromosome = Chromosome::new(chromosome.vehicle);

        offspring_chromosome.add_stop(chromosome.stops[0], 0.0).ok();

        let unrepeated_genes: Vec<Gene> =
            Individual::drop_gene_duplicates(&chromosome, &self.gene_set);

        if unrepeated_genes.len() == 2 {
            return Some(offspring_chromosome);
        }

        unrepeated_genes
            .windows(2)
            .map(|window| {
                (
                    window[1],
                    distance_service.get_distance(&window[0], &window[1]),
                )
            })
            .try_for_each(|(gene, distance)| offspring_chromosome.add_stop(gene, distance))
            .ok()?;

        Some(offspring_chromosome)
    }
}
