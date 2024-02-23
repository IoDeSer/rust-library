use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::{delete_tabulator, from_io, IoDeSer};

impl IoDeSer for Duration{
    fn to_io_string(&self, tab: u8) -> String {
        let tab = (0..tab).map(|_| "\t").collect::<String>();
        format!(
            "|
{}\tseconds->|{}|
{}\tnanoseconds->|{}|
{}|", &tab,self.as_secs(),&tab, self.as_nanos(),&tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        let _ = delete_tabulator(io_input)?;

        let l = io_input.trim().split_terminator('\n').collect::<Vec<&str>>();

        if l.len()!=2{
            return Err(
                crate::Error::io_format(io_input.to_string(), format!("Wrong number of line encountered in the passed io string. Expected 2, received {}.",l.len())).into()
            );
        }

        let seconds = l[0].trim();
        let nano_seconds = l[1].trim();

        if !seconds.starts_with("seconds->|") || !nano_seconds.starts_with("nanoseconds->|"){
            return Err(
                crate::Error::io_format(io_input.to_string(), "Fields 'seconds' or 'nanoseconds' were not found during SystemTime type deserialization.".to_string()).into()
            );
        }

        let seconds = from_io!(seconds.split("->").nth(1).unwrap().to_string(),u64)?;
        let nano_seconds = from_io!(nano_seconds.split("->").nth(1).unwrap().to_string(),u128)?;

        Ok(Duration::new(seconds, nano_seconds as u32))
    }
}

impl IoDeSer for SystemTime{
    #[inline]
    fn to_io_string(&self, tab: u8) -> String {
        self.duration_since(UNIX_EPOCH).expect("TODO - handle this error better").to_io_string(tab)
    }

    #[inline]
    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        Ok(
            UNIX_EPOCH + from_io!(io_input, Duration)?
        )
    }
}

#[cfg(feature = "chrono")]
impl IoDeSer for chrono::DateTime<chrono::Utc>{
    fn to_io_string(&self, _tab: u8)->String{format!("|{}|", &self.to_rfc3339())}

    #[inline]
    fn from_io_string(io_input:&mut String)->crate::Result<Self> where Self: Sized{
        Ok(chrono::DateTime::parse_from_rfc3339(from_io!(io_input, &str)?).map_err(
            |e| crate::errors::Error::io_format(io_input.to_string(), e.to_string())
        )?.to_utc())
    }
}

#[cfg(feature = "chrono")]
impl IoDeSer for chrono::DateTime<chrono::FixedOffset>{
    fn to_io_string(&self, _tab: u8)->String{format!("|{}|", &self.to_rfc3339())}

    #[inline]
    fn from_io_string(io_input:&mut String)->crate::Result<Self> where Self: Sized{
        Ok(chrono::DateTime::parse_from_rfc3339(from_io!(io_input, &str)?).map_err(
            |e| crate::errors::Error::io_format(io_input.to_string(), e.to_string())
        )?)
    }
}

#[cfg(feature = "chrono")]
impl IoDeSer for chrono::DateTime<chrono::Local>{
    fn to_io_string(&self, _tab: u8)->String{format!("|{}|", &self.to_rfc3339())}

    #[inline]
    fn from_io_string(io_input:&mut String)->crate::Result<Self> where Self: Sized{
        Ok(chrono::DateTime::<chrono::Local>::from(from_io!(io_input, chrono::DateTime<chrono::FixedOffset>)?))
    }
}

#[cfg(feature = "chrono")]
impl IoDeSer for chrono::NaiveDate{
    fn to_io_string(&self, _tab: u8)->String{format!("|{}|", &self.format("%Y-%m-%dT00:00:00.00+00:00"))}

    #[inline]
    fn from_io_string(io_input:&mut String)->crate::Result<Self> where Self: Sized{
        Ok(chrono::NaiveDate::parse_from_str(from_io!(io_input, &str)?, "%Y-%m-%dT%H:%M:%S%.f+%Z").map_err(
            |e| crate::errors::Error::io_format(io_input.to_string(), e.to_string())
        )?)
    }
}

#[cfg(feature = "chrono")]
impl IoDeSer for chrono::NaiveDateTime{
    fn to_io_string(&self, _tab: u8)->String{format!("|{}|", &self.format("%Y-%m-%dT%H:%M:%S%.f+00:00"))}

    #[inline]
    fn from_io_string(io_input:&mut String)->crate::Result<Self> where Self: Sized{
        Ok(chrono::NaiveDateTime::parse_from_str(from_io!(io_input, &str)?, "%Y-%m-%dT%H:%M:%S%.f+%Z")
            .map_err(
                |e| crate::errors::Error::io_format(io_input.to_string(), e.to_string())
            )?)
    }
}

#[cfg(feature = "chrono")]
impl IoDeSer for chrono::TimeDelta{
    fn to_io_string(&self, tab: u8)->String{
       self.to_std().unwrap().to_io_string(tab)
    }

    #[inline]
    fn from_io_string(io_input:&mut String)->crate::Result<Self> where Self: Sized{
        let dur = from_io!(io_input, std::time::Duration)?; // TODO better implementation for negative time delta
        Ok(chrono::TimeDelta::from_std(dur).unwrap())
    }
}

#[cfg(feature = "chrono")]
impl IoDeSer for chrono::NaiveTime{
    fn to_io_string(&self, _tab: u8)->String{
        format!("|{}|", self.format("1970-01-01T%H:%M:%S%.f+00:00"))
    }

    #[inline]
    fn from_io_string(io_input:&mut String)->crate::Result<Self> where Self: Sized{
        Ok(chrono::NaiveTime::parse_from_str(
            from_io!(io_input, &str)?,
            "%Y-%m-%dT%H:%M:%S%.f+%Z"
        )
               .map_err(
                   |e| crate::errors::Error::io_format(io_input.to_string(), e.to_string())
               )?
        )
    }
}