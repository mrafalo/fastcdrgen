
use std::{path::Path, time::Instant};
use std::fs;
use generator::{ config::Config, consts, generator::Generator};
use config_file::FromConfigFile;
use polars::prelude::*;

fn remove_file_if_exists(_path: &str) {
    let file_path = Path::new(_path);

    if file_path.exists() {
        match fs::remove_file(file_path) {
            Ok(_) => println!("File '{}' removed.", _path),
            Err(e) => eprintln!("Failed to remove file '{}': {}", _path, e),
        }
    } 
}


fn aggregate_results(_source_path: &str, _dest_path: &str){


    let lf = LazyCsvReader::new(_source_path)
        .with_separator(b';')
        .finish()
        .unwrap();


    let g1 = lf.clone().group_by(["from_msisdn", "from_imei"])
        .agg(&[
            col("from_imei").n_unique().alias("caller_count"),
            col("duration_sec").mean().alias("mean_duration_sec"),
            col("bts_id").n_unique().alias("bts_cnt"),
            col("to_msisdn").count().alias("outgoing_cnt"),
            col("roaming").sum().alias("roaming_cnt"),
            col("simbox").max().alias("target"),
        ]);

     let g2 = lf.clone().group_by(["to_msisdn"])
        .agg(&[
            col("from_msisdn").count().alias("incoming_cnt"),
        ]);   


    let mut df = g1
        // .clone()
        // .lazy()
        .join(
            g2.clone().lazy(),
            [col("from_msisdn")],
            [col("to_msisdn")],
            JoinArgs::default(),
        )
        .collect().unwrap();


//   res.columns = ['caller_id', 'caller_imei', 'bts_cnt', 'outgoing_cnt', 'mean_duration_sec', 'roaming_cnt', 'target', 'caller_count','incoming_cnt']


    let mut file = std::fs::File::create(_dest_path).unwrap();
    let mut df = df.rename("from_msisdn", PlSmallStr::from_str("caller_id")).unwrap();

    CsvWriter::new(&mut file).with_separator(b';').finish(&mut df).unwrap();

    let res = df;
    println!("{}", res);
 
}


fn main() {

    let start = Instant::now();

    println!("Starting for config: {:?}...", consts::CONFIG_FILE);
    let cfg = Config::from_config_file(consts::CONFIG_FILE).unwrap();
    let gen = Generator::new(cfg.clone());

    for (name, profile) in &cfg.scenarios {
        println!("Profile name: {}", name);
        println!("  customer_profile: {:?}", profile.customer_profile);
        println!("  customer_type: {:?}", profile.customer_type);
    }


    remove_file_if_exists(&cfg.technical.detailed_resut_filename);
    remove_file_if_exists(&cfg.technical.agg_resut_filename);
    
    let total_cnt = gen.generate_cdr();
    let duration = start.elapsed();
    let elapsed_minutes = duration.as_secs_f64() / 60.0;

    println!("Elapsed time: {:.2} minutes...", elapsed_minutes );
    println!("Number of CDR generated: {:?}, result file: {:?}", total_cnt, cfg.technical.detailed_resut_filename);

    //aggregate_results(&cfg.detailed_resut_filename, &cfg.agg_resut_filename);

    gen.print_summary();

}
