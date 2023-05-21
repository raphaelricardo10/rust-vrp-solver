use std::collections::HashSet;

use rand::Rng;

use crate::{
    services::distance::distance_service::DistanceService,
    solvers::genetic::individual::{Chromosome, Gene, GeneAddress, Individual},
};

use super::parent_slice::ParentSlice;

impl Individual {
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
        &mut self,
        parent_slice: ParentSlice,
        insertion_point: GeneAddress,
        distance_service: &DistanceService,
    ) {
        let genes = &self.chromosomes[insertion_point.0].stops;
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
            parent_slice
                .slice
                .first()
                .expect("the parent slice should not be empty"),
        );

        let new_distance_after = match genes.len() == 1 {
            true => 0.0,
            false => distance_service.get_distance(
                parent_slice
                    .slice
                    .last()
                    .expect("the parent slice should not be empty"),
                &genes[end_of_slice],
            ),
        };

        self.chromosomes[insertion_point.0].add_multiple_stops_at(
            parent_slice.slice,
            end_of_slice,
            parent_slice.cost + new_distance_before + new_distance_after - current_distance,
        );

        self.update_fitness();
    }

    pub(crate) fn crossover_with<R>(
        &self,
        parent: Individual,
        rng: &mut R,
        distance_service: &DistanceService,
    ) -> Option<Individual>
    where
        R: Rng + ?Sized,
    {
        let parent_slice = ParentSlice::from((self, &mut *rng, distance_service));

        let mut offspring_chromosomes: Vec<Chromosome> = Vec::new();

        for chromosome in parent.chromosomes {
            let merged_chromosome = parent_slice.merge_into(chromosome, distance_service)?;

            offspring_chromosomes.push(merged_chromosome);
        }

        let mut offspring = Individual::new(offspring_chromosomes);
        let insertion_point: GeneAddress = offspring.choose_random_gene(rng);

        offspring.insert_parent_slice(parent_slice, insertion_point, distance_service);

        Some(offspring)
    }
}
