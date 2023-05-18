use vrp_solver::parsers::{
    cvrplib::cvrplib_parser::CvrpLibParser,
    vrp_parser::{VrpInputs, VrpParser},
};

pub(super) struct InstanceRepository;

impl InstanceRepository {
    pub(super) fn get_instance(name: &str, number_of_vehicles: u32) -> VrpInputs {
        CvrpLibParser::from_file(
            &format!("./benches/cvrplib/instances/{}.vrp", name),
            number_of_vehicles,
        )
        .parse()
    }
}
