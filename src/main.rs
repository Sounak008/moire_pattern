use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let thickness: f32 = rng.gen_range(15.0..40.0);
    let mut distance:f32;
    let mut modulo:f32;

    for y in -17..=17 {
        for x in -34..=34 {
            distance = (x * x + y * y) as f32;
            modulo = distance%thickness;

            if modulo < 0.5*thickness { print!("@#")}
            else { print!("..")};
        }
        println!();
    }    
}
