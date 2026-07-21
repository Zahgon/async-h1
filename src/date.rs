use std::fmt::{self, Display, Formatter};
use std::str::{from_utf8, FromStr};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use http_types::{bail, ensure, format_err};

const IMF_FIXDATE_LENGTH: usize = 29;
const RFC850_MAX_LENGTH: usize = 23;
const ASCTIME_LENGTH: usize = 24;

const YEAR_9999_SECONDS: u64 = 253402300800;
const SECONDS_IN_DAY: u64 = 86400;
const SECONDS_IN_HOUR: u64 = 3600;

#[derive(Copy, Clone, Debug, Eq)]
pub struct HttpDate {
    
    second: u8,
    
    minute: u8,
    
    hour: u8,
    
    day: u8,
    
    month: u8,
    
    year: u16,
    
    week_day: u8,
}

#[allow(dead_code)]
pub(crate) fn parse_http_date(s: &str) -> http_types::Result<SystemTime> { panic!("STUB: not implemented") }

pub(crate) fn fmt_http_date(d: SystemTime) -> String { panic!("STUB: not implemented") }

impl HttpDate {
    fn is_valid(self) -> bool { panic!("STUB: not implemented") }
}

fn parse_imf_fixdate(s: &[u8]) -> http_types::Result<HttpDate> { panic!("STUB: not implemented") }

fn parse_rfc850_date(s: &[u8]) -> http_types::Result<HttpDate> { panic!("STUB: not implemented") }

fn parse_asctime(s: &[u8]) -> http_types::Result<HttpDate> { panic!("STUB: not implemented") }

impl From<SystemTime> for HttpDate {
    fn from(system_time: SystemTime) -> Self { panic!("STUB: not implemented") }
}

impl From<HttpDate> for SystemTime {
    fn from(http_date: HttpDate) -> Self { panic!("STUB: not implemented") }
}

impl FromStr for HttpDate {
    type Err = http_types::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> { panic!("STUB: not implemented") }
}

impl Display for HttpDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl PartialEq for HttpDate {
    fn eq(&self, other: &HttpDate) -> bool { panic!("STUB: not implemented") }
}

impl PartialOrd for HttpDate {
    fn partial_cmp(&self, other: &HttpDate) -> Option<std::cmp::Ordering> { panic!("STUB: not implemented") }
}

fn is_leap_year(year: u16) -> bool { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use std::time::{Duration, UNIX_EPOCH};

    use super::{fmt_http_date, parse_http_date, HttpDate, SECONDS_IN_DAY, SECONDS_IN_HOUR};

    #[test]
    fn test_rfc_example() {
        let d = UNIX_EPOCH + Duration::from_secs(784111777);
        assert_eq!(
            d,
            parse_http_date("Sun, 06 Nov 1994 08:49:37 GMT").expect("#1")
        );
        assert_eq!(
            d,
            parse_http_date("Sunday, 06-Nov-94 08:49:37 GMT").expect("#2")
        );
        assert_eq!(d, parse_http_date("Sun Nov  6 08:49:37 1994").expect("#3"));
    }

    #[test]
    fn test2() {
        let d = UNIX_EPOCH + Duration::from_secs(1475419451);
        assert_eq!(
            d,
            parse_http_date("Sun, 02 Oct 2016 14:44:11 GMT").expect("#1")
        );
        assert!(parse_http_date("Sun Nov 10 08:00:00 1000").is_err());
        assert!(parse_http_date("Sun Nov 10 08*00:00 2000").is_err());
        assert!(parse_http_date("Sunday, 06-Nov-94 08+49:37 GMT").is_err());
    }

    #[test]
    fn test3() {
        let mut d = UNIX_EPOCH;
        assert_eq!(d, parse_http_date("Thu, 01 Jan 1970 00:00:00 GMT").unwrap());
        d += Duration::from_secs(SECONDS_IN_HOUR);
        assert_eq!(d, parse_http_date("Thu, 01 Jan 1970 01:00:00 GMT").unwrap());
        d += Duration::from_secs(SECONDS_IN_DAY);
        assert_eq!(d, parse_http_date("Fri, 02 Jan 1970 01:00:00 GMT").unwrap());
        d += Duration::from_secs(2592000);
        assert_eq!(d, parse_http_date("Sun, 01 Feb 1970 01:00:00 GMT").unwrap());
        d += Duration::from_secs(2592000);
        assert_eq!(d, parse_http_date("Tue, 03 Mar 1970 01:00:00 GMT").unwrap());
        d += Duration::from_secs(31536005);
        assert_eq!(d, parse_http_date("Wed, 03 Mar 1971 01:00:05 GMT").unwrap());
        d += Duration::from_secs(15552000);
        assert_eq!(d, parse_http_date("Mon, 30 Aug 1971 01:00:05 GMT").unwrap());
        d += Duration::from_secs(6048000);
        assert_eq!(d, parse_http_date("Mon, 08 Nov 1971 01:00:05 GMT").unwrap());
        d += Duration::from_secs(864000000);
        assert_eq!(d, parse_http_date("Fri, 26 Mar 1999 01:00:05 GMT").unwrap());
    }

    #[test]
    fn test_fmt() {
        let d = UNIX_EPOCH;
        assert_eq!(fmt_http_date(d), "Thu, 01 Jan 1970 00:00:00 GMT");
        let d = UNIX_EPOCH + Duration::from_secs(1475419451);
        assert_eq!(fmt_http_date(d), "Sun, 02 Oct 2016 14:44:11 GMT");
    }

    #[test]
    fn size_of() {
        assert_eq!(::std::mem::size_of::<HttpDate>(), 8);
    }
}
