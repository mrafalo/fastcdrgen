use std::collections::HashMap;

use serde::Deserialize;

use crate::{consts, customer::CustomerType, customer::CustomerProfile};


#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub market: Market,
    
    #[serde(flatten)]
    pub scenarios: HashMap<String, CallProfile>,
    pub technical: Technical
}


#[derive(Deserialize, Debug, Clone)]
pub struct CallProfile {
    pub profile_occurence_prc: f32,
    pub customer_type: CustomerType,
    pub customer_profile: CustomerProfile,
    pub call_duration_distribution:consts::DistributionType, 
    pub avg_call_duration:u32,
    pub std_call_duration:u32,
    pub call_success_prc: f32,

    pub msisdn_per_imei: u32,
    pub bts_used_prc: f32,

    pub avg_relation_friends_cnt:u32,
    pub std_relation_friends_cnt:u32,

    pub avg_relation_family_cnt:u32,
    pub std_relation_family_cnt:u32,

    pub avg_relation_business_cnt:u32,
    pub std_relation_business_cnt:u32,

    pub avg_relation_other_cnt:u32,
    pub std_relation_other_cnt:u32,

    pub avg_calls_cnt_per_relation:u32,
    pub std_calls_cnt_per_relation:u32,

    pub avg_sms_cnt_per_relation:u32,
    pub std_sms_cnt_per_relation:u32,

    pub avg_mms_cnt_per_relation:u32,
    pub std_mms_cnt_per_relation:u32,

    pub relation_prob:f32,
    pub inverse_relation_prob:f32
}

#[derive(Deserialize, Debug, Clone)]
pub struct Market {

    pub number_of_local_customers:u32,
    pub number_of_intl_customers:u32,
    pub local_operators: Vec<f32>,
    pub intl_operators: Vec<f32>,
    pub number_of_own_bts:u16,
    pub number_of_external_bts:u16,
    pub random_noise_prc: f32
}    

#[derive(Deserialize, Debug, Clone)]
pub struct Technical {
    pub start_date:String,
    pub batch_size:u32,
    pub detailed_resut_filename:String,
    pub agg_resut_filename:String,
}
