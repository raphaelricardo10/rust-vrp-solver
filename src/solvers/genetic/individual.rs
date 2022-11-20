use crate::domain::{route::Route, stop::Stop};

type Gene = Stop;
type Chromosome = Route;
type GeneAddress = (usize, usize);

#[derive(Clone)]
pub(crate) struct Individual {
    pub(crate) fitness: f64,
    pub(crate) chromosomes: Vec<Chromosome>,
}

impl Individual {
    pub fn new(chromosomes: Vec<Chromosome>) -> Self {
        let fitness = Self::calculate_fitness(&chromosomes);

        Self {
            fitness,
            chromosomes,
        }
    }

    fn calculate_fitness(chromosomes: &[Chromosome]) -> f64 {
        chromosomes
            .iter()
            .map(|chromosome| chromosome.total_distance())
            .sum()
    }

    pub(crate) fn get_gene(&self, address: GeneAddress) -> Option<Gene> {
        self.chromosomes
            .get(address.0)?
            .stops
            .get(address.1)
            .copied()
    }

    pub(crate) fn insert_gene(&mut self, address: GeneAddress, gene: Gene) -> Option<()> {
        self.chromosomes.get_mut(address.0)?.stops[address.1] = gene;
        Some(())
    }

    pub(crate) fn swap_genes(
        &mut self,
        address1: GeneAddress,
        address2: GeneAddress,
        fitness_change: f64,
    ) -> Option<()> {
        let gene1 = self.get_gene(address1)?;
        let gene2 = self.get_gene(address2)?;

        let aux = gene1;
        self.insert_gene(address1, gene2);
        self.insert_gene(address2, aux);

        self.fitness += fitness_change;

        Some(())
    }
}
