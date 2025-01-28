
use std::time::Instant;

use generator::{cdr::CDR, config::Config, consts, customer::Customer, generator::Generator, operator::Operator, utils::{draw_integer_from_cauchy, draw_integer_from_normal, draw_integer_from_poisson, draw_integer_from_weibull}};
use config_file::FromConfigFile;


fn main() {

    let start = Instant::now();

    println!("Starting ...");
    let cfg = Config::from_config_file(consts::CONFIG_FILE).unwrap();
    let gen = Generator::new(cfg.clone());

    let total_cnt = gen.generate_cdr();
    let duration = start.elapsed();
    let elapsed_minutes = duration.as_secs_f64() / 60.0;

    println!("Elapsed time: {:.2} minutes...", elapsed_minutes );
    println!("Number of CDR: {:?}", total_cnt);

    gen.print_summary();

    // let avg = 413;

    // for i in 0..1000{
    //     let x1 = draw_integer_from_normal(avg as f32, 10.0);
    //     let x2 = draw_integer_from_poisson(avg as f32);
    //     let x3 = draw_integer_from_weibull(avg as f32);
    //     let x4 = draw_integer_from_cauchy(avg as f32);

    //     println!("normal: {:?}, poisson: {:?}, weilbull: {:?}, cauchy: {:?}", x1, x2, x3, x4);
    // }

}
