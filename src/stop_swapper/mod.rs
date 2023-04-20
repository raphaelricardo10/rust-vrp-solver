pub(crate) mod neighbor;
pub(crate) mod neighborhood;

#[allow(clippy::module_inception)]
mod stop_swapper;
mod tests;
pub(crate) use stop_swapper::StopSwapper;
