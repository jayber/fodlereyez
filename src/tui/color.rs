pub(crate) fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> (u8, u8, u8) {
    fn check_bounds(hue: f64, saturation: f64, value: f64) {
        fn panic_bad_params(name: &str, from_value: &str, to_value: &str, supplied: String) -> ! {
            panic!("{} must be between {} and {} inclusive, was {}", name, from_value, to_value, supplied)
        }

        if hue < 0.0 || hue > 360.0 {
            panic_bad_params("hue", "0.0", "360.0", hue.to_string())
        } else if saturation < 0.0 || saturation > 1.0 {
            panic_bad_params("saturation", "0.0", "1.0", saturation.to_string())
        } else if value < 0.0 || value > 1.0 {
            panic_bad_params("value", "0.0", "1.0", value.to_string())
        }
    }

    fn is_between(value: f64, min: f64, max: f64) -> bool {
        min <= value && value < max
    }

    check_bounds(hue, saturation, value);

    let c = value * saturation;
    let h = hue / 60.0;
    let x = c as f64 * (1.0 - ((h % 2.0) - 1.0).abs());
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

    (((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "hue must be between 0.0 and 360.0 inclusive, was -0.1")]
    fn test_hsv_bounds_fail1() {
        hsv_to_rgb(-0.1, 0.0, 0.0);
    }

    #[test]
    #[should_panic(expected = "hue must be between 0.0 and 360.0 inclusive, was 360.1")]
    fn test_hsv_bounds_fail2() {
        hsv_to_rgb(360.1, 0.0, 0.0);
    }

    #[test]
    #[should_panic(expected = "saturation must be between 0.0 and 1.0 inclusive, was -0.1")]
    fn test_hsv_bounds_fail3() {
        hsv_to_rgb(0.1, -0.1, 0.0);
    }

    #[test]
    #[should_panic(expected = "saturation must be between 0.0 and 1.0 inclusive, was 1.1")]
    fn test_hsv_bounds_fail4() {
        hsv_to_rgb(0.1, 1.1, 0.0);
    }

    #[test]
    #[should_panic(expected = "value must be between 0.0 and 1.0 inclusive, was -0.1")]
    fn test_hsv_bounds_fail5() {
        hsv_to_rgb(0.1, 0.1, -0.1);
    }

    #[test]
    #[should_panic(expected = "value must be between 0.0 and 1.0 inclusive, was 1.1")]
    fn test_hsv_bounds_fail6() {
        hsv_to_rgb(0.1, 0.1, 1.1);
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
    fn test_hsv_rust() {
        assert_eq!(hsv_to_rgb(28.0, 0.92, 0.71), (181, 92, 14));
    }

    #[test]
    fn test_hsv_purple() {
        assert_eq!(hsv_to_rgb(277.0, 0.87, 0.94), (159, 31, 239));
    }
}