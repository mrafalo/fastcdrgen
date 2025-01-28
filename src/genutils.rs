use chrono::{NaiveDateTime};
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, WeightedIndex};

use crate::{cdr::{self, ContactType, CDR}, config::Config, consts::{self, DistributionType}, customer::{Customer, CustomerType}, operator::Operator, relation::{Relation, RelationType}, utils::{self, draw_integer_from_uniform}};


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


pub fn get_customer_type(_priv_prc: f32) -> CustomerType{

    let rnd = utils::draw_integer_from_uniform(0, 100);

    let mut res: CustomerType = CustomerType::PRIVATE;

    if rnd < (_priv_prc * 100.0) as u32 {
        res = CustomerType::PRIVATE
    }

    if (rnd >= (_priv_prc * 100.0) as u32) & (rnd < 100) {
        res = CustomerType::BUSINESS
    }

    return res
} 

pub fn get_call_cnt(_mean: u32, _std_dev:u32, _dist: consts::DistributionType) ->u32{

    let mut res:u32 = 0;

    match _dist{
        DistributionType::CAUCHY => res = utils::draw_integer_from_cauchy(_mean as f32, _std_dev as f32),
        DistributionType::NORMAL => res = utils::draw_integer_from_normal(_mean as f32, _std_dev as f32),
        DistributionType::UNIFORM => res = utils::draw_integer_from_normal(_mean as f32, _std_dev as f32),
        DistributionType::POISON => res = utils::draw_integer_from_poisson(_mean as f32, _std_dev as f32),
        DistributionType::WEIBULL => res = utils::draw_integer_from_weibull(_mean as f32, _std_dev as f32),
    }

    let res = utils::draw_integer_from_normal(_mean as f32, 5.0);

    return res as u32;
} 

pub fn call_success(_customer_type: &CustomerType) -> bool{

    let mut res = true;
    let mut rng = rand::thread_rng();

    match _customer_type{
        CustomerType::BUSINESS => res = rng.gen_range(0..10) > 6,
        CustomerType::PRIVATE => res = rng.gen_range(0..10) > 4,
        CustomerType::SIMBOX => res = rng.gen_range(0..10) > 3,
        CustomerType::PROBE => res = rng.gen_range(0..10) > 2,
        CustomerType::MULTISIM => res = rng.gen_range(0..10) > 7,
    }

    
    return res;
} 

pub fn get_imei(_number_of_customers: u32) ->u32{

    let res = utils::draw_integer_from_uniform(0, _number_of_customers);

    return res as u32;
} 

pub fn get_sms_cnt(_mean: u32, _std_dev:u32) ->u32{

    let res = utils::draw_integer_from_normal(_mean as f32,  _std_dev as f32);
    return res as u32;
} 

pub fn get_mms_cnt(_mean: u32, _std_dev:u32) ->u32{

    let res = utils::draw_integer_from_normal(_mean as f32, _std_dev as f32);
    return res as u32;
} 

pub fn get_bts(_number_of_bts: u32) ->u32{
    let mut rng = rand::thread_rng();
    let res = rng.gen_range(0.._number_of_bts);
    return res as u32;
} 

pub fn get_start_time(_year:u32, _month:u32) -> NaiveDateTime{

    let res = utils::random_date_time(_year, _month);
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

