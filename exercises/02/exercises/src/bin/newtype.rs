struct CarId(u32);
struct DriverId(u32);

fn get_car_id() -> CarId {
    CarId(0)
}
fn get_driver_id() -> DriverId {
    DriverId(0)
}

fn order_taxi(car_id: CarId, driver_id: DriverId) {}

fn main() {
    let car_id = get_car_id();
    let driver_id = get_driver_id();
    order_taxi(driver_id, car_id);
}
