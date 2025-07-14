use crate::operator::Operator;
use serde::Serialize;
use serde::Deserialize;


#[derive(Serialize, Clone, Debug, PartialEq, Deserialize, Copy)]
pub enum CustomerType {
    PRIVATE,
    BUSINESS,
    MIXED,
    PROBE,
    SIMBOX,
    MULTISIM
}


#[derive(Serialize, Clone, Debug, PartialEq, Deserialize, Copy)]
pub enum CustomerProfile {
    NORMAL,
    ANOMALY,
    FRAUD,
    CHURN,
}


#[derive(Clone, Debug)]
pub struct Customer{
    pub scenario: String,
    pub msisdn: u32,
    pub imei: u32,
    pub operator: Operator,
    pub intl: u8,
    pub customer_type: CustomerType,
    pub customer_profile: CustomerProfile,
    pub avg_calls_cnt: u32,
    pub avg_sms_cnt: u32,
    pub avg_mms_cnt: u32

}
