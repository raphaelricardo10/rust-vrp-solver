use rand::{seq::IteratorRandom, thread_rng, Rng};

use crate::{
    domain::{route::Route, stop::Stop},
    services::route::route_service::RouteService,
    stop_swapper::{neighborhood::Neighborhood, StopSwapper},
};

pub(super) type Gene = Stop;
pub(super) type Chromosome = Route;
pub(super) type GeneAddress = (usize, usize);

impl Default for Individual {
    fn default() -> Self {
        Self {
            fitness: f32::MAX,
            chromosomes: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Individual {
    pub(super) fitness: f32,
    pub(super) chromosomes: Vec<Chromosome>,
}

pub(crate) type RandomIndividualGeneratorParams<'a, 'b, R> = (&'a mut R, &'b mut RouteService);

impl<'a, 'b, R: Rng + ?Sized> From<RandomIndividualGeneratorParams<'a, 'b, R>> for Individual {
    fn from((rng, route_service): RandomIndividualGeneratorParams<R>) -> Self {
        let vehicle_ids: Vec<u32> = route_service
            .get_vehicles()
            .iter()
            .map(|vehicle| vehicle.id)
            .collect();

        route_service.assign_starting_points();

        while route_service.has_available_stop() {
            for vehicle_id in vehicle_ids.iter() {
                let stop = match route_service.get_random_stop(*vehicle_id, rng) {
                    Some(stop) => stop,
                    None => continue,
                };

                route_service
                    .assign_stop_to_route(*vehicle_id, stop.id)
                    .unwrap_or_else(|_| panic!("the vehicle {vehicle_id} should support the load"));
            }
        }

        route_service.assign_stop_points();

        let routes: Vec<Route> = route_service.get_all_routes().values().cloned().collect();

        Individual::new(routes)
    }
}

impl Individual {
    pub fn new(chromosomes: Vec<Chromosome>) -> Self {
        let fitness = Self::calculate_fitness(&chromosomes);

        Self {
            fitness,
            chromosomes,
        }
    }

    fn calculate_fitness(chromosomes: &[Chromosome]) -> f32 {
        chromosomes
            .iter()
            .map(|chromosome| chromosome.total_distance())
            .sum()
    }

    pub(crate) fn update_fitness(&mut self) {
        self.fitness = Self::calculate_fitness(&self.chromosomes);
    }

    pub(crate) fn swap_genes(
        &mut self,
        address1: GeneAddress,
        address2: GeneAddress,
        fitness_change: f32,
    ) {
        self.chromosomes[address1.0].swap_stops(address1.1, address2.1, fitness_change);
        self.fitness += fitness_change;
    }

    pub(crate) fn choose_random_chromosome<R>(
        &self,
        rng: &mut R,
        min_genes: usize,
    ) -> (usize, &Chromosome)
    where
        R: Rng + ?Sized,
    {
        self.chromosomes
            .iter()
            .enumerate()
            .filter(|(_, chromosome)| chromosome.stops.len() >= min_genes)
            .choose(rng)
            .expect("the chromosome should not be empty")
    }

    pub(crate) fn choose_random_gene_pair<R>(&self, rng: &mut R) -> (GeneAddress, GeneAddress)
    where
        R: Rng + ?Sized,
    {
        let (chromosome_index, chromosome): (usize, &Chromosome) = self
            .chromosomes
            .iter()
            .enumerate()
            .filter(|(_, chromosome)| chromosome.stops.len() > 3)
            .choose(&mut thread_rng())
            .expect("the chromosome should not be empty");

        let addresses: Vec<GeneAddress> = chromosome
            .stops
            .iter()
            .enumerate()
            .skip(1)
            .take(chromosome.stops.len() - 2)
            .choose_multiple(rng, 2)
            .iter()
            .map(|(gene_index, _)| (chromosome_index, *gene_index))
            .collect();

        (addresses[0], addresses[1])
    }

    pub(crate) fn choose_random_gene<R>(&self, rng: &mut R) -> GeneAddress
    where
        R: Rng + ?Sized,
    {
        let (chromosome_index, chromosome) = self
            .chromosomes
            .iter()
            .enumerate()
            .choose(rng)
            .expect("the chromosome should not be empty");

        if chromosome.stops.len() == 1 {
            return (chromosome_index, 0);
        }

        let (gene_index, _) = chromosome
            .stops
            .iter()
            .enumerate()
            .skip(1)
            .take(chromosome.stops.len() - 2)
            .choose(rng)
            .expect("the chromosome should not be empty");

        (chromosome_index, gene_index)
    }

    pub(crate) fn swap_random_genes<R>(&mut self, stop_swapper: &StopSwapper, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        let (address1, address2): (GeneAddress, GeneAddress) = self.choose_random_gene_pair(rng);

        let neighborhood1 = Neighborhood::from((
            self.chromosomes
                .as_slice()
                .get(address1.0)
                .expect("the chromosome index must be inside of the bounds vector")
                .stops
                .as_slice(),
            address1.1,
            stop_swapper.distance_service.as_ref(),
        ));

        let neighborhood2 = Neighborhood::from((
            self.chromosomes
                .get(address2.0)
                .expect("the chromosome index must be inside of the bounds vector")
                .stops
                .as_slice(),
            address2.1,
            stop_swapper.distance_service.as_ref(),
        ));

        let swap_cost = stop_swapper.calculate_swap_cost(&neighborhood1, &neighborhood2);

        self.swap_genes(address1, address2, swap_cost);
    }
}
