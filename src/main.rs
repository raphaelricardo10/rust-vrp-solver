use vrp_solver::domain::stop::Stop;
use vrp_solver::domain::vehicle::Vehicle;

fn main() {
    let mut a: Vehicle = Vehicle::new(10);
    
    if let Err(err) = a.load(9) {
        println!("{}", err)
    };

    if let Err(err) = a.unload(10) {
        println!("{}", err)
    }

    println!("Hello world!");
}
