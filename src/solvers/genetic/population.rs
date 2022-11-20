use super::individual::Individual;

#[derive(Default)]
pub(crate) struct Population {
    pub(crate) individuals: Vec<Individual>,
}

impl Population {
    pub(super) fn new(individuals: Vec<Individual>) -> Self {

        Self { individuals }
    }

    pub(super) fn sort_individuals(&mut self) {
        self.individuals.sort_by(|individual1, individual2| {
            individual1.fitness.partial_cmp(&individual2.fitness).unwrap()
        });
    }

    pub(super) fn get_k_bests(&self, k: usize) -> &[Individual] {
        &self.individuals[..k]
    }
}
