use std::{fs::{OpenOptions}, path::Path};
use csv::{WriterBuilder};
use rand::{Rng};
use uuid::Uuid;

use crate::{cdr::{ContactType, CDR}, config::Config, customer::{Customer, CustomerType}, operator::Operator, relation::{Relation, RelationType}, utils::{draw_integer_from_uniform}};
use crate::genutils::*;

pub struct Generator{
    operators: Vec<Operator>,
    customers: Vec<Customer>,
    relations: Vec<Relation>,
    cfg: Config

}

impl Generator{

    pub fn print_summary(&self){

        println!("Number of customers: {:?}", self.customers.len());

        let cnt = self.customers
            .iter()
            .filter(|customer| customer.customer_type == CustomerType::PRIVATE) 
            .count();
        println!("Number of private customers: {:?}", cnt);

        let cnt = self.customers
            .iter()
            .filter(|customer| customer.customer_type == CustomerType::BUSINESS) 
            .count();
        println!("Number of business customers: {:?}", cnt);

        let cnt = self.customers
            .iter()
            .filter(|customer| customer.customer_type == CustomerType::MIXED) 
            .count();
        println!("Number of mixed customers: {:?}", cnt);

        let cnt = self.customers
            .iter()
            .filter(|customer| customer.customer_type == CustomerType::SIMBOX) 
            .count();
        println!("Number of simbox customers: {:?}", cnt);

        let cnt = self.customers
            .iter()
            .filter(|customer| customer.customer_type == CustomerType::PROBE) 
            .count();
        println!("Number of probe customers: {:?}", cnt);

        let cnt = self.customers
            .iter()
            .filter(|customer| customer.customer_type == CustomerType::MULTISIM) 
            .count();
        println!("Number of multisim customers: {:?}", cnt);

        println!("Number of relations: {:?}", self.relations.len());
        let cnt = self.relations
            .iter()
            .filter(|relation| relation.from.customer_type == CustomerType::PRIVATE) 
            .count();
        println!("Number of private relations: {:?}", cnt);

        let cnt = self.relations
            .iter()
            .filter(|relation| relation.from.customer_type == CustomerType::BUSINESS) 
            .count();
        println!("Number of business relations: {:?}", cnt);

        let cnt = self.relations
            .iter()
            .filter(|relation| relation.from.customer_type == CustomerType::MIXED) 
            .count();
        println!("Number of mixed relations: {:?}", cnt);

        let cnt = self.relations
            .iter()
            .filter(|relation| relation.from.customer_type == CustomerType::SIMBOX) 
            .count();
        println!("Number of simbox relations: {:?}", cnt);

        let cnt = self.relations
            .iter()
            .filter(|relation| relation.from.customer_type == CustomerType::PROBE) 
            .count();
        println!("Number of probe relations: {:?}", cnt);

        let cnt = self.relations
            .iter()
            .filter(|relation| relation.from.customer_type == CustomerType::MULTISIM) 
            .count();
        println!("Number of multisim relations: {:?}", cnt);


    }

    pub fn generate_operators(_number_of_local_operators:u16, _number_of_intl_operators: u16)-> Vec<Operator> {

        let mut operators: Vec<Operator> = Vec::with_capacity((_number_of_local_operators + _number_of_intl_operators) as usize);

        for i in 0.._number_of_local_operators{
            operators.push(Operator{operator_id:(i as u16), intl:0});
        }

        for i in _number_of_local_operators + 1.._number_of_intl_operators + _number_of_local_operators{
            operators.push(Operator{operator_id:i as u16, intl:1});
        }

        return operators;
        
    }

