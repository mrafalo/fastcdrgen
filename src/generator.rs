use std::{fs::{File, OpenOptions}, path::Path};
use csv::{Writer, WriterBuilder};
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, WeightedIndex};

use crate::{cdr::{self, ContactType, CDR}, config::Config, consts::{self, DistributionType}, customer::{Customer, CustomerType}, operator::Operator, relation::{Relation, RelationType}, utils::{self, draw_integer_from_uniform}};
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

        let cnt = self.relations
            .iter()
            .filter(|relation| relation.from.customer_type == CustomerType::PRIVATE) 
            .count();
        println!("Number of private relations: {:?}", cnt);

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

        let mut loc_customers: Vec<Customer> = Vec::with_capacity((_cfg.number_of_local_customers + _cfg.number_of_intl_customers) as usize);
        let mut msisdn_pos = 0;
        
        for i in 0.._cfg.number_of_local_customers{
            let op = _operators[draw_integer_from_uniform(0, _operators.len() as u32)  as usize].clone();

            loc_customers.push(Customer{
                imei: get_imei(_cfg.number_of_local_customers  + _cfg.number_of_intl_customers ), 
                msisdn: i, 
                intl: 0, 
                operator:op, 
                customer_type: get_customer_type(_cfg.priv_prc), 
                avg_calls_cnt:get_call_cnt(_cfg.avg_calls_cnt_per_relation, _cfg.stdev_calls_cnt_per_relation, DistributionType::NORMAL),
                avg_sms_cnt:get_sms_cnt(_cfg.avg_sms_cnt_per_relation, _cfg.stdev_sms_cnt_per_relation),
                avg_mms_cnt:get_mms_cnt(_cfg.avg_mms_cnt_per_relation, _cfg.stdev_mms_cnt_per_relation)
            });
        }

        for i in _cfg.number_of_local_customers + 1.._cfg.number_of_local_customers + _cfg.number_of_intl_customers{
            let op = _operators[draw_integer_from_uniform(0, _operators.len() as u32)  as usize].clone();
            loc_customers.push(Customer{
                imei:get_imei(_cfg.number_of_local_customers + _cfg.number_of_intl_customers), 
                msisdn:i, 
                intl: 1, 
                operator:op, 
                customer_type: get_customer_type(_cfg.priv_prc), 
                avg_calls_cnt:get_call_cnt(_cfg.avg_calls_cnt_per_relation, _cfg.stdev_calls_cnt_per_relation, DistributionType::NORMAL),
                avg_sms_cnt:get_sms_cnt(_cfg.avg_sms_cnt_per_relation, _cfg.stdev_sms_cnt_per_relation),
                avg_mms_cnt:get_mms_cnt(_cfg.avg_mms_cnt_per_relation, _cfg.stdev_mms_cnt_per_relation)
            });
        }

        msisdn_pos = _cfg.number_of_local_customers + _cfg.number_of_intl_customers + 100;

        //PROBE
        for i in msisdn_pos..msisdn_pos + draw_integer_from_uniform(1, (_cfg.probe_prc * _cfg.number_of_local_customers as f32) as u32 + 1){
            let op = _operators[draw_integer_from_uniform(0, _operators.len() as u32)  as usize].clone();
            let loc_imei = get_imei(_cfg.number_of_local_customers + _cfg.number_of_intl_customers);

            for j in 0..draw_integer_from_uniform(5, 50){
                msisdn_pos = msisdn_pos + 1;

                loc_customers.push(Customer{
                    imei:loc_imei, 
                    msisdn:i, 
                    intl: 0, 
                    operator:op.clone(), 
                    customer_type:CustomerType::PROBE, 
                    avg_calls_cnt:get_call_cnt(_cfg.avg_calls_cnt_per_relation_probe, _cfg.stdev_calls_cnt_per_relation_probe, DistributionType::NORMAL),
                    avg_sms_cnt:get_sms_cnt(_cfg.avg_sms_cnt_per_relation, _cfg.stdev_sms_cnt_per_relation),
                    avg_mms_cnt:get_mms_cnt(_cfg.avg_mms_cnt_per_relation, _cfg.stdev_mms_cnt_per_relation)
                });
            }
        }

        //SIMBOX
        for i in msisdn_pos..msisdn_pos + draw_integer_from_uniform(1, (_cfg.simbox_prc * _cfg.number_of_local_customers as f32) as u32 + 1){
            let op = _operators[draw_integer_from_uniform(0, _operators.len() as u32)  as usize].clone();
            let loc_imei = get_imei(_cfg.number_of_local_customers + _cfg.number_of_intl_customers);

            for j in 0..draw_integer_from_uniform(10, 50){
                msisdn_pos = msisdn_pos + 1;
                loc_customers.push(Customer{
                    imei:loc_imei, 
                    msisdn:i, 
                    intl: 0, 
                    operator:op.clone(), 
                    customer_type:CustomerType::SIMBOX, 
                    avg_calls_cnt:get_call_cnt(_cfg.avg_calls_cnt_per_relation_simbox, _cfg.stdev_calls_cnt_per_relation_simbox, DistributionType::NORMAL),
                    avg_sms_cnt:get_sms_cnt(_cfg.avg_sms_cnt_per_relation / 3, _cfg.stdev_sms_cnt_per_relation / 2),
                    avg_mms_cnt:get_mms_cnt(_cfg.avg_mms_cnt_per_relation / 3, _cfg.stdev_mms_cnt_per_relation / 2)
                });
            }
        }

        //MULTISIM
        for i in msisdn_pos..msisdn_pos + draw_integer_from_uniform(1, (_cfg.multisim_prc * _cfg.number_of_local_customers as f32) as u32 + 3){
            let op = _operators[draw_integer_from_uniform(0, _operators.len() as u32)  as usize].clone();
            let loc_imei = get_imei(_cfg.number_of_local_customers + _cfg.number_of_intl_customers);

            for j in 0..draw_integer_from_uniform(1, 10){
                msisdn_pos = msisdn_pos + 1;
                loc_customers.push(Customer{
                    imei:loc_imei, 
                    msisdn:i, 
                    intl: 0, 
                    operator:op.clone(), 
                    customer_type:CustomerType::MULTISIM, 
                    avg_calls_cnt:get_call_cnt(_cfg.avg_calls_cnt_per_relation_multi, _cfg.stdev_calls_cnt_per_relation_multi, DistributionType::NORMAL),
                    avg_sms_cnt:get_sms_cnt(_cfg.avg_sms_cnt_per_relation * 2, _cfg.stdev_sms_cnt_per_relation * 3),
                    avg_mms_cnt:get_mms_cnt(_cfg.avg_mms_cnt_per_relation * 2, _cfg.stdev_mms_cnt_per_relation * 3)
                });
            }
        }

        return loc_customers;

    }


    pub fn generate_relations(_cfg: Config, _customers:Vec<Customer>) -> Vec<Relation> {

        let mut loc_relations: Vec<Relation> = Vec::with_capacity(_customers.len() * 200);
        
        let mut rng = rand::thread_rng();

        let customers = _customers.clone();

        for c in _customers{

            if c.customer_type == CustomerType::PRIVATE {

                let friends_count = get_friend_cnt(_cfg.avg_relation_friends_cnt_priv, _cfg.std_relation_friends_cnt_priv);
                let family_count = get_family_cnt(_cfg.avg_relation_family_cnt_priv, _cfg.std_relation_family_cnt_priv);
                let other_count = get_other_cnt(_cfg.avg_relation_other_cnt_priv, _cfg.std_relation_other_cnt_priv);

                for j in 0..friends_count{
                    let to_idx = rng.gen_range(0..=customers.len()-1);
                    let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::FRIEND};
                    loc_relations.push(rel);

                    if rng.gen_range(0..10) > 3{
                        let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::FRIEND};
                        loc_relations.push(inv_rel);
                    }
                }

                for j in 0..family_count{
                    let to_idx = rng.gen_range(0..=customers.len()-1);
                    let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::FAMILY};
                    loc_relations.push(rel);

                    if rng.gen_range(0..10) > 3{
                        let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::FAMILY};
                        loc_relations.push(inv_rel);
                    }

                }

                for j in 0..other_count{
                    let to_idx = rng.gen_range(0..=customers.len()-1);
                    let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::OTHER};
                    loc_relations.push(rel);
                    if rng.gen_range(0..10) > 8{
                        let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::OTHER};
                        loc_relations.push(inv_rel);
                    }
                }
            }

            if c.customer_type == CustomerType::BUSINESS {
                let friends_count = get_friend_cnt(_cfg.avg_relation_friends_cnt_business, _cfg.std_relation_friends_cnt_business);
                let family_count = get_family_cnt(_cfg.avg_relation_family_cnt_business, _cfg.std_relation_family_cnt_business);
                let other_count = get_other_cnt(_cfg.avg_relation_other_cnt_business, _cfg.std_relation_other_cnt_business);

                for j in 0..friends_count{
                    let to_idx = rng.gen_range(0..=customers.len()-1);
                    let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::FRIEND};
                    loc_relations.push(rel);

                    if rng.gen_range(0..10) > 5{
                        let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::FRIEND};
                        loc_relations.push(inv_rel);
                    }
                }

                for j in 0..family_count{
                    let to_idx = rng.gen_range(0..=customers.len()-1);
                    let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::FAMILY};
                    loc_relations.push(rel);

                    if rng.gen_range(0..10) > 3{
                        let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::FAMILY};
                        loc_relations.push(inv_rel);
                    }

                }

                for j in 0..other_count{
                    let to_idx = rng.gen_range(0..=customers.len()-1);
                    let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::OTHER};
                    loc_relations.push(rel);
                    if rng.gen_range(0..10) > 5{
                        let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::OTHER};
                        loc_relations.push(inv_rel);
                    }
                }
            }

            if c.customer_type == CustomerType::PROBE {
                let other_count = get_other_cnt(_cfg.avg_relation_cnt_probe, _cfg.std_relation_cnt_probe);
                for j in 0..other_count{
                    let to_idx = rng.gen_range(0..=customers.len()-1);
                    let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::OTHER};
                    loc_relations.push(rel);

                    if rng.gen_range(0..100) > 88{
                        let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::OTHER};
                        loc_relations.push(inv_rel);
                    }

                }
            }

            if c.customer_type == CustomerType::SIMBOX {
                let other_count = get_other_cnt(_cfg.avg_relation_cnt_simbox, _cfg.std_relation_cnt_simbox);
                for j in 0..other_count{
                    let to_idx = rng.gen_range(0..=customers.len()-1);
                    let rel = Relation{from:c.clone(), to:customers[to_idx].clone(), relation_type:RelationType::OTHER};
                    loc_relations.push(rel);

                    if rng.gen_range(0..100) > 88{
                        let inv_rel = Relation{to:c.clone(), from:customers[to_idx].clone(), relation_type:RelationType::OTHER};
                        loc_relations.push(inv_rel);
                    }
                }
            }

            if c.customer_type == CustomerType::MULTISIM {
                let other_count = get_other_cnt(_cfg.avg_relation_cnt_multi, _cfg.std_relation_cnt_multi);
                for j in 0..other_count{
                    let rel = Relation{from:c.clone(), to:customers[rng.gen_range(0..=customers.len()-1)].clone(), relation_type:RelationType::OTHER};
                    loc_relations.push(rel);

                    if rng.gen_range(0..100) > 55{
                        let inv_rel = Relation{to:c.clone(), from:customers[rng.gen_range(0..=customers.len()-1)].clone(), relation_type:RelationType::OTHER};
                        loc_relations.push(inv_rel);
                    }
                }
            }

        }

        return loc_relations;

    }



    pub fn new( _cfg:Config) -> Self {
        let operators = Self::generate_operators(_cfg.number_of_local_operators, _cfg.number_of_intl_operators);
        let customers = Self::generate_customers(_cfg.clone(), operators.clone(), );
        let relations = Self::generate_relations(_cfg.clone(), customers.clone());

        Generator { operators, customers, relations, cfg:_cfg}
    }
    

    fn save_batch(&self, _batch:Vec<CDR>){
        
        let file_exists = Path::new(&self.cfg.resut_filename).exists();
        // let file = File::create(_dest).unwrap();
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.cfg.resut_filename).unwrap();
    
        let mut wtr = WriterBuilder::new().has_headers(!file_exists).from_writer(file);
    
        for cdr in _batch {
            wtr.serialize(cdr);
        }
    
        wtr.flush();
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

        let mut cdr_batch: Vec<CDR> = Vec::with_capacity(self.cfg.batch_size as usize);
        let mut pos: u32 = 0;
        let mut total_cnt: u128 = 0;
        let mut curr_rel: u64 = 0;

        let parts: Vec<&str> = self.cfg.start_date.split('-').collect();
        let year = parts[0].parse::<u32>().unwrap();
        let month = parts[1].parse::<u32>().unwrap();

        let mut rng = rand::thread_rng();

        println!("Number of relations: {:?}", self.relations.len());
        let total_relations = self.relations.len();

        for r in &self.relations{

            curr_rel = curr_rel + 1;

            if curr_rel % 20000 == 0 {
                println!("Completed {}/{} relations", curr_rel, total_relations);
            }


            for c in 0..r.from.avg_calls_cnt{
                pos = pos + 1;
                if call_success(&r.from.customer_type) {
                    total_cnt = total_cnt + 1;

                    let mut bts_num = 0;
                    let mut probe = 0;
                    let mut simbox = 0;
                    
                    if &r.from.customer_type == &CustomerType::PRIVATE {
                        bts_num = get_bts(self.cfg.number_of_bts as u32);
                    }

                    if &r.from.customer_type == &CustomerType::BUSINESS {
                        bts_num = get_bts(self.cfg.number_of_bts as u32);
                    }

                    if &r.from.customer_type == &CustomerType::MULTISIM {
                        bts_num = get_bts(self.cfg.number_of_bts as u32);
                    }

                    if &r.from.customer_type == &CustomerType::PROBE {
                        bts_num = get_bts((self.cfg.number_of_bts/15) as u32);
                        probe = 1;
                    }

                    if &r.from.customer_type == &CustomerType::SIMBOX {
                        bts_num = get_bts((self.cfg.number_of_bts/15) as u32);
                        simbox = 1;
                    }

                    let tmp_cdr:CDR = CDR { 
                        timestamp: get_start_time(year, month).to_string(), 
                        duration_sec: self.get_duration(&r.from.customer_type), 
                        from_msisdn: r.from.msisdn, 
                        to_msisdn: r.to.msisdn, 
                        from_imei: r.from.imei, 
                        to_imei: r.to.imei, 
                        from_operator_id: r.from.operator.operator_id, 
                        to_operator_id: r.to.operator.operator_id, 
                        bts_id: bts_num, 
                        contact_type: ContactType::VOICE, 
                        customer_type: r.from.customer_type.clone(),
                        roaming: r.from.operator.intl, 
                        probe: probe, 
                        simbox: simbox };

                        cdr_batch.push(tmp_cdr);

                    if pos >= self.cfg.batch_size{
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

                    let mut bts_num = 0;
                    let mut probe = 0;
                    let mut simbox = 0;

                    if r.from.customer_type == CustomerType::PRIVATE {
                        bts_num = get_bts(self.cfg.number_of_bts as u32);
                    }

                    if r.from.customer_type == CustomerType::BUSINESS {
                        bts_num = get_bts(self.cfg.number_of_bts as u32)
                    }

                    if r.from.customer_type == CustomerType::PROBE {
                        bts_num = get_bts((self.cfg.number_of_bts/5) as u32);
                        probe = 1;
                    }

                    if r.from.customer_type == CustomerType::SIMBOX {
                        bts_num = get_bts((self.cfg.number_of_bts/10) as u32);
                        simbox = 1;
                    }

                    let tmp_cdr:CDR = CDR { 
                        timestamp: get_start_time(year, month).to_string(), 
                        duration_sec: 0, 
                        from_msisdn: r.from.msisdn, 
                        to_msisdn: r.to.msisdn, 
                        from_imei: r.from.imei, 
                        to_imei: r.to.imei, 
                        from_operator_id: r.from.operator.operator_id, 
                        to_operator_id: r.to.operator.operator_id, 
                        bts_id: bts_num, 
                        contact_type: ContactType::SMS, 
                        customer_type: r.from.customer_type.clone(),
                        roaming: r.from.operator.intl, 
                        probe: probe, 
                        simbox: simbox };

                        cdr_batch.push(tmp_cdr);

                    if pos >= self.cfg.batch_size{
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

                    let mut bts_num = 0;
                    let mut probe = 0;
                    let mut simbox = 0;

                    if r.from.customer_type == CustomerType::PRIVATE {
                        bts_num = get_bts(self.cfg.number_of_bts as u32);
                    }

                    if r.from.customer_type == CustomerType::BUSINESS {
                        bts_num = get_bts(self.cfg.number_of_bts as u32)
                    }

                    if r.from.customer_type == CustomerType::PROBE {
                        bts_num = get_bts((self.cfg.number_of_bts/5) as u32);
                        probe = 1;
                    }

                    if r.from.customer_type == CustomerType::SIMBOX {
                        bts_num = get_bts((self.cfg.number_of_bts/10) as u32);
                        simbox = 1;
                    }

                    let tmp_cdr:CDR = CDR { 
                        timestamp: get_start_time(year, month).to_string(), 
                        duration_sec: 0, 
                        from_msisdn: r.from.msisdn, 
                        to_msisdn: r.to.msisdn, 
                        from_imei: r.from.imei, 
                        to_imei: r.to.imei, 
                        from_operator_id: r.from.operator.operator_id, 
                        to_operator_id: r.to.operator.operator_id, 
                        bts_id: bts_num, 
                        contact_type: ContactType::MMS, 
                        customer_type: r.from.customer_type.clone(),
                        roaming: r.from.operator.intl, 
                        probe: probe, 
                        simbox: simbox };

                        cdr_batch.push(tmp_cdr);

                    if pos >= self.cfg.batch_size{
                        pos = 0;
                        self.save_batch(cdr_batch.clone());
                        cdr_batch.clear();
                    }

                }
            }

        }

        self.save_batch(cdr_batch.clone());

        return total_cnt;


    }

    pub fn get_duration(&self, _customer_type: &CustomerType) ->u32{

        let mut res:u32 = 0;
    
        match _customer_type{
            CustomerType::BUSINESS => res = draw_duration_from_distribution(self.cfg.avg_call_duration_business, self.cfg.std_call_duration_business, self.cfg.business_duration_distribution),
            CustomerType::MULTISIM => res = draw_duration_from_distribution(self.cfg.avg_call_duration_multi, self.cfg.std_call_duration_multi, self.cfg.multi_duration_distribution),
            CustomerType::PRIVATE => res = draw_duration_from_distribution(self.cfg.avg_call_duration_priv, self.cfg.std_call_duration_priv, self.cfg.priv_duration_distribution),
            CustomerType::PROBE => res = draw_duration_from_distribution(self.cfg.avg_call_duration_probe, self.cfg.std_call_duration_probe, self.cfg.probe_duration_distribution),
            CustomerType::SIMBOX => res = draw_duration_from_distribution(self.cfg.avg_call_duration_simbox, self.cfg.std_call_duration_simbox, self.cfg.simbox_duration_distribution),
    

        }
    
        // let res = utils::draw_integer_from_poisson(_mean as f32);
        return res as u32;
    } 

}



