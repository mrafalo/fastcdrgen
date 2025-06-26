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
# General
number_of_local_customers = 10000
number_of_intl_customers = 100
number_of_local_operators = 4
number_of_intl_operators = 2
number_of_bts = 100
simbox_prc = 0.005
probe_prc = 0.05
multisim_prc = 0.01
priv_prc = 0.7

# Relations
avg_relation_friends_cnt_priv = 5
std_relation_friends_cnt_priv = 10
avg_relation_family_cnt_priv = 5
std_relation_family_cnt_priv = 3
avg_relation_other_cnt_priv = 3
std_relation_other_cnt_priv = 5
avg_relation_friends_cnt_business = 10
std_relation_friends_cnt_business = 4
avg_relation_family_cnt_business = 3
std_relation_family_cnt_business = 3
avg_relation_other_cnt_business = 10
std_relation_other_cnt_business = 10
avg_relation_cnt_probe = 20
std_relation_cnt_probe = 20
avg_relation_cnt_simbox = 20
std_relation_cnt_simbox = 30
avg_relation_cnt_multi = 40
std_relation_cnt_multi = 20

# Calls
avg_calls_cnt_per_relation_probe = 20
stdev_calls_cnt_per_relation_probe = 10
avg_calls_cnt_per_relation_simbox = 20
stdev_calls_cnt_per_relation_simbox = 10
avg_calls_cnt_per_relation_multi = 20
stdev_calls_cnt_per_relation_multi = 5
avg_calls_cnt_per_relation = 4
stdev_calls_cnt_per_relation = 4
avg_sms_cnt_per_relation = 4
stdev_sms_cnt_per_relation = 8
avg_mms_cnt_per_relation = 4
stdev_mms_cnt_per_relation = 4

# Durations
avg_call_duration_priv = 413
std_call_duration_priv = 60
avg_call_duration_business = 600
std_call_duration_business = 70
avg_call_duration_probe = 413
std_call_duration_probe = 60
avg_call_duration_simbox = 413
std_call_duration_simbox = 60
avg_call_duration_multi = 415
std_call_duration_multi = 60

avg_sms_cnt = 3
avg_mms_cnt = 2

# Distributions
simbox_duration_distribution = 'WEIBULL'
probe_duration_distribution = 'POISON'
business_duration_distribution = 'NORMAL'
priv_duration_distribution = 'NORMAL'
multi_duration_distribution = 'NORMAL'

# Technical
start_date = '2024-05-01'
batch_size = 100000
detailed_resut_filename = 'results/cdr.csv'
agg_resut_filename = 'results/cdr_agg.csv'
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
