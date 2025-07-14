# Fast CDR Generator

`fastcdrgen` is a high-performance tool for generating massive Call Detail Record (CDR) datasets. Using **Markov Chains**, it simulates realistic call and message patterns for both local and international users, providing a configurable way to model telecom datasets. 

## Features
- **Markov Chain-Based Simulation**: Generates realistic CDRs by modeling user behavior and relationships.
- **Highly Configurable**: Control the number of users, operators, BTS (base transceiver stations), and call/message distributions.
- **Flexible Output**: Easily set simulation parameters such as call durations, relationship types, and output filenames.
- **Multiple Use Cases**: Useful for testing telecom fraud detection, network analysis, and other telecom-related research.

---

## Configuration
The tool uses a configuration file to control the simulation parameters. Below is a sample configuration:

```ini
[market]
number_of_local_customers = 10000
number_of_intl_customers = 50
local_operators = [0.4, 0.3, 0.3]
intl_operators = [0.5, 0.2, 0.3]
number_of_own_bts = 20
number_of_external_bts = 10
random_noise_prc = 0.05

[private]
customer_type = "PRIVATE"
customer_profile = "NORMAL"
profile_occurence_prc = 0.6
call_duration_distribution = "NORMAL"
avg_call_duration = 110
std_call_duration = 15
avg_relation_friends_cnt = 3
std_relation_friends_cnt = 4
avg_relation_family_cnt = 3
std_relation_family_cnt = 3
avg_relation_business_cnt = 2
std_relation_business_cnt = 7
avg_relation_other_cnt = 3
std_relation_other_cnt = 1
avg_calls_cnt_per_relation = 5
std_calls_cnt_per_relation = 3
avg_sms_cnt_per_relation = 5
std_sms_cnt_per_relation = 2
avg_mms_cnt_per_relation = 2
std_mms_cnt_per_relation = 1
msisdn_per_imei = 1
bts_used_prc = 0.8
call_success_prc = 0.7
inverse_relation_prob = 0.2
relation_prob = 0.2

[business]
customer_type = "BUSINESS"
customer_profile = "NORMAL"
profile_occurence_prc = 0.5
call_duration_distribution = "NORMAL"
avg_call_duration = 130
std_call_duration = 43
avg_relation_friends_cnt = 6
std_relation_friends_cnt = 4
avg_relation_family_cnt = 4
std_relation_family_cnt = 3
avg_relation_business_cnt = 10
std_relation_business_cnt = 7
avg_relation_other_cnt = 5
std_relation_other_cnt = 5
avg_calls_cnt_per_relation = 4
std_calls_cnt_per_relation = 5
avg_sms_cnt_per_relation = 10
std_sms_cnt_per_relation = 3
avg_mms_cnt_per_relation = 2
std_mms_cnt_per_relation = 1
msisdn_per_imei = 1
bts_used_prc = 0.9
call_success_prc = 0.5
inverse_relation_prob = 0.3
relation_prob = 0.2

[mixed]
customer_type = "MIXED"
customer_profile = "NORMAL"
profile_occurence_prc = 0.05
call_duration_distribution = "WEIBULL"
avg_call_duration = 100
std_call_duration = 20
avg_relation_friends_cnt = 3
std_relation_friends_cnt = 3
avg_relation_family_cnt = 3
std_relation_family_cnt = 3
avg_relation_business_cnt = 7
std_relation_business_cnt = 4
avg_relation_other_cnt = 5
std_relation_other_cnt = 5
avg_calls_cnt_per_relation = 4
std_calls_cnt_per_relation = 4
avg_sms_cnt_per_relation = 3
std_sms_cnt_per_relation = 3
avg_mms_cnt_per_relation = 2
std_mms_cnt_per_relation = 1
msisdn_per_imei = 1
bts_used_prc = 0.9
call_success_prc = 0.6
inverse_relation_prob = 0.1
relation_prob = 0.4

[multisim]
customer_type = "MULTISIM"
customer_profile = "NORMAL"
profile_occurence_prc = 0.001
call_duration_distribution = "WEIBULL"
avg_call_duration = 90
std_call_duration = 30
avg_relation_friends_cnt = 3
std_relation_friends_cnt = 1
avg_relation_family_cnt = 2
std_relation_family_cnt = 1
avg_relation_business_cnt = 3
std_relation_business_cnt = 2
avg_relation_other_cnt = 3
std_relation_other_cnt = 1
avg_calls_cnt_per_relation = 8
std_calls_cnt_per_relation = 3
avg_sms_cnt_per_relation = 4
std_sms_cnt_per_relation = 2
avg_mms_cnt_per_relation = 1
std_mms_cnt_per_relation = 1
msisdn_per_imei = 4
bts_used_prc = 0.3
call_success_prc = 0.4
inverse_relation_prob = 0.1
relation_prob = 0.1

[simbox]
customer_type = "SIMBOX"
customer_profile = "FRAUD"
profile_occurence_prc = 0.001
call_duration_distribution = "WEIBULL"
avg_call_duration = 110
std_call_duration = 15
avg_relation_friends_cnt = 6
std_relation_friends_cnt = 15
avg_relation_family_cnt = 3
std_relation_family_cnt = 3
avg_relation_business_cnt = 4
std_relation_business_cnt = 10
avg_relation_other_cnt = 5
std_relation_other_cnt = 5
avg_calls_cnt_per_relation = 11
std_calls_cnt_per_relation = 10
avg_sms_cnt_per_relation = 2
std_sms_cnt_per_relation = 2
avg_mms_cnt_per_relation = 1
std_mms_cnt_per_relation = 1
msisdn_per_imei = 50
bts_used_prc = 0.2
call_success_prc = 0.7
relation_prob = 0.4
inverse_relation_prob = 0.01

[probe]
customer_type = "PROBE"
customer_profile = "ANOMALY"
profile_occurence_prc = 0.001
call_duration_distribution = "WEIBULL"
avg_call_duration = 110
std_call_duration = 15
avg_relation_friends_cnt = 6
std_relation_friends_cnt = 15
avg_relation_family_cnt = 3
std_relation_family_cnt = 3
avg_relation_business_cnt = 4
std_relation_business_cnt = 10
avg_relation_other_cnt = 5
std_relation_other_cnt = 5
avg_calls_cnt_per_relation = 11
std_calls_cnt_per_relation = 10
avg_sms_cnt_per_relation = 2
std_sms_cnt_per_relation = 2
avg_mms_cnt_per_relation = 1
std_mms_cnt_per_relation = 1
msisdn_per_imei = 50
bts_used_prc = 0.2
call_success_prc = 0.8
relation_prob = 0.4
inverse_relation_prob = 0.01

[churn]
customer_type = "PRIVATE"
customer_profile = "CHURN"
profile_occurence_prc = 0.001
call_duration_distribution = "WEIBULL"
avg_call_duration = 110
std_call_duration = 15
avg_relation_friends_cnt = 3
std_relation_friends_cnt = 3
avg_relation_family_cnt = 3
std_relation_family_cnt = 3
avg_relation_business_cnt = 5
std_relation_business_cnt = 2
avg_relation_other_cnt = 5
std_relation_other_cnt = 5
avg_calls_cnt_per_relation = 5
std_calls_cnt_per_relation = 1
avg_sms_cnt_per_relation = 2
std_sms_cnt_per_relation = 2
avg_mms_cnt_per_relation = 1
std_mms_cnt_per_relation = 1
msisdn_per_imei = 1
bts_used_prc = 0.9
call_success_prc = 0.7
relation_prob = 0.2
inverse_relation_prob = 0.2

[technical]
start_date = "2025-01-01"
batch_size = 200000
detailed_resut_filename = "results/sample.csv"
agg_resut_filename = "results/cdr4_agg.csv"

```

---

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/fastcdrgen.git
   ```
2. Navigate to the project directory:
   ```bash
   cd fastcdrgen
   ```
3. Build the project using Rust's package manager, `cargo`:
   ```bash
   cargo build --release
   ```

---

## Usage
1. **Prepare Configuration File**:
   Customize the sample configuration file and save it as `config.toml`.

2. **Run the Generator**:
   Use the following command to generate the CDR dataset:
   ```bash
   ./fastcdrgen
   ```

3. **Output**:
   The generated dataset will be saved to the file specified in `resut_filename`.

---

## License
Copyright (c) 2025 Mariusz Rafało

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

---

## Contribution
Contributions are welcome! If you'd like to contribute, please:
1. Fork the repository.
2. Create a feature branch.
3. Submit a pull request.

---

For any inquiries or support, feel free to reach out.

---

## Roadmap
- Add support for more duration distributions.
- Implement additional fraud simulation scenarios.
- Improve performance for extremely large datasets.
