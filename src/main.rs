use roots::find_roots_quadratic;

fn max_ball(v0: i32) -> i32 {
    // your code
    let h: f32;
    let t: f32;
    let v: f32;
    let g: f32;

    let vms: f32 = (v0 as f32) * 10.0 / 36.0;
    let mut h = vms;
    let t: f32;
    let g = 9.81;

    h = vms * t * 0.1 - 0.5 * g * t * t * 0.01;

    println!("{:?}", h);
    let roots = find_roots_quadratic(-0.005 * 9.81, 0.1 * vms, -h);
    println!("{:?}", roots);
    h as i32
}

fn main() {
    max_ball(37);
    max_ball(45);
}
