
use rand::prelude::*;
use rand_distr::{Cauchy, Distribution, Normal, Poisson, Weibull};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rand::Rng;
use special::Gamma;


pub fn draw_integer_from_uniform(_min:u32, _max:u32) -> u32 {
    let mut rng = rand::thread_rng();
    let res =  rng.gen_range(_min.._max);

    return res;
}

pub fn draw_integer_from_weibull(_mean: f32, _std_dev: f32) -> u32 {

    let shape = 1.0;

    let gamma_term = Gamma::gamma(1.0 + 1.0 / shape);
    let scale = _mean / gamma_term;
    let weibull = Weibull::new(scale, shape).unwrap();
    let mut rng = rand::thread_rng();
    let mut value = weibull.sample(&mut rng) as u32 ;


    if value < 0 {
        value = 0;
    }

    if value > (20.0 * _mean) as u32 {
        value = (20.0 * _mean) as u32;
    }

    return value;

}

pub fn draw_integer_from_cauchy(_mean: f32, _std_dev: f32) -> u32 {

    let scale = 1.0;
    let cauchy = Cauchy::new(_mean, scale).unwrap();
    let mut rng = rand::thread_rng();
    //let sample = cauchy.sample(&mut rng);

    let mut value = (cauchy.sample(&mut rng)) as u32;
    
    if value < 0 {
        value = 0;
    }

    if value > (20.0 * _mean) as u32 {
        value = (20.0 * _mean) as u32;
    }

    return value
}

pub fn draw_integer_from_poisson(_mean: f32, _std_dev: f32) -> u32 {
    let mut rng = thread_rng();
    let poisson = Poisson::new(_mean).unwrap();
    let mut value = (poisson.sample(&mut rng)) as u32;
    
    if value < 0 {
        value = 0;
    }

    return value
}

pub fn draw_hour_from_normal(_mean: f32, _std_dev: f32) -> i32 {
    let mut rng = thread_rng();
    let normal = Normal::new(_mean, _std_dev).unwrap();
    let mut value = normal.sample(&mut rng).round() as i32;
    
    if value < 0 {
        value = 0;
    }

    if value > 23 {
        value = 23
    }

    return value
}

pub fn draw_integer_from_normal(_mean: f32, _std_dev: f32) -> u32 {
    let mut rng = thread_rng();
    let normal = Normal::new(_mean, _std_dev).unwrap();
    let mut value = normal.sample(&mut rng).round() as i32;
    
    if value < 0 {
        value = 0;
    }

    return value as u32
}

fn is_leap_year(_year: u32) -> bool {
    (_year % 4 == 0 && _year % 100 != 0) || (_year % 400 == 0)
}


pub fn random_date_time(_year: u32, _month: u32) -> NaiveDateTime {
    let mut rng = rand::thread_rng();

    let days_in_month = match _month {
        1 => 31,
        2 => if is_leap_year(_year) { 29 } else { 28 },
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => panic!("Invalid month"),
    };

    let day = rng.gen_range(1..=days_in_month);

    let hour = draw_hour_from_normal(13.0, 10.0) as u32;
    let minute = rng.gen_range(0..60);
    let second = rng.gen_range(0..60);

    let date = NaiveDate::from_ymd_opt(_year as i32, _month, day).unwrap();
    let time = NaiveTime::from_hms_opt(hour, minute, second).unwrap();
    let res = NaiveDateTime::new(date, time);

    return res;
}



