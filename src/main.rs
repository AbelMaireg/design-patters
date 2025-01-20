fn main() {
    let nav = Navigator::new(WalkStrategy {});
    println!("{}", nav.route("home", "school"));

    let nav = Navigator::new(PublicTransportStrategy {});
    println!("{}", nav.route("home", "school"));
}

#[allow(dead_code)]
trait RouteStrategy {
    fn build_route(&self, from: &str, to: &str) -> String;
}

#[allow(dead_code)]
struct WalkStrategy {}

#[allow(dead_code)]
struct PublicTransportStrategy {}

impl RouteStrategy for WalkStrategy {
    fn build_route(&self, from: &str, to: &str) -> String {
        format!("it takes 400 steps from {from} to {to}")
    }
}

impl RouteStrategy for PublicTransportStrategy {
    fn build_route(&self, from: &str, to: &str) -> String {
        format!("take the green bus to go from {from} to {to}")
    }
}

struct Navigator<T: RouteStrategy> {
    route_strategy: T,
}

impl<T: RouteStrategy> Navigator<T> {
    fn new(route_strategy: T) -> Self {
        Navigator { route_strategy }
    }

    fn route(&self, from: &str, to: &str) -> String {
        self.route_strategy.build_route(from, to)
    }
}