    pub fn generate_customers(_cfg: Config,  _operators:Vec<Operator>) -> Vec<Customer>  {

        let mut loc_customers: Vec<Customer> = Vec::with_capacity((_cfg.clone().market.number_of_local_customers + _cfg.clone().market.number_of_intl_customers) as usize);
        let mut rng = rand::thread_rng();
        let intl_perc = _cfg.market.number_of_intl_customers as f64 / (_cfg.market.number_of_intl_customers as f32+ _cfg.market.number_of_local_customers as f32)as f64;
        let mut msisdn_pos = 0;

        for (scenario_name, profile) in &_cfg.scenarios {
            let number_of_customers = (profile.profile_occurence_prc  * _cfg.market.number_of_local_customers as f32) as u32;

            for i in 0.. number_of_customers{
                let op = _operators[get_operator_id(_cfg.clone().market.local_operators)  as usize].clone();
                let loc_imei = get_imei(_cfg.clone().market.number_of_local_customers + _cfg.clone().market.number_of_intl_customers);

                let max_msisdn_cnt = if profile.msisdn_per_imei > 1 {draw_integer_from_uniform(5, 50)} else {1};

                for k in 0..max_msisdn_cnt {
                    loc_customers.push(Customer{
                        scenario: scenario_name.to_string(),
                        imei: loc_imei, 
                        msisdn: msisdn_pos, 
                        intl: if rng.gen_bool(intl_perc) { 1 } else { 0 }, 
                        operator: op.clone(), 
                        customer_type: profile.customer_type,
                        customer_profile: profile.customer_profile,
                        avg_calls_cnt: get_call_cnt(scenario_name, _cfg.clone()),
                        avg_sms_cnt: get_sms_cnt(scenario_name, _cfg.clone()),
                        avg_mms_cnt: get_mms_cnt(scenario_name, _cfg.clone())
                    });

                    msisdn_pos = msisdn_pos + 1;
                }

            }

        }

        return loc_customers;

    }


    pub fn generate_relations(_cfg: Config, _customers:Vec<Customer>) -> Vec<Relation> {

        let mut loc_relations: Vec<Relation> = Vec::with_capacity(_customers.len() * 200);
        let mut rng = rand::thread_rng();
        let customers = _customers.clone();
        let mut loc_customers: Vec<Customer> = Vec::with_capacity((_cfg.clone().market.number_of_local_customers + _cfg.clone().market.number_of_intl_customers) as usize);
        
        for c in _customers{
            let profile_cfg = _cfg.scenarios.get(&(c.scenario)).unwrap(); 

            let friends_count = get_friend_cnt(profile_cfg.avg_relation_friends_cnt, profile_cfg.std_relation_friends_cnt);
            let family_count = get_family_cnt(profile_cfg.avg_relation_family_cnt, profile_cfg.std_relation_family_cnt);
            let business_count = get_family_cnt(profile_cfg.avg_relation_business_cnt, profile_cfg.std_relation_business_cnt);
            let other_count = get_other_cnt(profile_cfg.avg_relation_other_cnt, profile_cfg.std_relation_other_cnt);

                for j in 0..friends_count{
                    if rng.gen_bool(profile_cfg.relation_prob as f64){
                        let to_idx = rng.gen_range(0..=customers.len()-1);
                        let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::FRIEND};
                        loc_relations.push(rel);

                        if rng.gen_bool(profile_cfg.inverse_relation_prob as f64){
                            let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::FRIEND};
                            loc_relations.push(inv_rel);
                        }
                    }
                }

