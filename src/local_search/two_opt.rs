use std::rc::Rc;

use crate::{
    domain::{route::Route, stop::Stop},
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

    fn should_swap_stops(swap_cost: f32) -> bool {
        swap_cost < 0.0
    }

    fn find_improvements(
        &self,
        stops: &Vec<Stop>,
        neighborhood: &Neighborhood,
    ) -> Option<(usize, f32)> {
        let (swap_candidate_index, swap_cost) =
            self.stop_swapper.get_minimum_swap_cost(neighborhood, stops);

        let swap_candidate = Neighborhood::from((
            stops.as_slice(),
            swap_candidate_index,
            self.distance_service.as_ref(),
        ));

        match Self::should_swap_stops(swap_cost) {
            true => Some((swap_candidate.current.index, swap_cost)),
            false => None,
        }
    }

    pub fn run(&self, route: &mut Route) {
        let mut found_improvement = false;

        while !found_improvement {
            found_improvement = false;

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
