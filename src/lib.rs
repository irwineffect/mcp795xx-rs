//#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]
#![allow(non_snake_case)]

extern crate embedded_hal as hal;
use hal::blocking::spi;
use hal::digital::v2::OutputPin;

mod registers;

pub struct DateTime {
    seconds: u8,
    minutes: u8,
    hours: u8,

    weekday: u8,
    date: u8,
    month: u8,
    year: u16,
}

//#[macro_use]
extern crate bitfield;


/// MCP795xx Driver
pub struct Mcp795xx<SPI, CS> {
    spi: SPI,
    cs: CS,
}


impl<SPI, CS, E1, E2> Mcp795xx<SPI, CS>
    where SPI: spi::Transfer<u8, Error = E1> + spi::Write<u8, Error = E1>,
          CS:  OutputPin<Error = E2>,
          E1: core::fmt::Debug,
          E2: core::fmt::Debug
{
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self {
            spi: spi,
            cs: cs
        }
    }

    pub fn get_time(&mut self) -> DateTime {
        let mut buff = [0 as u8; 10];

        // Instruction
        buff[0] = Instructions::WRITE as u8;

        // Address
        buff[1] = Addresses::RTCSEC as u8;

        self.cs.set_low().unwrap();
        self.spi.transfer(&mut buff).unwrap();
        self.cs.set_high().unwrap();

        DateTime {
            seconds : registers::RTCSEC(buff[2]).seconds(),
            minutes : registers::RTCMIN(buff[3]).minutes(),
            hours : registers::RTCHOUR(buff[4]).hours(),
            weekday : registers::RTCWKDAY(buff[5]).WKDAY(),
            date : registers::RTCDATE(buff[6]).date(),
            month : registers::RTCMTH(buff[7]).month(),
            year : registers::RTCYEAR(buff[8]).year()
        }
    }

    pub fn set_time(&mut self, datetime: DateTime) {

        let mut seconds = registers::RTCSEC(0);
        seconds.set_seconds(datetime.seconds);
        seconds.set_st(true);

        let mut minutes = registers::RTCMIN(0);
        minutes.set_minutes(datetime.minutes);

        let mut hours = registers::RTCHOUR(0);
        hours.set_hours_military(datetime.hours);

        let mut weekday = registers::RTCWKDAY(0);
        weekday.set_PWRFAIL(false);
        weekday.set_VBATEN(true);
        weekday.set_WKDAY(datetime.weekday);

        let mut date = registers::RTCDATE(0);
        date.set_date(datetime.date);

        let mut month = registers::RTCMTH(0);
        month.set_month(datetime.month);

        let mut year = registers::RTCYEAR(0);
        year.set_year(datetime.year);

        let buff = [
            // Instruction
            (Instructions::WRITE as u8),
            // Address
            Addresses::RTCHSEC as u8,
            0, // set hundreths of seconds to 0
            seconds.0,
            minutes.0,
            hours.0,
            weekday.0,
            date.0,
            month.0,
            year.0,
        ];

        self.cs.set_low().unwrap();
        self.spi.write(&buff).unwrap();
        self.cs.set_high().unwrap();
    }
}

#[allow(unused)]
enum Instructions {
    /// Read data from EEPROM array beginning at selected address
    EEREAD = 0b0000_0011,
    /// Write data to EEPROM array beginning at selected address
    EEWRITE = 0b0000_0010,
    /// Reset the write enable latch (disable write operations)
    EEWRDI = 0b0000_0100,
    /// Set the write enable latch (enable write operations)
    EEWREN = 0b0000_0110,
    /// Read STATUS register
    SRREAD = 0b0000_0101,
    /// Write STATUS register
    SRWRITE = 0b0000_0001,
    /// Read data from RTCC/SRAM array beginning at selected address
    READ = 0b0001_0011,
    /// Write data to RTCC/SRAM array beginning at selected address
    WRITE = 0b0001_0010,
    /// Unlock the protected EEPROM block for a write operation
    UNLOCK = 0b0001_0100,
    /// Write data to the protected EEPROM block beginning at selected address
    IDWRITE = 0b0011_0010,
    /// Read data from the protected EEPROM block beginning at the selected address
    IDREAD = 0b0011_0011,
    /// Clear all SRAM data to 0
    CLRRAM = 0b0101_0100,
}

#[allow(unused)]
enum Addresses {
    RTCHSEC = 0x00,
    RTCSEC= 0x01,
    RTCMIN = 0x02,
    RTCHOUR= 0x03,
    RTCWKDAY= 0x04,
    RTCDATE= 0x05,
    RTCMTH= 0x06,
    RTCYEAR= 0x07,
    CONTROL= 0x08,
    ALM0SEC= 0x0C,
    ALM0MIN= 0x0D,
    ALM0HOUR= 0x0E,
    ALM0WKDAY= 0x0F,
    ALM0DATE= 0x10,
    ALM0MTH= 0x11,
    ALM1HSEC= 0x12,
    ALM1SEC= 0x13,
    ALM1MIN= 0x14,
    ALM1HOUR= 0x15,
    ALM1WKDAY= 0x16,
    ALM1DATE= 0x17,
    PWRDNMIN= 0x18,
    PWRDNHOUR= 0x19,
    PWRDNDATE= 0x1A,
    PWRDNMTH= 0x1B,
    PWRUPMIN= 0x1C,
    PWRUPHOUR= 0x1D,
    PWRUPDATE= 0x1E,
    PWRUPMTH= 0x1F,
}


#[cfg(test)]
#[macro_use]
extern crate std;
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.

}
