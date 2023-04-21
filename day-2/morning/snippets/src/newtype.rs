struct Newton(f64);
struct Pascal(f64);
struct Area(f64);

fn main() {
    let force = Newton(10.0);
    let area = Area(23.0);

    println!("{}", calculate_pressure(force, area).0);
}

fn calculate_pressure(force: Newton, area: Area) -> Pascal {
    Pascal(force.0 / area.0)
}
