use std::{cmp, collections::HashSet};

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
    ) -> Option<()> {
        let distance_before = match self.chromosomes[insertion_point.0].stops.len() == 1 {
            true => 0.0,
            false => distance_service.get_distance(
                &self.chromosomes[insertion_point.0].stops[parent_slice.address.1 - 1],
                parent_slice.slice.first()?,
            )?,
        };

        let distance_after = distance_service.get_distance(
            parent_slice.slice.last()?,
            &self.chromosomes[insertion_point.0].stops[insertion_point.1],
        )?;

        self.chromosomes[insertion_point.0].add_multiple_stops_at(
            parent_slice.slice,
            parent_slice.address.1,
            parent_slice.cost + distance_before + distance_after,
        );

        self.update_fitness();

        Some(())
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
        let parent_slice = ParentSlice::from_random(self, rng, distance_service)?;

        let mut offspring_chromosomes: Vec<Chromosome> = Vec::new();

        for chromosome in parent.chromosomes {
            offspring_chromosomes.push(parent_slice.merge_into(chromosome, distance_service));
        }

        let mut offspring = Individual::new(offspring_chromosomes);
        let insertion_point: GeneAddress = offspring.choose_random_gene(rng).unwrap();

        offspring.insert_parent_slice(parent_slice, insertion_point, distance_service)?;

        Some(offspring)
    }
}
