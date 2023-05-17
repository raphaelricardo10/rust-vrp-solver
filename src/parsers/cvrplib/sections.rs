#[derive(PartialEq, Debug, Clone, Copy)]
pub(super) struct Node {
    pub(super) id: u32,
    pub(super) x_position: i16,
    pub(super) y_position: i16,
}

impl From<(&str, &str, &str)> for Node {
    fn from((id, x_position, y_position): (&str, &str, &str)) -> Self {
        Self {
            id: id.parse().unwrap(),
            x_position: x_position.parse().unwrap(),
            y_position: y_position.parse().unwrap(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub(super) struct Header<'a> {
    pub(super) name: &'a str,
    pub(super) comment: &'a str,
    pub(super) r#type: &'a str,
    pub(super) dimension: u32,
    pub(super) edge_weight_type: &'a str,
    pub(super) capacity: u32,
}

#[derive(PartialEq, Debug)]
pub(super) struct Demand {
    pub(super) node_id: u32,
    pub(super) demand: u32,
}

impl From<(&str, &str)> for Demand {
    fn from((node_id, demand): (&str, &str)) -> Self {
        Self {
            node_id: node_id.parse().unwrap(),
            demand: demand.parse().unwrap(),
        }
    }
}
