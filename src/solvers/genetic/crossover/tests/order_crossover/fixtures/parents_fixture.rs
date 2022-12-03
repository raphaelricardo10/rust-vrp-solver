use rstest::fixture;

use crate::{
    domain::stop::Stop,
    solvers::genetic::individual::Individual,
    tests::fixtures::{
        routes_fixture::{route_factory, RouteFactory},
        stops_fixture::stops,
    },
};

pub(crate) type Parents = (Individual, Individual);

#[fixture]
pub(crate) fn parents(stops: Vec<Stop>, route_factory: RouteFactory) -> Parents {
    let route1 = route_factory(vec![stops[0], stops[1], stops[0]].to_vec());
    let route2 = route_factory(vec![stops[0], stops[2], stops[3], stops[0]].to_vec());

    let parent1 = Individual::new(vec![route1, route2]);
    let mut parent2 = parent1.clone();

    parent2.swap_genes((0, 1), (1, 1), 3.0);

    (parent1, parent2)
}
