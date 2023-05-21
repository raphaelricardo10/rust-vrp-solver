use rstest::rstest;

use crate::domain::stop::Stop;

use crate::tests::fixtures::routes_fixture::{route_factory, RouteFactory};
use crate::tests::fixtures::stops_fixture::stops;

use crate::solvers::genetic::individual::Individual;

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
    assert_eq!(
        individual.chromosomes[address1.0].stops[address1.1].id,
        stops[2].id
    );
    assert_eq!(
        individual.chromosomes[address2.0].stops[address2.1].id,
        stops[1].id
    );
}
