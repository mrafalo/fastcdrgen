use chrono::{NaiveDate, NaiveDateTime};
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, WeightedIndex};

use crate::{cdr::{ContactType, CreationMethod, ForwardReason, VoiceCallResult}, config::Config, consts::{DistributionType}, customer::{CustomerType}, utils::{self}};


pub fn draw_duration_from_distribution(_mean: u32, _std_dev:u32, _dist:DistributionType) -> u32{

    let mut res:u32 = 0;

    match _dist{
        DistributionType::CAUCHY => res = (
            utils::draw_integer_from_weibull(_mean as f32, _std_dev as f32) + 
            utils::draw_integer_from_cauchy(_mean as f32, _std_dev as f32) + 
            utils::draw_integer_from_normal(_mean as f32, _std_dev as f32)) / 3,
        DistributionType::NORMAL => res = utils::draw_integer_from_normal(_mean as f32, _std_dev as f32),
        DistributionType::UNIFORM => res = utils::draw_integer_from_normal(_mean as f32, _std_dev as f32),
        DistributionType::POISON => res = (utils::draw_integer_from_poisson(_mean as f32, _std_dev as f32) + utils::draw_integer_from_normal(_mean as f32, _std_dev as f32)) / 2,
        DistributionType::WEIBULL => res = (utils::draw_integer_from_weibull(_mean as f32, _std_dev as f32) + utils::draw_integer_from_normal(_mean as f32, _std_dev as f32)) / 2 ,

    }

    return res
}


pub  fn get_friend_cnt(_mean: u32, _std_dev:u32) -> u32{

    let res = utils::draw_integer_from_normal(_mean as f32, _std_dev as f32);
    return res as u32;
} 

pub  fn get_family_cnt(_mean: u32, _std_dev:u32) -> u32{

    let res = utils::draw_integer_from_normal(_mean as f32, _std_dev as f32);
    return res as u32;
} 

pub fn get_other_cnt(_mean: u32, _std_dev:u32) -> u32{

    let res = utils::draw_integer_from_poisson(_mean as f32, _std_dev as f32);
    return res as u32;
} 


pub fn get_operator_id(_weights: Vec<f32>) -> u32{

    let mut rng = thread_rng();
    let values: Vec<u32> = (0..(_weights.len() as u32)).collect();
    let dist = WeightedIndex::new(&_weights).unwrap();
    let res = values[dist.sample(&mut rng)];

    return res
} 


pub fn get_customer_type(_weights: Vec<f32>) -> CustomerType{

    let mut rng = thread_rng();
    let values: Vec<u32> = (0..(_weights.len() as u32)).collect();
    let dist = WeightedIndex::new(&_weights).unwrap();
    let res_id = values[dist.sample(&mut rng)];

    let mut res: CustomerType = CustomerType::PRIVATE;

    match res_id{
        0 => res = CustomerType::PRIVATE,
        1 => res = CustomerType::BUSINESS,
        2 => res = CustomerType::MIXED,
        3_u32..=u32::MAX => todo!()
    }

    return res
} 

pub fn get_creation_method() -> CreationMethod{

    let weights: Vec<f32> = vec![0.1, 0.7, 0.05, 0.05, 0.05, 0.05];
    let mut rng = thread_rng();
    let values: Vec<u32> = (0..(weights.len() as u32)).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let res_id = values[dist.sample(&mut rng)];

    let mut res: CreationMethod = CreationMethod::DIVERT;

    match res_id{
        0 => res = CreationMethod::DIVERT,
        1 => res = CreationMethod::INIT,
        2 => res = CreationMethod::JOIN,
        3 => res = CreationMethod::PICKUP,
        4 => res = CreationMethod::ROUTE,
        5 => res = CreationMethod::TRANSFER,
        5_u32..=u32::MAX => todo!()
    }

    return res
} 


pub fn get_forward_reason() -> ForwardReason{

    let weights: Vec<f32> = vec![0.1, 0.05, 0.05, 0.1, 0.7];
    let mut rng = thread_rng();
    let values: Vec<u32> = (0..(weights.len() as u32)).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let res_id = values[dist.sample(&mut rng)];

    let mut res: ForwardReason = ForwardReason::NONE;

    match res_id{
        0 => res = ForwardReason::BUSY,
        1 => res = ForwardReason::CALLBACK,
        2 => res = ForwardReason::FORWARDCALL,
        3 => res = ForwardReason::NOANSWER,
        4 => res = ForwardReason::NONE,
        5_u32..=u32::MAX => todo!()
    }

    return res
} 



