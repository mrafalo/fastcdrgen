use crate::operator::Operator;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum CustomerType {
    PRIVATE,
    BUSINESS,
    PROBE,
    SIMBOX,
    MULTISIM
}


#[derive(Clone, Debug)]
pub struct Customer{
    pub msisdn: u32,
    pub imei: u32,
    pub operator: Operator,
    pub intl: u8,
    pub customer_type: CustomerType,
    pub avg_calls_cnt: u32,
    pub avg_sms_cnt: u32,
    pub avg_mms_cnt: u32

}
