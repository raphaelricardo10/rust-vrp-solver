mod cvrplib;

use criterion::criterion_main;

criterion_main! {
    cvrplib::genetic_grasp_benchmarks::genetic_grasp,
}
