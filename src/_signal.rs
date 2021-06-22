use std::fs;
use std::io::Seek;
use std::io::Read;
use std::io::SeekFrom;
use std::error::Error;


#[derive(Debug, Clone, PartialEq)]
pub struct WFDBSignal {
    pub signal:Vec<u16>,
}
impl WFDBSignal {
    // ! signal[n] = buffer[2n] + buffer[2n+1]
    // ! p_signal[n] = ((buffer[2n] + buffer[2n+1]) - baseline) / adc_gain
    pub fn new(filename: &str, start:u64, length:usize) -> Result<WFDBSignal, Box<dyn Error>> {
        let mut f = fs::File::open(filename)?;
        let mut buffer = vec![0; length*2];
        f.seek(SeekFrom::Start(start))?;
        f.read(&mut buffer)?;

        let result:Vec<u16> = buffer
            .iter()
            .step_by(2)
            .zip(buffer.iter().skip(1).step_by(2))
            .map(|(a, b)| set_u16_le(&[*a, *b]))
            .collect();

        Ok(WFDBSignal {
            signal: result,
        })
    }
}

fn set_u16_le(a: &[u8; 2]) -> u16 {
    (a[0] as u16) + ((a[1] as u16) >> 8)
}