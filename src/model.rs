use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct WFDBChannel {
    pub file_name: String,
    pub fmt: String,
    pub samps_per_frame: String,
    pub skew: String,
    pub byte_offset: String,
    pub adc_gain: String,
    pub baseline: String,
    // pub units: String,
    // pub adc_res: String,
    // pub adc_zero: String,
    // pub init_value: String,
    // pub checksum: String,
    // pub block_size: String,
    // pub sig_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WFDBHeader {
    pub record_name: String,
    pub n_seg: String,
    pub n_sig: i32,
    pub fs: i32,
    pub counter_freq: String,
    pub base_counter: String,
    pub sig_len: i64,

    pub channels: Vec<WFDBChannel>,
}

impl WFDBHeader {
    pub fn build_general_header(contents: &str) -> WFDBHeader {
        let re = Regex::new(r"(?P<record_name>[-\w]+)/?(?P<n_seg>\d*)[ \t]+(?P<n_sig>\d+)[ \t]*(?P<fs>\d*\.?\d*)/*(?P<counter_freq>-?\d*\.?\d*)\(?(?P<base_counter>-?\d*\.?\d*)\)?[ \t]*(?P<sig_len>\d*)[ \t]*").unwrap();
        let caps = re.captures(&contents).unwrap();

        WFDBHeader {
            record_name: caps["record_name"].to_string(),
            n_seg: caps["n_seg"].to_string().parse().unwrap(),
            n_sig: caps["n_sig"].to_string().parse().unwrap(),
            fs: caps["fs"].to_string().parse().unwrap_or(250),
            counter_freq: caps["counter_freq"].to_string(),
            base_counter: caps["base_counter"].to_string(),
            sig_len: caps["sig_len"].to_string().parse().unwrap(),
            channels: Vec::new(),
        }
    }

    pub fn build_channel_header(&mut self, contents: Vec<&str>) {
        let re = Regex::new(r"(?P<file_name>~?[-\w]*\.?[\w]*)[ \t]+(?P<fmt>\d+)x?(?P<samps_per_frame>\d*):?(?P<skew>\d*)\+?(?P<byte_offset>\d*)[ \t]*(?P<adc_gain>-?\d*\.?\d*e?[\+-]?\d*)\(?(?P<baseline>-?\d*)\)?").unwrap();
       
        self.channels = contents.iter().map(|text| {
            let caps = re.captures(text).unwrap();
            WFDBChannel {
                file_name: caps["file_name"].to_string(),
                fmt: caps["fmt"].to_string(),
                samps_per_frame: caps["samps_per_frame"].to_string(),
                skew: caps["skew"].to_string(),
                byte_offset: caps["byte_offset"].to_string(),
                adc_gain: caps["adc_gain"].to_string(),
                baseline: caps["baseline"].to_string(),
                // units: caps["units"].to_string(),
                // adc_res: caps["adc_res"].to_string(),
                // adc_zero: caps["adc_zero"].to_string(),
                // init_value: caps["init_value"].to_string(),
                // checksum: caps["checksum"].to_string(),
                // block_size: caps["block_size"].to_string(),
                // sig_name: caps["sig_name"].to_string(),
            }
        }).collect();
    }
}