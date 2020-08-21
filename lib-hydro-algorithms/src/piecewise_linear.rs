use crate::IntoAndFromF64Array3;




//============================================================================
fn sgn(a: f64) -> f64 {
    1.0f64.copysign(a)
}

fn minabs(a: f64, b: f64, c: f64) -> f64 {
    a.abs().min(b.abs()).min(c.abs())
}

fn plm_gradient_f64(theta: f64, yl: f64, y0: f64, yr: f64) -> f64 {
    let a = (y0 - yl) * theta;
    let b = (yr - yl) * 0.5;
    let c = (yr - y0) * theta;
    0.25 * (sgn(a) + sgn(b)).abs() * (sgn(a) + sgn(c)) * minabs(a, b, c)
}




//============================================================================
pub fn plm_gradient3<T: Copy + IntoAndFromF64Array3>(theta: f64, xl: &T, x0: &T, xr: &T) -> T {
    let yl = xl.into_f64_array3();
    let y0 = x0.into_f64_array3();
    let yr = xr.into_f64_array3();

    let p0 = plm_gradient_f64(theta, yl[0], y0[0], yr[0]);
    let p1 = plm_gradient_f64(theta, yl[1], y0[1], yr[1]);
    let p2 = plm_gradient_f64(theta, yl[2], y0[2], yr[2]);

    T::from_f64_array3([p0, p1, p2])
}
