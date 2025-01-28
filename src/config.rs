use serde::Deserialize;

use crate::consts;


#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub number_of_local_customers:u32,
    pub number_of_intl_customers:u32,
    pub number_of_local_operators:u16,
    pub number_of_intl_operators:u16,
    pub number_of_bts:u16,
    pub simbox_prc: f32,
    pub probe_prc: f32,
    pub multisim_prc: f32,

    pub avg_relation_friends_cnt_priv:u32,
    pub std_relation_friends_cnt_priv:u32,
    pub avg_relation_family_cnt_priv:u32,
    pub std_relation_family_cnt_priv:u32,
    pub avg_relation_other_cnt_priv:u32,
    pub std_relation_other_cnt_priv:u32,
    
    pub avg_relation_friends_cnt_business:u32,
    pub std_relation_friends_cnt_business:u32,
    pub avg_relation_family_cnt_business:u32,
    pub std_relation_family_cnt_business:u32,
    pub avg_relation_other_cnt_business:u32,
    pub std_relation_other_cnt_business:u32,

    pub avg_relation_cnt_probe:u32,
    pub std_relation_cnt_probe:u32,
    
    pub avg_relation_cnt_simbox:u32,
    pub std_relation_cnt_simbox:u32,
    
    pub avg_relation_cnt_multi:u32,
    pub std_relation_cnt_multi:u32,

    pub avg_call_duration_priv:u32,
    pub std_call_duration_priv:u32,
    pub avg_call_duration_business:u32,
    pub std_call_duration_business:u32,
    pub avg_call_duration_probe:u32,
    pub std_call_duration_probe:u32,
    pub avg_call_duration_simbox:u32,
    pub std_call_duration_simbox:u32,
    pub avg_call_duration_multi:u32,
    pub std_call_duration_multi:u32,

    pub avg_calls_cnt_per_relation_probe:u32,
    pub stdev_calls_cnt_per_relation_probe:u32,

    pub avg_calls_cnt_per_relation_simbox:u32,
    pub stdev_calls_cnt_per_relation_simbox:u32,

    pub avg_calls_cnt_per_relation_multi:u32,
    pub stdev_calls_cnt_per_relation_multi:u32,

    pub avg_calls_cnt_per_relation:u32,
    pub stdev_calls_cnt_per_relation:u32,
    
    pub avg_sms_cnt_per_relation:u32,
    pub stdev_sms_cnt_per_relation:u32,

    pub avg_mms_cnt_per_relation:u32,
    pub stdev_mms_cnt_per_relation:u32,

    pub start_date:String,
    pub batch_size:u32,
    pub resut_filename:String,
    pub priv_prc:f32,
    
    pub simbox_duration_distribution:consts::DistributionType, 
    pub probe_duration_distribution:consts::DistributionType,
    pub priv_duration_distribution:consts::DistributionType, 
    pub business_duration_distribution:consts::DistributionType, 
    pub multi_duration_distribution:consts::DistributionType, 
}
