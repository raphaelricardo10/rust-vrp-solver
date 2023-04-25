use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rstest::fixture;

use crate::solvers::genetic::individual::Individual;

use crate::tests::fixtures::services_fixture::{route_service_factory, RouteServiceFactory};

pub(crate) type IndividualFactory = Box<dyn FnMut(u32) -> Individual>;

#[fixture]
pub(crate) fn individual_factory(route_service_factory: RouteServiceFactory) -> IndividualFactory {
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    let wrapper = move |number_of_chromosomes| -> Individual {
        let mut route_service = route_service_factory(number_of_chromosomes);

        Individual::from((&mut rng, &mut route_service))
    };

    Box::new(wrapper)
}