pub fn get_voice_call_result() -> VoiceCallResult{

    let weights: Vec<f32> = vec![0.1, 0.8, 0.05, 0.05];
    let mut rng = thread_rng();
    let values: Vec<u32> = (0..(weights.len() as u32)).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let res_id = values[dist.sample(&mut rng)];

    let mut res: VoiceCallResult = VoiceCallResult::BUSY;

    match res_id{
        0 => res = VoiceCallResult::BUSY,
        1 => res = VoiceCallResult::CALL,
        2 => res = VoiceCallResult::IDLE,
        3 => res = VoiceCallResult::OOS,
        4_u32..=u32::MAX => todo!()
    }

    return res
} 


pub fn get_call_cnt(_scenario_name: &String, _cfg:Config) ->u32{

    let profile_cfg = _cfg.scenarios.get(_scenario_name).unwrap(); 
    let res = utils::draw_integer_from_normal(profile_cfg.avg_calls_cnt_per_relation as f32, profile_cfg.std_calls_cnt_per_relation as f32);

    return res as u32;
} 

pub fn call_success(_scenario_name: &String, _cfg:Config) -> bool{
    let mut rng = rand::thread_rng();
    let profile_cfg = _cfg.scenarios.get(_scenario_name).unwrap(); 
    let res = if rng.gen_bool(profile_cfg.call_success_prc as f64) { true } else { false };
    
    return res;
} 

pub fn get_imei(_number_of_customers: u32) ->u32{

    let res = utils::draw_integer_from_uniform(0, _number_of_customers);

    return res as u32;
} 


pub fn get_sms_cnt(_scenario_name: &String, _cfg:Config) ->u32{

    let profile_cfg = _cfg.scenarios.get(_scenario_name).unwrap(); 
    let res = utils::draw_integer_from_normal(profile_cfg.avg_sms_cnt_per_relation as f32, profile_cfg.std_sms_cnt_per_relation as f32);

    return res as u32;
} 


pub fn get_mms_cnt(_scenario_name: &String, _cfg:Config) ->u32{

    let profile_cfg = _cfg.scenarios.get(_scenario_name).unwrap(); 
    let res = utils::draw_integer_from_normal(profile_cfg.avg_mms_cnt_per_relation as f32, profile_cfg.std_mms_cnt_per_relation as f32);

    return res as u32;
} 


pub fn get_bts(_scenario_name: &String, _cfg:Config) ->u32{
    let profile_cfg = _cfg.scenarios.get(_scenario_name).unwrap(); 
    let mut rng = rand::thread_rng();

    let number_of_bts = profile_cfg.bts_used_prc * _cfg.market.number_of_own_bts as f32;


    let res = rng.gen_range(0..number_of_bts as u32);
    return res as u32;
} 

pub fn get_start_time(_year:u32, _month:u32) -> NaiveDateTime{
    let mut rng = thread_rng();
    let hour_weights: Vec<u32> = (0..24)
        .map(|hour| match hour {
            8..=20 => 10,          // peak
            6..=7 | 21..=23 => 4,  // medium
            _ => 1,                // low
        })
        .collect();
    let hours: Vec<u32> = (0..24).collect();
    let hour_dist = WeightedIndex::new(&hour_weights).unwrap();
    let hour = hours[hour_dist.sample(&mut rng)];

    let minute = rng.gen_range(0..60);
    let second = rng.gen_range(0..60);
    let day = rng.gen_range(1..=29);

    let res = NaiveDate::from_ymd_opt(_year as i32, _month, day)
        .unwrap()
        .and_hms_opt(hour, minute, second)
        .unwrap();

    return res;
}

pub fn get_contact_type() -> ContactType {
    let mut rng = rand::thread_rng();
    let choices = [ContactType::VOICE, ContactType::SMS, ContactType::MMS];
    let weights = [7, 2, 1];
    let dist = WeightedIndex::new(&weights).unwrap();

    let res = &choices[dist.sample(&mut rng)];

    return res.clone();
}

pub fn get_duration(_scenario_name: &String, _cfg:Config) ->u32{

    let profile_cfg = _cfg.scenarios.get(_scenario_name).unwrap(); 
    let res = draw_duration_from_distribution(profile_cfg.avg_call_duration, profile_cfg.std_call_duration, profile_cfg.call_duration_distribution);

    return res as u32;
} 



