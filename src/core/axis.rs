pub const DEFAULT_SPACE_LEN: i32 = 50;
pub const DEFAULT_LOGSPACE_BASE: f32 = 10.0;

pub fn linspace_f32(start: f32, end: f32, num: i32) -> Vec<f32> {
    let delta = (end - start) / ((num as f32) - 1.0);
    let mut result = vec![];
    for i in 0..num {
        result.push(start + (i as f32) * delta);
    }
    return result;
}

pub fn logspace_f32(start: f32, end: f32, num: i32, base: f32) -> Vec<f32> {
    let exponents = linspace_f32(start, end, num);
    let mut result = vec![];
    for i in exponents {
        result.push(base.powf(i));
    }
    return result;
}

pub fn arange_f32(start: f32, end: f32, step: f32) -> Vec<f32> {
    let mut result = vec![];
    let mut i = 0;
    loop {
        let new_val = start + (i as f32) * step;
        if new_val.abs() < end.abs() {
            result.push(new_val);
            i += 1;
        } else {
            break;
        }
    }
    return result;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//======================================|| Macros ||=============================================//
///////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! linspace {
    ($start:expr, $end:expr, $num:expr) => {{
        assert_eq!(true, rom_rs::utils::is_int($num as f32));
        rom_rs::linspace_f32($start as f32, $end as f32, $num as i32)
    }};
    ($start:expr, $end:expr) => {
        linspace_f32($start as f32, $end as f32, DEFAULT_SPACE_LEN)
    };
}

#[macro_export]
macro_rules! logspace {
    ($start:expr, $end:expr, $num:expr, $base:expr) => {{
        assert_eq!(true, rom_rs::utils::is_int($num as f32));
        rom_rs::logspace_f32($start as f32, $end as f32, $num as i32, $base as f32)
    }};
    ($start:expr, $end:expr, $num:expr) => {{
        assert_eq!(true, rom_rs::utils::is_int($num as f32));
        rom_rs::logspace_f32(
            $start as f32,
            $end as f32,
            $num as i32,
            rom_rs::axis::DEFAULT_LOGSPACE_BASE,
        )
    }};
    ($start:expr, $end:expr) => {
        logspace_f32(
            $start as f32,
            $end as f32,
            rom_rs::axis::DEFAULT_SPACE_LEN,
            rom_rs::axis::DEFAULT_LOGSPACE_BASE,
        )
    };
}

#[macro_export]
macro_rules! arange {
    ($start:expr, $end:expr, $step:expr) => {
        rom_rs::arange_f32($start as f32, $end as f32, $step as f32)
    };
    ($start:expr, $end:expr) => {
        rom_rs::arange_f32($start as f32, $end as f32, 1.0)
    };
    ($end:expr) => {
        rom_rs::arange_f32(0.0, $end as f32, 1.0)
    };
}
