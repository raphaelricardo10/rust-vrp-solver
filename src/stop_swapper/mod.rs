pub(crate) mod neighborhood;
pub(crate) mod neighbor;

#[allow(clippy::module_inception)]
mod stop_swapper;
mod tests;
pub(crate) use stop_swapper::StopSwapper;
