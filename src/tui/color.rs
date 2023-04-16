use hsv::hsv_to_rgb;

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
    const LOWEST_VALUE: f64 = 0.65;
    let size = size as f64;

    (
        calc_hue(size, HUE_MIN, HUE_MAX, BLUE_HUE),
        calc_saturation(size, SATURATION_MAX),
        calc_value(size, HUE_MAX, VALUE_MAX, LOWEST_VALUE),
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
    let hue_scale = if size <= hue_min {
        1.0
    } else if size >= hue_max {
        0.0
    } else {
        let range = (hue_max - hue_min).abs();
        let size_in_range = (size.max(hue_min).min(hue_max) - hue_min).max(0.0);
        let mid = (size_in_range / range) * FACTOR;
        (1.0 - (mid.log10() / FACTOR.log10())).max(0.0).min(1.0)
    };
    (hue_scale * base_hue * 100_f64).round() / 100_f64
}

#[cfg(test)]
mod test {
    use super::*;

    mod size_to_hsv {
        use crate::tui::color::{calc_hue, calc_saturation, calc_value};

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

            assert_eq!(
                calc_hue(1_000_000_000.0, 1_000_000.0, 1_000_000_000.0, hue_start),
                RED_HUE,
                "max size is {RED_HUE}"
            );
            assert_eq!(
                calc_hue(2_000_000_000.0, 1_000_000.0, 1_000_000_000.0, hue_start),
                RED_HUE,
                "more than max size is {RED_HUE}"
            );
            assert_eq!(
                calc_hue(1_000_000.0, 1_000_000.0, 1_000_000_000.0, hue_start),
                hue_start,
                "min size is {BLUE_HUE}"
            );
            assert_eq!(
                calc_hue(999_999.0, 1_000_000.0, 1_000_000_000.0, hue_start),
                hue_start,
                "less than min size is {BLUE_HUE}"
            );
            assert_eq!(calc_hue(1_500_000_000.0, 1_000_000_000.0, 2_000_000_000.0, hue_start), 24.08, "halfway");
            assert_eq!(
                calc_hue(1_750_000_000.0, 1_000_000_000.0, 2_000_000_000.0, hue_start),
                10.0,
                "nearly {RED_HUE}"
            );
            assert_eq!(
                calc_hue(1_250_000_000.0, 1_000_000_000.0, 2_000_000_000.0, hue_start),
                48.16,
                "nearly {BLUE_HUE}"
            )
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
