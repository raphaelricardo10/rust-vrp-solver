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
        for stop_index in 1..route.stops.len() - 1 {
            let neighborhood = Neighborhood::from((
                route.stops.as_slice(),
                stop_index,
                self.distance_service.as_ref(),
            ));

            let (swap_candidate_index, distance_change) =
                match self.find_improvements(&route.stops, &neighborhood) {
                    Some(candidate_index) => candidate_index,
                    None => continue,
                };

            route.swap_stops(stop_index, swap_candidate_index, distance_change);
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
