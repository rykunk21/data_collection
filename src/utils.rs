use regex::Regex;

pub trait U32Ext {
    fn from_time_str(str: &str) -> Result<u32, Box<dyn std::error::Error>>;
}

impl U32Ext for u32 {
    fn from_time_str(str: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let re = Regex::new(r"(?:\s*(\d+)\s*(?:hour|hr)s?)?(?:\s*(\d+)\s*(?:minute|min)s?)?")?;
        let caps = re.captures(str).ok_or("Failed to parse duration")?;

        let hours = caps.get(1).map_or(0, |m| {
            m.as_str().parse::<u32>().expect("Cannot parse hours")
        });
        let minutes = caps.get(2).map_or(0, |m| {
            m.as_str().parse::<u32>().expect("Cannot parse minutes")
        });

        Ok(hours * 60 + minutes)
    }
}
