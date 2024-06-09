use std::{cmp, collections::HashSet};

use rand::Rng;

use crate::{
    domain::vehicle::Vehicle,
    services::distance::distance_service::DistanceService,
    solvers::genetic::individual::{Chromosome, Gene, GeneAddress, Individual},
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

    pub(super) fn drop_gene_duplicates(
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

    pub(super) fn insert_parent_slice(
        self,
        individual: &mut Individual,
        insertion_point: GeneAddress,
        distance_service: &DistanceService,
    ) {
        const EMPTY_PARENT_SLICE_MESSAGE: &str = "the parent slice should not be empty";

        let genes = &individual.chromosomes[insertion_point.0].stops;
        let beginning_of_slice = insertion_point.1;
        let end_of_slice = insertion_point.1 + 1;

        let current_distance = match genes.len() == 1 {
            true => 0.0,
            false => {
                distance_service.get_distance(&genes[beginning_of_slice], &genes[end_of_slice])
            }
        };

        let new_distance_before = distance_service.get_distance(
            &genes[beginning_of_slice],
            self.slice.first().expect(EMPTY_PARENT_SLICE_MESSAGE),
        );

        let new_distance_after = match genes.len() == 1 {
            true => 0.0,
            false => distance_service.get_distance(
                self.slice.last().expect(EMPTY_PARENT_SLICE_MESSAGE),
                &genes[end_of_slice],
            ),
        };

        individual.chromosomes[insertion_point.0].add_multiple_stops_at(
            self.slice,
            end_of_slice,
            self.cost + new_distance_before + new_distance_after - current_distance,
        );

        individual.update_fitness();
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
        let offspring_vehicle = Vehicle::new(chromosome.vehicle.id, chromosome.vehicle.capacity);
        let mut offspring_chromosome = Chromosome::new(offspring_vehicle);

        offspring_chromosome.add_stop(chromosome.stops[0], 0.0).ok();

        let unrepeated_genes: Vec<Gene> = Self::drop_gene_duplicates(&chromosome, &self.gene_set);

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
