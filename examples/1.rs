fn main() {
    let r = RedLight;
    let g = GreenLight;
    let y = YellowLight;

    println!("{}", r.lightingtime());
    println!("{}", g.lightingtime());
    println!("{}", y.lightingtime());
}

trait LightingTime {
    fn lightingtime(&self) -> u32;
}

struct RedLight;
struct GreenLight;
struct YellowLight;

impl LightingTime for RedLight {
    fn lightingtime(&self) -> u32 {
        60
    }
}

impl LightingTime for GreenLight {
    fn lightingtime(&self) -> u32 {
        20
    }
}

impl LightingTime for YellowLight {
    fn lightingtime(&self) -> u32 {
        5
    }
}
