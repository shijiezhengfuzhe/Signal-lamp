fn main() {
    let light=TrafficLight::Red;
    println!("light is :{}",light.time());
}

enum  TrafficLight{
    Red ,
    Green ,
    Yellow,
}
impl TrafficLight {
    fn time(&self) -> u8{
        let x=60;
              x-15
    }
}