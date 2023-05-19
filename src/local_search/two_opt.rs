use std::rc::Rc;

use crate::{
    domain::route::Route,
    services::distance::distance_service::DistanceService,
    solvers::vrp_solution::VrpSolution,
    stop_swapper::{neighborhood::Neighborhood, StopSwapper},
};

use super::local_searcher::LocalSearcher;

pub struct TwoOptSearcher {
    stop_swapper: StopSwapper,
    distance_service: Rc<DistanceService>,
}

impl TwoOptSearcher {
    pub fn new(distance_service: Rc<DistanceService>) -> Self {
        Self {
            distance_service: distance_service.clone(),
            stop_swapper: StopSwapper { distance_service },
        }
    }

    pub fn run(&self, route: &mut Route) {
        if route.stops.len() < 3 {
            return;
        }

        loop {
            let mut found_improvement = false;

            for stop_index_1 in 1..route.stops.len() - 2 {
                for stop_index_2 in (stop_index_1 + 1)..route.stops.len() - 1 {
                    let neighborhood_1 = Neighborhood::from((
                        route.stops.as_slice(),
                        stop_index_1,
                        self.distance_service.as_ref(),
                    ));

                    let neighborhood_2 = Neighborhood::from((
                        route.stops.as_slice(),
                        stop_index_2,
                        self.distance_service.as_ref(),
                    ));

                    let swap_cost = self
                        .stop_swapper
                        .calculate_swap_cost(&neighborhood_1, &neighborhood_2);

                    if swap_cost < 0.0 {
                        route.swap_stops(stop_index_1, stop_index_2, swap_cost);
                        found_improvement = true;
                    }
                }
            }

            if !found_improvement {
                break;
            }
        }
    }
}

impl LocalSearcher<VrpSolution> for TwoOptSearcher {
    fn run(&self, solution: &mut VrpSolution) {
        for (_, route) in solution.routes.iter_mut() {
            self.run(route);
        }
    }
}
