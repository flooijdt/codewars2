use roots::find_roots_quadratic;
use roots::Roots;

fn max_ball(v0: i32) -> i32 {
    // your code
    let h: f32;
    let t: f32;
    let v: f32;
    let g: f32;

    let vms: f32 = v0 * 10 / 36;
    let h = vms;
    let t = 1.0;
    let g = 9.81;

    h = vms * t - 0.5 * g * t * t;

    println!("{:?}", t);
}

fn main() {
    max_ball(10);
}
