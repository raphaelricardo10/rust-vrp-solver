use std::fs;

use vrp_solver::parsers::{
    cvrplib::cvrplib_parser::CvrpLibParser,
    vrp_parser::{VrpInputs, VrpParser},
};

pub(super) struct InstanceRepository;

impl InstanceRepository {
    pub(super) fn get_instance(name: &str) -> VrpInputs {
        match fs::read_to_string(format!("./benches/cvrplib/instances/{}.vrp", name)) {
            Ok(content) => CvrpLibParser {
                content,
                number_of_vehicles: 5,
            }
            .parse(),

            Err(error) => panic!("Error reading file: {:?}", error),
        }
    }
}
