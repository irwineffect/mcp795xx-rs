extern crate bitfield;
use bitfield::{bitfield};

use core::convert::TryInto;

// base year, change to 2100 when needed
const base_year: u16 = 2000;

bitfield! {
  pub struct RTCHSEC(u8);
  impl Debug;

  ///bit 7-4 HSECTEN<3:0>: Binary-Coded Decimal Value of Hundredth of Second’s Tens Digit
  ///Contains a value from 0 to 9
  HSECTEN, set_HSECTEN : 7, 4;
  ///bit 3-0 HSECONE<3:0>: Binary-Coded Decimal Value of Hundredth of Second’s Ones Digit
  ///Contaihundretns a value from 0 to 9
  HSECONE, set_HSECONE : 3, 0;
}

impl RTCHSEC {
    pub fn set_hundreth_seconds(&mut self, hundreth_seconds: u8) {
        if hundreth_seconds > 99 {
            panic!("Invalid input: {}", hundreth_seconds)
        }
        self.set_HSECTEN(hundreth_seconds/10);
        self.set_HSECONE(hundreth_seconds % 10);
    }

    pub fn hundreth_seconds(&self) -> u8 {
        self.HSECTEN() * 10 + self.HSECONE()
    }
}


bitfield! {
  pub struct RTCSEC(u8);
  impl Debug;

  /// ST: Start Oscillator bit
  ///   1 = Oscillator enabled
  ///   0 = Oscillator disabled
  pub ST, set_st: 7;
  /// bit 6-4 SECTEN<2:0>: Binary-Coded Decimal Value of Second’s Tens Digit
  ///   Contains a value from 0 to 5
  SECTEN, set_SECTEN : 6, 4;
  /// bit 3-0 SECONE<3:0>: Binary-Coded Decimal Value of Second’s Ones Digit
  ///   Contains a value from 0 to 9
  SECONE, set_SECONE : 3, 0;
}

impl RTCSEC {
    pub fn set_seconds(&mut self, seconds: u8) {
        if seconds > 59 {
            panic!("Invalid input: {}", seconds)
        }
        self.set_SECTEN(seconds/10);
        self.set_SECONE(seconds % 10);
    }

    pub fn seconds(&self) -> u8 {
        self.SECTEN() * 10 + self.SECONE()
    }
}

bitfield! {
  pub struct RTCMIN(u8);
  impl Debug;

  MINTEN, set_MINTEN : 6, 4;
  MINONE, set_MINONE : 3, 0;
}

impl RTCMIN {
    pub fn set_minutes(&mut self, value: u8) {
        if value > 59 {
            panic!("Invalid input: {}", value)
        }
        self.set_MINTEN(value/10);
        self.set_MINONE(value % 10);
    }

    pub fn minutes(&self) -> u8 {
        self.MINTEN() * 10 + self.MINONE()
    }
}

bitfield! {
  pub struct RTCHOUR(u8);
  impl Debug;

  /// TRIMSIGN: Trim Sign bit
  /// 1 = Add clocks to correct for slow time
  /// 0 = Subtract clocks to correct for fast time
  pub TRIMSIGN, set_TRIMSIGN : 7;


  /// 12/24 hour format
  /// 1 = 12 hour format (STD)
  /// 0 = 24 hour format (MILTARY)
  FORMAT, set_FORMAT: 6;

  /// PM indicator, used when format is STD
  PM, set_PM : 5;

  /// HOUR tens place, used when format is STD
  HOURTEN_STD, set_HOURTEN_STD : 4, 4;

  /// HOUR tens place, used when format is MILITARY
  HOURTEN_MILITARY, set_HOURTEN_MILITARY : 5, 4;

  HOURONE, set_HOURONE : 3, 0;
}

impl RTCHOUR {
    pub fn set_hours_military(&mut self, value: u8) {
        if value > 23 {
            panic!("Invalid input: {}", value);
        }
        self.set_HOURTEN_MILITARY(value/10);
        self.set_HOURONE(value % 10);
        self.set_FORMAT(false);
    }

    pub fn set_hours_std(&mut self, value: u8, pm: bool) {
        if value > 12 {
            panic!("Invalid input: {}", value);
        }

        self.set_HOURTEN_STD(value/10);
        self.set_HOURONE(value % 10);
        self.set_PM(pm);
        self.set_FORMAT(true);
    }

    pub fn hours(&self) -> u8 {
        match self.FORMAT() {
        false => self.HOURTEN_MILITARY() * 10 + self.HOURONE(),
        true => self.HOURTEN_STD() * 10 + self.HOURONE()
        }
    }
}

bitfield! {
  pub struct RTCWKDAY(u8);
  impl Debug;

  pub OSCRUN, set_OSCRUN : 5;
  pub PWRFAIL, set_PWRFAIL : 4;
  pub VBATEN, set_VBATEN : 3;

  pub WKDAY, set_WKDAY : 2, 0;
}

bitfield! {
  pub struct RTCDATE(u8);
  impl Debug;

  DATETEN, set_DATETEN : 5, 4;
  DATEONE, set_DATEONE : 3, 0;
}

impl RTCDATE {
    pub fn set_date(&mut self, value: u8) {
        if value > 31 {
            panic!("Invalid input: {}", value)
        }
        self.set_DATETEN(value/10);
        self.set_DATEONE(value % 10);
    }

    pub fn date(&self) -> u8 {
        self.DATETEN() * 10 + self.DATEONE()
    }
}

bitfield! {
  pub struct RTCMTH(u8);
  impl Debug;

  LPYR, _ : 5;
  MTHTEN, set_MTHTEN : 4, 4;
  MTHONE, set_MTHONE : 3, 0;
}

impl RTCMTH {
    pub fn set_month(&mut self, value: u8) {
        if value > 12 {
            panic!("Invalid input: {}", value)
        }
        self.set_MTHTEN(value/10);
        self.set_MTHONE(value % 10);
    }

    pub fn month(&self) -> u8 {
        self.MTHTEN() * 10 + self.MTHONE()
    }
}

bitfield! {
  pub struct RTCYEAR(u8);
  impl Debug;

  YEARTEN, set_YEARTEN : 7, 4;
  YEARONE, set_YEARONE : 3, 0;
}

impl RTCYEAR {
    pub fn set_year(&mut self, value: u16) {
        if value > base_year+99 || value < base_year {
            panic!("Invalid input: {}", value)
        }

        let value: u8 = (value - base_year).try_into().unwrap();
        self.set_YEARTEN(value/10);
        self.set_YEARONE(value % 10);
    }

    pub fn year(&self) -> u16 {
        let year: u16 = (self.YEARTEN() * 10 + self.YEARONE()).into();

        year + base_year
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn RTCSEC_test() {
        let mut a = RTCSEC(1<<7 | 4 << 4 | 5);
        assert_eq!(a.ST(), true);
        assert_eq!(a.seconds(), 45);
    }
}
