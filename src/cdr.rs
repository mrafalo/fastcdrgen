use serde::Serialize;

use crate::customer::CustomerType;
#[derive(Serialize, Clone, Debug)]
pub enum ContactType {
    VOICE,
    SMS,
    MMS,
}

#[derive(Serialize,Clone, Debug)]
pub struct CDR{
    pub timestamp: String,
    pub duration_sec: u32,
    pub from_msisdn: u32,
    pub to_msisdn: u32,
    pub from_imei: u32,
    pub to_imei: u32,
    pub from_operator_id: u16,
    pub to_operator_id: u16,
    pub bts_id: u32,
    pub contact_type: ContactType,
    pub customer_type: CustomerType,
    pub roaming: u8,
    pub probe: u8,
    pub simbox: u8 

}


impl CDR{

    pub fn print_cdr(&self){
        println!("{:?}", &self);
    }

}

