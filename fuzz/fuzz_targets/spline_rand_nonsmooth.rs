#![no_main]
use libfuzzer_sys::fuzz_target;
use spline;
use kurbo;

fuzz_target!(|value: Vec<(f64, f64)>| {
    if value.len() > 0 {
        let points: Vec<kurbo::Point> = value.iter().map(|x| kurbo::Point::new(x.0, x.1)).collect();
        let mut spec = spline::SplineSpec::new();
        spec.move_to(points[0]);

        for pt in &points[1..] {
            spec.spline_to(None, None, *pt, false);
        }

        spec.solve();
    }
});