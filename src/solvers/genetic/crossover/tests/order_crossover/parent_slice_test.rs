use crate::domain::stop::Stop;
use crate::solvers::genetic::crossover::parent_slice::ParentSlice;
use crate::solvers::genetic::crossover::tests::order_crossover::fixtures::parent_slice_fixture::{
    parent_slice_factory, ParentSliceFactory,
};
use crate::solvers::genetic::individual::Individual;
use crate::solvers::genetic::tests::fixtures::{individual_factory, IndividualFactory};
use crate::tests::fixtures::routes_fixture::{route_factory, RouteFactory};
use crate::tests::fixtures::services_fixture::distance_service;
use crate::tests::fixtures::stops_fixture::stops;
use rstest::rstest;

use crate::services::distance::distance_service::DistanceService;

#[rstest]
fn test_slice_cost_is_correct(mut parent_slice_factory: ParentSliceFactory) {
    let (_, slice) = parent_slice_factory(2);

    assert_eq!(slice.cost, 2.0);
}

#[rstest]
fn test_can_drop_gene_duplicates(mut parent_slice_factory: ParentSliceFactory) {
    let (parent, slice) = parent_slice_factory(2);

    let chromosome_without_duplicates =
        Individual::drop_gene_duplicates(&parent.chromosomes[0], &slice.gene_set);

    assert_eq!(chromosome_without_duplicates.len(), 3);

    for gene in chromosome_without_duplicates {
        assert!(!slice.gene_set.contains(&gene));
    }
}

#[rstest]
fn test_can_drop_all_genes_from_duplicates(mut parent_slice_factory: ParentSliceFactory) {
    let (parent, slice) = parent_slice_factory(3);

    let chromosome_without_duplicates =
        Individual::drop_gene_duplicates(&parent.chromosomes[0], &slice.gene_set);

    assert_eq!(chromosome_without_duplicates.len(), 2);
    assert_eq!(chromosome_without_duplicates[0].id, 0);
    assert_eq!(chromosome_without_duplicates[1].id, 0);
}

#[rstest]
fn test_can_generate_offspring_chromosome(
    distance_service: DistanceService,
    mut individual_factory: IndividualFactory,
    mut parent_slice_factory: ParentSliceFactory,
) {
    let (_, parent1_slice) = parent_slice_factory(2);
    let parent2 = individual_factory(1);

    let chromosome = parent1_slice.merge_into(parent2.chromosomes[0].clone(), &distance_service);

    assert_eq!(chromosome.stops.len(), 3);
}

#[rstest]
fn test_can_generate_offspring_chromosome_dropping_a_whole_chromosome(
    distance_service: DistanceService,
    mut individual_factory: IndividualFactory,
    mut parent_slice_factory: ParentSliceFactory,
) {
    let (_, parent1_slice) = parent_slice_factory(3);
    let parent2 = individual_factory(1);

    let chromosome = parent1_slice.merge_into(parent2.chromosomes[0].clone(), &distance_service);

    assert_eq!(chromosome.stops.len(), 1);
}

#[rstest]
fn test_can_insert_parent_slice_in_empty_offspring(
    stops: Vec<Stop>,
    route_factory: RouteFactory,
    distance_service: DistanceService,
) {
    let chromosome = route_factory(stops[0..=0].to_vec());
    let insertion_point = (0, 0);

    let mut offspring = Individual::new(vec![chromosome]);

    let slice = ParentSlice::new(stops[1..=3].to_vec(), (0, 0), &distance_service);

    offspring
        .insert_parent_slice(slice, insertion_point, &distance_service)
        .unwrap();

    assert_ne!(offspring.fitness, 0.0);
}