                for j in 0..family_count{
                    if rng.gen_bool(profile_cfg.relation_prob as f64){
                        let to_idx = rng.gen_range(0..=customers.len()-1);
                        let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::FAMILY};
                        loc_relations.push(rel);

                        if rng.gen_bool(profile_cfg.inverse_relation_prob as f64){
                            let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::FAMILY};
                            loc_relations.push(inv_rel);
                        }
                    }

                }

                for j in 0..business_count{
                    if rng.gen_bool(profile_cfg.relation_prob as f64){
                        let to_idx = rng.gen_range(0..=customers.len()-1);
                        let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::BUSINESS};
                        loc_relations.push(rel);

                        if rng.gen_bool(profile_cfg.inverse_relation_prob as f64){
                            let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::BUSINESS};
                            loc_relations.push(inv_rel);
                        }
                    }

                }

                for j in 0..other_count{
                    if rng.gen_bool(profile_cfg.relation_prob as f64){
                        let to_idx = rng.gen_range(0..=customers.len()-1);
                        let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::OTHER};
                        loc_relations.push(rel);

                        if rng.gen_bool(profile_cfg.inverse_relation_prob as f64){
                            let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::OTHER};
                            loc_relations.push(inv_rel);
                        }
                    }
                }
    
        
        }
        
        return loc_relations;

    }



    pub fn new( _cfg:Config) -> Self {
        let operators = Self::generate_operators(_cfg.clone().market.local_operators.len() as u16, _cfg.clone().market.intl_operators.len() as u16);
        let customers = Self::generate_customers(_cfg.clone(), operators.clone(), );
        let relations = Self::generate_relations(_cfg.clone(), customers.clone());

        Generator { operators, customers, relations, cfg:_cfg}
    }
    

    fn save_batch(&self, _batch:Vec<CDR>){
        
        let file_exists = Path::new(&self.cfg.technical.detailed_resut_filename).exists();
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.cfg.technical.detailed_resut_filename).unwrap();
    
        let mut wtr = WriterBuilder::new().has_headers(!file_exists).delimiter(b';').from_writer(file);
    
        for cdr in _batch {
            wtr.serialize(cdr).unwrap();
        }
    
        wtr.flush().unwrap();
    }

    
    pub fn get_customer_count(&self) -> u64{
        return self.customers.len() as u64;
    }

    pub fn get_operator_count(&self) -> u32{
        return self.operators.len() as u32;
    }

    pub fn get_relations_count(&self) -> u32{
        return self.relations.len() as u32;
    }

    pub fn generate_cdr(&self) -> u128{

        let mut cdr_batch: Vec<CDR> = Vec::with_capacity(self.cfg.technical.batch_size as usize);
        let mut pos: u32 = 0;
        let mut total_cnt: u128 = 0;
        let mut curr_rel: u64 = 0;

        let parts: Vec<&str> = self.cfg.technical.start_date.split('-').collect();
        let year = parts[0].parse::<u32>().unwrap();
        let month = parts[1].parse::<u32>().unwrap();

        let mut rng = rand::thread_rng();

        println!("Number of relations: {:?}", self.relations.len());
        let total_relations = self.relations.len();

        for r in &self.relations{

            curr_rel = curr_rel + 1;

            if curr_rel % 100000 == 0 {
                println!("completed: {}/{} relations, cdr batch: {}", curr_rel, total_relations, cdr_batch.len());
            }

            for c in 0..r.from.avg_calls_cnt{
                pos = pos + 1;
                if call_success(&r.from.scenario, self.cfg.clone()) {
                    total_cnt = total_cnt + 1;
                    
                    let bts_num = get_bts(&r.from.scenario, self.cfg.clone());

                    let tmp_cdr:CDR = CDR { 
                        cdr_id: Uuid::new_v4(),
                        originating_cdr_id: Uuid::new_v4(),
                        continued_in_cdr_id: Uuid::new_v4(), 
                        creation_method: get_creation_method(),
                        forward_reason: get_forward_reason(),
                        timestamp: get_start_time(year, month).to_string(), 
                        duration_sec: get_duration(&r.from.scenario, self.cfg.clone()), 
                        from_msisdn: r.from.msisdn, 
                        billing_msisdn: r.from.msisdn,
                        to_msisdn: r.to.msisdn, 
                        from_imei: r.from.imei, 
                        to_imei: r.to.imei, 
                        from_operator_id: r.from.operator.operator_id, 
                        to_operator_id: r.to.operator.operator_id, 
                        bts_id: bts_num, 
                        contact_type: ContactType::VOICE, 
                        customer_type: r.from.customer_type.clone(),
                        customer_profile: r.from.customer_profile.clone(),
                        voice_call_result: get_voice_call_result(),
                        roaming: r.from.operator.intl,
                        scenario: r.from.scenario.clone()
                    };

                        cdr_batch.push(tmp_cdr);

                    if pos >= self.cfg.technical.batch_size{
                        pos = 0;
                        self.save_batch(cdr_batch.clone());
                        cdr_batch.clear();
                    }

                }
            }

            for c in 0..r.from.avg_sms_cnt{
                pos = pos + 1;
                if rng.gen_range(0..10) > 6 {
                    total_cnt = total_cnt + 1;

                    let bts_num = get_bts(&r.from.scenario, self.cfg.clone());

                    let tmp_cdr:CDR = CDR { 
                        cdr_id: Uuid::new_v4(),
                        originating_cdr_id: Uuid::new_v4(),
                        continued_in_cdr_id: Uuid::new_v4(),  
                        creation_method: get_creation_method(),
                        forward_reason: get_forward_reason(),
                        timestamp: get_start_time(year, month).to_string(), 
                        duration_sec: 0, 
                        from_msisdn: r.from.msisdn,
                        billing_msisdn: r.from.msisdn, 
                        to_msisdn: r.to.msisdn, 
                        from_imei: r.from.imei, 
                        to_imei: r.to.imei, 
                        from_operator_id: r.from.operator.operator_id, 
                        to_operator_id: r.to.operator.operator_id, 
                        bts_id: bts_num, 
                        contact_type: ContactType::SMS, 
                        customer_type: r.from.customer_type.clone(),
                        customer_profile: r.from.customer_profile.clone(),
                        voice_call_result: get_voice_call_result(),
                        roaming: r.from.operator.intl, 
                        scenario: r.from.scenario.clone()
                    };

                        cdr_batch.push(tmp_cdr);

                    if pos >= self.cfg.technical.batch_size{
                        pos = 0;
                        self.save_batch(cdr_batch.clone());
                        cdr_batch.clear();
                    }

                }
            }
        
            for c in 0..r.from.avg_mms_cnt{
                pos = pos + 1;
                if rng.gen_range(0..10) > 7 {
                    total_cnt = total_cnt + 1;

                    let bts_num = get_bts(&r.from.scenario, self.cfg.clone());

                    let tmp_cdr:CDR = CDR { 
                        cdr_id: Uuid::new_v4(),
                        originating_cdr_id: Uuid::new_v4(),
                        continued_in_cdr_id: Uuid::new_v4(),  
                        creation_method: get_creation_method(),
                        forward_reason: get_forward_reason(), 
                        timestamp: get_start_time(year, month).to_string(), 
                        duration_sec: 0, 
                        from_msisdn: r.from.msisdn, 
                        to_msisdn: r.to.msisdn, 
                        from_imei: r.from.imei,
                        billing_msisdn: r.from.msisdn, 
                        to_imei: r.to.imei, 
                        from_operator_id: r.from.operator.operator_id, 
                        to_operator_id: r.to.operator.operator_id, 
                        bts_id: bts_num, 
                        contact_type: ContactType::MMS, 
                        customer_type: r.from.customer_type.clone(),
                        customer_profile: r.from.customer_profile.clone(),
                        voice_call_result: get_voice_call_result(),
                        roaming: r.from.operator.intl, 
                        scenario: r.from.scenario.clone()
                    };

                        cdr_batch.push(tmp_cdr);

                    if pos >= self.cfg.technical.batch_size{
                        pos = 0;
                        self.save_batch(cdr_batch.clone());
                        cdr_batch.clear();
                    }

                }
            }

        }

//random noise for 5%

        for i in 0..((self.cfg.market.number_of_local_customers as f32 * self.cfg.market.random_noise_prc) as u32){
            let from_idx = rng.gen_range(0..=self.customers.len()-1);
            let to_idx = rng.gen_range(0..=self.customers.len()-1);

            let tmp_cdr:CDR = CDR { 
                        cdr_id: Uuid::new_v4(),
                        originating_cdr_id: Uuid::new_v4(),
                        continued_in_cdr_id: Uuid::new_v4(),  
                        creation_method: get_creation_method(),
                        forward_reason: get_forward_reason(), 
                        timestamp: get_start_time(year, month).to_string(), 
                        duration_sec: 0, 
                        from_msisdn: self.customers[from_idx].msisdn, 
                        to_msisdn: self.customers[to_idx].msisdn, 
                        from_imei: self.customers[from_idx].imei,
                        billing_msisdn: self.customers[from_idx].msisdn, 
                        to_imei: self.customers[to_idx].imei, 
                        from_operator_id: self.customers[from_idx].operator.operator_id, 
                        to_operator_id: self.customers[to_idx].operator.operator_id, 
                        bts_id:  rng.gen_range(0..=self.cfg.market.number_of_own_bts as u32), 
                        contact_type: ContactType::VOICE, 
                        customer_type: self.customers[from_idx].clone().customer_type,
                        customer_profile: self.customers[from_idx].clone().customer_profile,
                        voice_call_result: get_voice_call_result(),
                        roaming: if rng.gen_bool(0.05) { 1 } else { 0 }, 
                        scenario: self.customers[from_idx].clone().scenario
                    };

                cdr_batch.push(tmp_cdr);

            if pos >= self.cfg.technical.batch_size{
                pos = 0;
                self.save_batch(cdr_batch.clone());
                cdr_batch.clear();
            }
        }

        self.save_batch(cdr_batch.clone());

        return total_cnt;


    }

}


