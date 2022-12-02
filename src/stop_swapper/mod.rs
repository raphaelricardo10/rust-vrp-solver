pub(crate) mod path;
pub(crate) mod path_node;

#[allow(clippy::module_inception)]
mod stop_swapper;
mod tests;
pub(crate) use stop_swapper::StopSwapper;
