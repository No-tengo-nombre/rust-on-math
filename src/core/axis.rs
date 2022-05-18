pub fn linspace_f32(start: f32, end: f32, num: i32) -> Vec<f32> {
    let delta = (end - start) / ((num as f32) - 1.0);
    let mut result = vec![];
    for i in 0..num {
        result.push(start + (i as f32) * delta);
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
    ($x:expr, $y:expr, $z:expr) => {{
        assert_eq!(true, rom_rs::utils::is_int($z as f32));
        rom_rs::linspace_f32($x as f32, $y as f32, $z as i32)
    }};
    ($x:expr, $y:expr) => {
        linspace_f32($x as f32, $y as f32, 50)
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
