pub trait LocalSearcher<T> {
    fn run(&self, target: &mut T);
}
