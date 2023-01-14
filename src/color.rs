pub(crate) fn convert_file_size_to_color(size: u64) -> (u8, u8, u8) {
    let (hue, saturation, value) = size_to_hsv(size);
    hsv_to_rgb(hue, saturation, value)
}

const BLUE_HUE: f64 = 210.0;

fn size_to_hsv(size: u64) -> (f64, f64, f64) {
    const HUE_MIN: f64 = 1024_f64 * 1024_f64;
    const HUE_MAX: f64 = 1024_f64 * 1024_f64 * 1024_f64 * 100_f64;
    const SATURATION_MAX: f64 = 1024_f64 * 1024_f64 * 500_f64;
    const VALUE_MAX: f64 = 1024_f64 * 1024_f64 * 1024_f64 * 200_f64;
    const LOWEST_VALUE: f64 = 0.55;
    let size = size as f64;
    (
        calc_hue(size, HUE_MIN, HUE_MAX, BLUE_HUE),
        calc_saturation(size, SATURATION_MAX),
        calc_value(size, HUE_MAX, VALUE_MAX, LOWEST_VALUE)
    )
}

fn calc_value(size: f64, value_min: f64, value_max: f64, lowest_value: f64) -> f64 {
    (1.0 - ((size - value_min).max(0.0).min(value_max) / value_max)).max(lowest_value)
}

fn calc_saturation(size: f64, saturation_max: f64) -> f64 {
    const FACTOR: f64 = 1000_f64;
    let sat = (size / saturation_max) * FACTOR;
    let sat = sat.log10() / FACTOR.log10();
    ((sat * 100_f64).round() / 100_f64).min(1.0).max(0.0)
}

fn calc_hue(size: f64, hue_min: f64, hue_max: f64, base_hue: f64) -> f64 {
    const FACTOR: f64 = 1_000_f64;
    let hue_scale = if size <= hue_min { 1.0 } else if size >= hue_max { 0.0 } else {
        let range = (hue_max - hue_min).abs();
        let size_in_range = (size.max(hue_min).min(hue_max) - hue_min).max(0.0);
        let mid = (size_in_range / range) * FACTOR;
        (1.0 - (mid.log10() / FACTOR.log10())).max(0.0).min(1.0)
    };
    (hue_scale * base_hue * 100_f64).round() / 100_f64
}

fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> (u8, u8, u8) {
    fn is_between(value: f64, min: f64, max: f64) -> bool {
        min <= value && value < max
    }

    check_bounds(hue, saturation, value);

    let c = value * saturation;
    let h = hue / 60.0;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = value - c;

    let (r, g, b): (f64, f64, f64) = if is_between(h, 0.0, 1.0) {
        (c, x, 0.0)
    } else if is_between(h, 1.0, 2.0) {
        (x, c, 0.0)
    } else if is_between(h, 2.0, 3.0) {
        (0.0, c, x)
    } else if is_between(h, 3.0, 4.0) {
        (0.0, x, c)
    } else if is_between(h, 4.0, 5.0) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn check_bounds(hue: f64, saturation: f64, value: f64) {
    fn panic_bad_params(name: &str, from_value: &str, to_value: &str, supplied: f64) -> ! {
        panic!(
            "param {} must be between {} and {} inclusive; was: {}",
            name, from_value, to_value, supplied
        )
    }

    if hue < 0.0 || hue > 360.0 {
        panic_bad_params("hue", "0.0", "360.0", hue)
    } else if saturation < 0.0 || saturation > 1.0 {
        panic_bad_params("saturation", "0.0", "1.0", saturation)
    } else if value < 0.0 || value > 1.0 {
        panic_bad_params("value", "0.0", "1.0", value)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    mod check_bounds {
        use super::check_bounds;

        #[test]
        #[should_panic(expected = "param hue must be between 0.0 and 360.0 inclusive; was: -0.1")]
        fn test_check_bounds_fail1() { check_bounds(-0.1, 0.0, 0.0); }

        #[test]
        #[should_panic(expected = "param hue must be between 0.0 and 360.0 inclusive; was: 360.1")]
        fn test_check_bounds_fail2() {
            check_bounds(360.1, 0.0, 0.0);
        }

        #[test]
        #[should_panic(expected = "param saturation must be between 0.0 and 1.0 inclusive; was: -0.1")]
        fn test_check_bounds_fail3() {
            check_bounds(0.1, -0.1, 0.0);
        }

        #[test]
        #[should_panic(expected = "param saturation must be between 0.0 and 1.0 inclusive; was: 1.1")]
        fn test_check_bounds_fail4() {
            check_bounds(0.1, 1.1, 0.0);
        }

        #[test]
        #[should_panic(expected = "param value must be between 0.0 and 1.0 inclusive; was: -0.1")]
        fn test_check_bounds_fail5() {
            check_bounds(0.1, 0.1, -0.1);
        }

        #[test]
        #[should_panic(expected = "param value must be between 0.0 and 1.0 inclusive; was: 1.1")]
        fn test_check_bounds_fail6() {
            check_bounds(0.1, 0.1, 1.1);
        }

        #[test]
        fn test_check_bounds_red() {
            check_bounds(0.0, 1.0, 1.0);
        }

        #[test]
        fn test_check_bounds_green() {
            check_bounds(120.0, 1.0, 1.0);
        }

        #[test]
        fn test_check_bounds_blue() {
            check_bounds(240.0, 1.0, 1.0);
        }

        #[test]
        fn test_check_bounds_yellow() {
            check_bounds(60.0, 1.0, 1.0);
        }

        #[test]
        fn test_check_bounds_rust() {
            check_bounds(28.0, 0.92, 0.71);
        }

        #[test]
        fn test_check_bounds_purple() {
            check_bounds(277.0, 0.87, 0.94);
        }
    }

    mod hsv_to_rgb {
        use super::hsv_to_rgb;

        #[test]
        fn test_hsv_black() {
            assert_eq!(hsv_to_rgb(0.0, 0.0, 0.0), (0, 0, 0));
        }

        #[test]
        fn test_hsv_white() {
            assert_eq!(hsv_to_rgb(0.0, 0.0, 1.0), (255, 255, 255));
        }

        #[test]
        fn test_hsv_red() {
            assert_eq!(hsv_to_rgb(0.0, 1.0, 1.0), (255, 0, 0));
        }

        #[test]
        fn test_hsv_green() {
            assert_eq!(hsv_to_rgb(120.0, 1.0, 1.0), (0, 255, 0));
        }

        #[test]
        fn test_hsv_blue() {
            assert_eq!(hsv_to_rgb(240.0, 1.0, 1.0), (0, 0, 255));
        }

        #[test]
        fn test_hsv_yellow() {
            assert_eq!(hsv_to_rgb(60.0, 1.0, 1.0), (255, 255, 0));
        }

        #[test]
        fn test_hsv_cyan() {
            assert_eq!(hsv_to_rgb(180.0, 1.0, 1.0), (0, 255, 255));
        }

        #[test]
        fn test_hsv_magenta() {
            assert_eq!(hsv_to_rgb(300.0, 1.0, 1.0), (255, 0, 255));
        }

        #[test]
        fn test_hsv_rust() {
            assert_eq!(hsv_to_rgb(28.0, 0.92, 0.71), (181, 92, 14));
        }

        #[test]
        fn test_hsv_purple() {
            assert_eq!(hsv_to_rgb(277.0, 0.87, 0.94), (159, 31, 239));
        }
    }

    mod size_to_hsv {
        use super::*;

        const RED_HUE: f64 = 0.0;

        #[test]
        fn test_calc_hue() {
            let hue_start = 240.0;
            assert_eq!(calc_hue(1.0, 0.0, 1.0, hue_start), RED_HUE, "max size is {RED_HUE}");
            assert_eq!(calc_hue(2.0, 0.0, 1.0, hue_start), RED_HUE, "more than max size is {RED_HUE}");
            assert_eq!(calc_hue(0.0, 0.0, 1.0, hue_start), hue_start, "min size is {BLUE_HUE}");
            assert_eq!(calc_hue(0.0, 1.0, 2.0, hue_start), hue_start, "less than min size is {BLUE_HUE}");
            assert_eq!(calc_hue(0.5, 0.0, 1.0, hue_start), 24.08, "halfway");
            assert_eq!(calc_hue(0.75, 0.0, 1.0, hue_start), 10.0, "nearly {RED_HUE}");
            assert_eq!(calc_hue(0.25, 0.0, 1.0, hue_start), 48.16, "nearly {BLUE_HUE}");

            assert_eq!(calc_hue(1_000_000_000.0, 1_000_000.0, 1_000_000_000.0, hue_start), RED_HUE, "max size is {RED_HUE}");
            assert_eq!(calc_hue(2_000_000_000.0, 1_000_000.0, 1_000_000_000.0, hue_start), RED_HUE, "more than max size is {RED_HUE}");
            assert_eq!(calc_hue(1_000_000.0, 1_000_000.0, 1_000_000_000.0, hue_start), hue_start, "min size is {BLUE_HUE}");
            assert_eq!(calc_hue(999_999.0, 1_000_000.0, 1_000_000_000.0, hue_start), hue_start, "less than min size is {BLUE_HUE}");
            assert_eq!(calc_hue(1_500_000_000.0, 1_000_000_000.0, 2_000_000_000.0, hue_start), 24.08, "halfway");
            assert_eq!(calc_hue(1_750_000_000.0, 1_000_000_000.0, 2_000_000_000.0, hue_start), 10.0, "nearly {RED_HUE}");
            assert_eq!(calc_hue(1_250_000_000.0, 1_000_000_000.0, 2_000_000_000.0, hue_start), 48.16, "nearly {BLUE_HUE}")
        }

        #[test]
        fn test_calc_saturation() {
            assert_eq!(calc_saturation(1.0, 1.0), 1.0, "max size is max saturation");
            assert_eq!(calc_saturation(0.0, 1.0), 0.0, "min size is min saturation");
            assert_eq!(calc_saturation(0.5, 1.0), 0.9, "halfway saturation");
            assert_eq!(calc_saturation(0.25, 1.0), 0.8, "quarter saturation");
            assert_eq!(calc_saturation(0.75, 1.0), 0.96, "three-quarter saturation");

            assert_eq!(calc_saturation(10_000_000_000_000.0, 1_000_000_000_000.0), 1.0, "max size is max saturation");
            assert_eq!(calc_saturation(500_000_000_000.0, 1_000_000_000_000.0), 0.9, "halfway saturation");
            assert_eq!(calc_saturation(250_000_000_000.0, 1_000_000_000_000.0), 0.8, "quarter saturation");
            assert_eq!(calc_saturation(750_000_000_000.0, 1_000_000_000_000.0), 0.96, "three-quarter saturation");
        }


        #[test]
        fn test_calc_value() {
            assert_eq!(calc_value(1.0, 0.0, 1.0, 0.0), 0.0, "max size is min value");
            assert_eq!(calc_value(0.0, 0.0, 1.0, 0.0), 1.0, "min size is max value");
            assert_eq!(calc_value(0.5, 0.0, 1.0, 0.0), 0.5, "halfway is halfway value");
        }
    }
}
