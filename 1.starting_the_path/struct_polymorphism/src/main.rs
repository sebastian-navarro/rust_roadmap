#[derive(Debug)]
struct Sedan;

impl LandCapable for Sedan {
    
}

struct SUV;

impl LandCapable for SUV {
    fn drive(&self){
        println!("SUV is driving");
    }
}

trait LandCapable {
    fn drive(&self){
        println!("Default land drive")
    }
}

trait WaterCapable {
    fn float(&self) {
        println!("Floating drive")
    }
}

trait Amphibious: WaterCapable + LandCapable {}

struct Hovercraft;

impl Amphibious for Hovercraft{}
impl LandCapable for Hovercraft{}
impl WaterCapable for Hovercraft{}

fn road_trip(vehicle: & dyn LandCapable) {
    vehicle.drive();
}

fn traverse_float_lake(vehicle: &dyn Amphibious){
    vehicle.drive();
    vehicle.float();
}



fn main() {
    let car = Sedan;
    road_trip(&car);
    
    let suv = SUV;
    road_trip(&suv);

    let amp = Hovercraft;
    traverse_float_lake(&amp);

}
