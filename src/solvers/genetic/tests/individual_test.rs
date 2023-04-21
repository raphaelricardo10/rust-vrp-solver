use rstest::rstest;

use crate::domain::stop::Stop;

use super::fixtures::{individual_factory, IndividualFactory};

use crate::tests::fixtures::routes_fixture::{route_factory, RouteFactory};
use crate::tests::fixtures::stops_fixture::stops;

use crate::solvers::genetic::individual::Individual;

#[rstest]
fn test_generate_random_individual(mut individual_factory: IndividualFactory) {
    let individual = individual_factory(2);

    for chromosome in individual.chromosomes.iter() {
        assert_eq!(chromosome.stops.first().unwrap().id, 0);
        assert_eq!(chromosome.stops.last().unwrap().id, 0);
    }

    assert_ne!(individual.chromosomes[0].stops.len(), 0);
    assert_ne!(individual.chromosomes[1].stops.len(), 0);
}

#[rstest]
fn test_fitness_is_correct(stops: Vec<Stop>, route_factory: RouteFactory) {
    let route1 = route_factory(stops[0..=1].to_vec());
    let route2 = route_factory(stops[2..=3].to_vec());

    let individual = Individual::new(vec![route1, route2]);

    assert_eq!(individual.fitness, 4.0);
}

#[rstest]
fn test_gene_swap(stops: Vec<Stop>, route_factory: RouteFactory) {
    let route = route_factory(stops[..4].to_vec());

    let mut individual = Individual::new(vec![route]);

    let address1 = (0, 1);
    let address2 = (0, 2);

    individual.swap_genes(address1, address2, -2.0);

    assert_eq!(individual.fitness, 7.0);
    assert_eq!(individual.get_gene(address1).id, stops[2].id);
    assert_eq!(individual.get_gene(address2).id, stops[1].id);
}
