use train_station::TrainStation;
use trains::{freight_train::FreightTrain, passenger_train::PassengerTrain};

pub mod train_station;
pub mod trains;

fn main() {
    let train_1 = PassengerTrain::new("12043");
    let train_2 = FreightTrain::new("12604");

    let mut station = TrainStation::default();

    station.accept(train_1);
    station.accept(train_2);

    station.depart("12043");
    station.depart("12604");
    station.depart("12000");
}
