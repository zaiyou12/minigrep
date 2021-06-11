use regex::Regex;

/// Notes
/// -----
/// Comments from wfdb-python
/// 
/// In the original WFDB package, cetain fields have default value, but
/// not all of them. Some attributes need to be present for core
/// functionality, i.e. baseline, whereas others are not essential, yet have
/// defaults, i.e. base_time.assert_eq!
/// 
/// This inconsistency has likely resulted in the generation of incorrect
/// files, and general confusion. This library aims to make explicit,
/// whether certain fields are present in the file, by setting their values
/// to None if they are not written in, unless the fields are essential, in
/// which case an actual default value will be set.
/// 
/// The read vs write default values are different for 2 reasons:
/// 1. We want to force the user to be explicit with certain important
///    fields when writing WFDB records fields, without affecting
///    existing WFDB headers when reading.
/// 2. Certain unimportant fields may be dependencies of other
///    important fields. When writing, we want to fill in defaults
///    so that the user doesn't need to. But when reading, it should
///    be clear that the fields are missing.
/// 
/// If all of the fields were filled out in a WFDB header file, they would appear
/// in this order with these seperators:
/// 
/// RECORD_NAME/NUM_SEG NUM_SIG SAMP_FREQ/COUNT_FREQ(BASE_COUNT_VAL) SAMPS_PER_SIG BASE_TIME BASE_DATE
/// FILE_NAME FORMATxSAMP_PER_FRAME:SKEW+BYTE_OFFSET ADC_GAIN(BASELINE)/UNITS ADC_RES ADC_ZERO CHECKSUM BLOCK_SIZE DESCRIPTION

#[derive(Debug, Clone, PartialEq)]
pub struct WFDBChannel {
    /// field: dependency, read_default, write_default
    /// file_name: None, None, None
    pub file_name: String,
    /// fmt: file_name, None, None
    pub fmt: i32,
    /// samps_per_frame: fmt, 1, None
    pub samps_per_frame: i32,
    /// skew: fmt, None, None
    pub skew: i32,
    /// byte_offset: fmt, None, None
    pub byte_offset: i32,
    /// adc_gain: fmt, 200.0, None
    pub adc_gain: i32,
    /// baseline: adc_gain, 0, None
    pub baseline: i32,
    /// units: adc_gain, 'mV', None
    pub units: String,
    /// adc_res: adc_gain, None, 0
    pub adc_res: i32,
    /// adc_zero: adc_res, None, 0
    pub adc_zero: i32,
    /// init_value: adc_zero, None, None
    pub init_value: i32,
    /// checksum: init_value, None, None
    pub checksum: i32,
    /// block_size: checksum, None, 0
    pub block_size: i32,
    /// sig_name: block_size, None, None
    pub sig_name: String,
}


#[derive(Debug, Clone, PartialEq)]
pub struct WFDBHeader {
    /// field: dependency, read_default, write_default
    /// record_name: None, None, None
    pub record_name: String,
    /// n_seg: record_name, None, None
    pub n_seg: String,
    /// n_sig: record_name, None, None
    pub n_sig: i32,
    /// fs: n_sig, 250, None
    pub fs: i32,
    /// counter_freq: fs, None, None
    pub counter_freq: String,
    /// base_counter: counter_freq, None, None
    pub base_counter: String,
    /// sig_len: fs, None, None
    pub sig_len: i64,
    /// base_time: sig_len, None, '00:00:00'
    pub base_time: String,
    /// base_date: base_time, None, None
    pub base_date: String,

    pub channels: Vec<WFDBChannel>,
}

impl WFDBHeader {
    pub fn build_general_header(contents: &str) -> WFDBHeader {
        let re = Regex::new(r"(?P<record_name>[-\w]+)/?(?P<n_seg>\d*)[ \t]+(?P<n_sig>\d+)[ \t]*(?P<fs>\d*\.?\d*)/*(?P<counter_freq>-?\d*\.?\d*)\(?(?P<base_counter>-?\d*\.?\d*)\)?[ \t]*(?P<sig_len>\d*)[ \t]*(?P<base_time>\d{2}:?\d{2}:?\d{2})[ \t]*(?P<base_date>\d{2}/?\d{2}/?\d{4})").unwrap();
        let caps = re.captures(&contents).unwrap();

        WFDBHeader {
            record_name: caps["record_name"].to_string(),
            n_seg: caps["n_seg"].to_string().parse().unwrap(),
            n_sig: caps["n_sig"].to_string().parse().unwrap(),
            fs: caps["fs"].to_string().parse().unwrap_or(250),
            counter_freq: caps["counter_freq"].to_string(),
            base_counter: caps["base_counter"].to_string(),
            sig_len: caps["sig_len"].to_string().parse().unwrap(),
            base_time: caps["base_time"].to_string(),
            base_date: caps["base_date"].to_string(),
            channels: Vec::new(),
        }
    }

    pub fn build_channel_header(&mut self, contents: Vec<&str>) {
        let re = Regex::new(r"(?P<file_name>~?[-\w]*\.?[\w]*)[ \t]+(?P<fmt>\d+)x?(?P<samps_per_frame>\d*):?(?P<skew>\d*)\+?(?P<byte_offset>\d*)[ \t]*(?P<adc_gain>-?\d*\.?\d*e?[\+-]?\d*)\(?(?P<baseline>-?\d*)\)?/?(?P<units>[\w\^\-\?%]*)[ \t]*(?P<adc_res>\d*)[ \t]*(?P<adc_zero>-?\d*)[ \t]*(?P<init_value>-?\d*)[ \t]*(?P<checksum>-?\d*)[ \t]*(?P<block_size>\d*)[ \t]*(?P<sig_name>[\S]?[^\t\n\r\f\v]*)").unwrap();
       
        self.channels = contents.iter().map(|text| {
            let caps = re.captures(text).unwrap();
            WFDBChannel {
                file_name: caps["file_name"].to_string(),
                fmt: caps["fmt"].to_string().parse().unwrap_or(0),
                samps_per_frame: caps["samps_per_frame"].to_string().parse().unwrap_or(1),
                skew: caps["skew"].to_string().parse().unwrap_or(0),
                byte_offset: caps["byte_offset"].to_string().parse().unwrap_or(0),
                adc_gain: caps["adc_gain"].to_string().parse().unwrap_or(200),
                baseline: caps["baseline"].to_string().parse().unwrap_or(0),
                units: caps["units"].to_string(),
                adc_res: caps["adc_res"].to_string().parse().unwrap_or(0),
                adc_zero: caps["adc_zero"].to_string().parse().unwrap_or(0),
                init_value: caps["init_value"].to_string().parse().unwrap_or(0),
                checksum: caps["checksum"].to_string().parse().unwrap_or(0),
                block_size: caps["block_size"].to_string().parse().unwrap_or(0),
                sig_name: caps["sig_name"].to_string(),
            }
        }).collect();
    }
}