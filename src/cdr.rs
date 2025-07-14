use serde::Serialize;
use uuid::Uuid;

use crate::customer::{CustomerProfile, CustomerType};
#[derive(Serialize, Clone, Debug)]
pub enum ContactType {
    VOICE,
    SMS,
    MMS,
}

#[derive(Serialize, Clone, Debug)]
pub enum VoiceCallResult {
    IDLE,
    BUSY,
    OOS,
    CALL
}

#[derive(Serialize, Clone, Debug)]
pub enum CreationMethod {
    INIT, 
    ROUTE, 
    TRANSFER, 
    DIVERT,
    JOIN,
    PICKUP
}

#[derive(Serialize, Clone, Debug)]
pub enum ForwardReason {
    NOANSWER, 
    BUSY, 
    FORWARDCALL,
    CALLBACK,
    NONE
}


#[derive(Serialize,Clone, Debug)]
pub struct CDR{
    pub cdr_id: Uuid,
    pub originating_cdr_id: Uuid,
    pub continued_in_cdr_id: Uuid,
    pub creation_method: CreationMethod, //The action that caused the CDR to be created.
    pub forward_reason:ForwardReason, //The reason provided by a Transfer, Route To, or Divert API request.
    pub timestamp: String,
    pub duration_sec: u32,
    pub from_msisdn: u32,
    pub to_msisdn: u32,
    pub billing_msisdn: u32,
    pub from_imei: u32,
    pub to_imei: u32,
    pub from_operator_id: u16,
    pub to_operator_id: u16,
    pub bts_id: u32,
    pub contact_type: ContactType,
    pub customer_type: CustomerType,
    pub voice_call_result: VoiceCallResult,
    pub roaming: u8,
    pub customer_profile: CustomerProfile,
    pub scenario: String


}


impl CDR{

    pub fn print_cdr(&self){
        println!("{:?}", &self);
    }

}

