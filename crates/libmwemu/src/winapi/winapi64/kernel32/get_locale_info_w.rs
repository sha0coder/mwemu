
use crate::{constants, emu};
use crate::winapi::winapi64::kernel32::{clear_last_error, LAST_ERROR};

const LOCALE_SABBREVMONTHNAME1: u64 = 68;
const LOCALE_SABBREVMONTHNAME2: u64 = 69;
const LOCALE_SABBREVMONTHNAME3: u64 = 70;
const LOCALE_SABBREVMONTHNAME4: u64 = 71;
const LOCALE_SABBREVMONTHNAME5: u64 = 72;
const LOCALE_SABBREVMONTHNAME6: u64 = 73;
const LOCALE_SABBREVMONTHNAME7: u64 = 74;
const LOCALE_SABBREVMONTHNAME8: u64 = 75;
const LOCALE_SABBREVMONTHNAME9: u64 = 76;
const LOCALE_SABBREVMONTHNAME10: u64 = 77;
const LOCALE_SABBREVMONTHNAME11: u64 = 78;
const LOCALE_SABBREVMONTHNAME12: u64 = 79;
const LOCALE_SLANGUAGE: u64 = 0x00000002;
const LOCALE_SCOUNTRY: u64 = 0x00000006;
const LOCALE_SLIST: u64 = 0x0000000C;
const LOCALE_SDECIMAL: u64 = 0x0000000E;
const LOCALE_STHOUSAND: u64 = 0x0000000F;
const LOCALE_SCURRENCY: u64 = 0x00000014;
const LOCALE_SDATE: u64 = 0x0000001D;
const LOCALE_STIME: u64 = 0x0000001E;
const LOCALE_RETURN_NUMBER: u64 = 0x20000000;

// Additional constants with their values and typical return values:
const LOCALE_SSHORTDATE: u64 = 0x0000001F;        // Short date format string (e.g., "M/d/yyyy")
const LOCALE_SLONGDATE: u64 = 0x00000020;         // Long date format string (e.g., "dddd, MMMM d, yyyy")
const LOCALE_STIMEFORMAT: u64 = 0x00001003;       // Time format string (e.g., "h:mm:ss tt")
const LOCALE_IDATE: u64 = 0x00000021;             // Short date format ordering (0=MDY, 1=DMY, 2=YMD)
const LOCALE_ILDATE: u64 = 0x00000022;            // Long date format ordering
const LOCALE_ITIME: u64 = 0x00000023;             // Time format specifier (0=12hr, 1=24hr)
const LOCALE_ICENTURY: u64 = 0x00000024;          // Century format specifier (0=2-digit, 1=4-digit)
const LOCALE_ITLZERO: u64 = 0x00000025;           // Leading zeros in time field (0=no, 1=yes)
const LOCALE_IDAYLZERO: u64 = 0x00000026;         // Leading zeros in day field (0=no, 1=yes)
const LOCALE_IMONLZERO: u64 = 0x00000027;         // Leading zeros in month field (0=no, 1=yes)
const LOCALE_S1159: u64 = 0x00000028;             // AM symbol (e.g., "AM")
const LOCALE_S2359: u64 = 0x00000029;             // PM symbol (e.g., "PM")
const LOCALE_ICALENDARTYPE: u64 = 0x0000002A;     // Calendar type (1=Gregorian)
const LOCALE_IOPTIONALCALENDAR: u64 = 0x0000002B; // Optional calendar type
const LOCALE_IFIRSTDAYOFWEEK: u64 = 0x0000002C;   // First day of week (0=Monday, 6=Sunday)
const LOCALE_IFIRSTWEEKOFYEAR: u64 = 0x0000002D;  // First week of year
const LOCALE_ICOUNTRY: u64 = 0x0000002E;          // Country code
const LOCALE_ICURRDIGITS: u64 = 0x00000019;       // Currency decimal digits
const LOCALE_IINTLCURRDIGITS: u64 = 0x0000001A;   // International currency decimal digits
const LOCALE_ICURRENCY: u64 = 0x0000001B;         // Currency format (0-3)
const LOCALE_INEGCURR: u64 = 0x0000001C;          // Negative currency format (0-15)

// Additional constants that might be encountered:
const LOCALE_ILANGUAGE: u64 = 0x00000001;         // Language identifier
const LOCALE_IDEFAULTLANGUAGE: u64 = 0x00000009;  // Default language
const LOCALE_IDEFAULTCOUNTRY: u64 = 0x0000000A;   // Default country
const LOCALE_IDEFAULTCODEPAGE: u64 = 0x0000000B;  // Default codepage
const LOCALE_IDEFAULTANSICODEPAGE: u64 = 0x00001004; // Default ANSI codepage
const LOCALE_IDEFAULTMACCODEPAGE: u64 = 0x00001011;  // Default MAC codepage
const LOCALE_SENGCURRNAME: u64 = 0x00001007;      // English currency name
const LOCALE_SNATIVECURRNAME: u64 = 0x00001008;   // Native currency name
const LOCALE_SYEARMONTH: u64 = 0x00001006;        // Year month format
const LOCALE_SSORTNAME: u64 = 0x00001013;         // Sort name
const LOCALE_IDIGITSUBSTITUTION: u64 = 0x00001014; // Digit substitution

// Constants that may return numeric values:
const LOCALE_IDIGITS: u64 = 0x00000011;           // Number of decimal digits (e.g., "2")
const LOCALE_ILZERO: u64 = 0x00000012;            // Leading zeros for decimal (0=no, 1=yes)
const LOCALE_IMEASURE: u64 = 0x0000000D;          // Measurement system (0=metric, 1=US)
const LOCALE_INEGNUMBER: u64 = 0x00001010;        // Negative number format
const LOCALE_SGROUPING: u64 = 0x00000010;         // Grouping for numbers (e.g., "3;0")
const LOCALE_SINTLSYMBOL: u64 = 0x00000015;       // International currency symbol
const LOCALE_SMONDECIMALSEP: u64 = 0x00000016;    // Monetary decimal separator
const LOCALE_SMONTHOUSANDSEP: u64 = 0x00000017;   // Monetary thousand separator
const LOCALE_SMONGROUPING: u64 = 0x00000018;      // Monetary grouping
const LOCALE_SPOSITIVESIGN: u64 = 0x00000050;     // Positive sign
const LOCALE_SNEGATIVESIGN: u64 = 0x00000051;     // Negative sign

// Day and month names:
const LOCALE_SDAYNAME1: u64 = 0x0000002A;         // Monday
const LOCALE_SDAYNAME2: u64 = 0x0000002B;         // Tuesday
const LOCALE_SDAYNAME3: u64 = 0x0000002C;         // Wednesday
const LOCALE_SDAYNAME4: u64 = 0x0000002D;         // Thursday
const LOCALE_SDAYNAME5: u64 = 0x0000002E;         // Friday
const LOCALE_SDAYNAME6: u64 = 0x0000002F;         // Saturday
const LOCALE_SDAYNAME7: u64 = 0x00000030;         // Sunday

const LOCALE_SMONTHNAME1: u64 = 0x00000038;       // January
const LOCALE_SMONTHNAME2: u64 = 0x00000039;       // February
const LOCALE_SMONTHNAME3: u64 = 0x0000003A;       // March
const LOCALE_SMONTHNAME4: u64 = 0x0000003B;       // April
const LOCALE_SMONTHNAME5: u64 = 0x0000003C;       // May
const LOCALE_SMONTHNAME6: u64 = 0x0000003D;       // June
const LOCALE_SMONTHNAME7: u64 = 0x0000003E;       // July
const LOCALE_SMONTHNAME8: u64 = 0x0000003F;       // August
const LOCALE_SMONTHNAME9: u64 = 0x00000040;       // September
const LOCALE_SMONTHNAME10: u64 = 0x00000041;      // October
const LOCALE_SMONTHNAME11: u64 = 0x00000042;      // November
const LOCALE_SMONTHNAME12: u64 = 0x00000043;      // December

const LOCALE_SABBREVDAYNAME1: u64 = 0x0000004E;   // Monday abbreviated
const LOCALE_SABBREVDAYNAME2: u64 = 0x0000004F;   // Tuesday abbreviated
const LOCALE_SABBREVDAYNAME3: u64 = 0x00000050;   // Wednesday abbreviated
const LOCALE_SABBREVDAYNAME4: u64 = 0x00000051;   // Thursday abbreviated
const LOCALE_SABBREVDAYNAME5: u64 = 0x00000052;   // Friday abbreviated
const LOCALE_SABBREVDAYNAME6: u64 = 0x00000053;   // Saturday abbreviated
const LOCALE_SABBREVDAYNAME7: u64 = 0x00000054;   // Sunday abbreviated

pub fn GetLocaleInfoW(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx as u64;
    let lctype = emu.regs().rdx as u64;
    let lp_lc_data = emu.regs().r8 as usize;
    let cch_data = emu.regs().r9 as usize;

    log_red!(emu, "** {} kernel32!GetLocaleInfoW locale: 0x{:x} lctype: 0x{:x} lp_lc_data: 0x{:x} cch_data: {}",
        emu.pos,
        locale,
        lctype,
        lp_lc_data,
        cch_data
    );

    let result = match lctype {
        LOCALE_SLANGUAGE => "English",
        LOCALE_SCOUNTRY => "United States",
        LOCALE_SLIST => ",",
        LOCALE_SDECIMAL => ".",
        LOCALE_STHOUSAND => ",",
        LOCALE_SCURRENCY => "$",
        LOCALE_SDATE => "/",
        LOCALE_STIME => ":",
        LOCALE_ICURRDIGITS => "2",           // LOCALE_ICURRDIGITS - Number of currency decimal digits
        LOCALE_IINTLCURRDIGITS => "2",           // LOCALE_IINTLCURRDIGITS - International currency decimal digits
        LOCALE_ICURRENCY => "0",           // LOCALE_ICURRENCY - Currency format (0-3)
        LOCALE_INEGCURR => "0",           // LOCALE_INEGCURR - Negative currency format (0-15)
        LOCALE_SSHORTDATE => "M/d/yyyy",    // LOCALE_SSHORTDATE - Short date format
        LOCALE_SLONGDATE => "dddd, MMMM d, yyyy", // LOCALE_SLONGDATE - Long date format
        LOCALE_IDATE => "0",           // LOCALE_IDATE - Date format ordering (0=MDY, 1=DMY, 2=YMD)
        LOCALE_ILDATE => "0",           // LOCALE_ILDATE - Long date format ordering
        LOCALE_ITIME => "0",           // LOCALE_ITIME - Time format (0=12hr, 1=24hr)
        LOCALE_ICENTURY => "1",           // LOCALE_ICENTURY - Century format (0=2-digit, 1=4-digit)
        LOCALE_ITLZERO => "0",           // LOCALE_ITLZERO - Leading zeros in time
        LOCALE_IDAYLZERO => "0",           // LOCALE_IDAYLZERO - Leading zeros in day
        LOCALE_IMONLZERO => "0",           // LOCALE_IMONLZERO - Leading zeros in month
        LOCALE_S1159 => "AM",          // LOCALE_S1159 - AM symbol
        LOCALE_S2359 => "PM",          // LOCALE_S2359 - PM symbol
        LOCALE_ICALENDARTYPE => "1",           // LOCALE_ICALENDARTYPE - Calendar type (1=Gregorian)
        LOCALE_IOPTIONALCALENDAR => "1",           // LOCALE_IOPTIONALCALENDAR - Optional calendar
        LOCALE_IFIRSTDAYOFWEEK => "6",           // LOCALE_IFIRSTDAYOFWEEK - First day of week (0=Mon, 6=Sun)
        LOCALE_IFIRSTWEEKOFYEAR => "0",           // LOCALE_IFIRSTWEEKOFYEAR - First week of year
        LOCALE_ICOUNTRY => "1",           // LOCALE_ICOUNTRY - Country code
        LOCALE_SDAYNAME6 => "Saturday",    // LOCALE_SDAYNAME6 - Saturday
        LOCALE_SDAYNAME7 => "Sunday",      // LOCALE_SDAYNAME7 - Sunday
        LOCALE_SDAYNAME1 => "Monday",      // LOCALE_SDAYNAME1 - Monday  
        LOCALE_SDAYNAME2 => "Tuesday",     // LOCALE_SDAYNAME2 - Tuesday
        LOCALE_SDAYNAME3 => "Wednesday",   // LOCALE_SDAYNAME3 - Wednesday
        LOCALE_SDAYNAME4 => "Thursday",    // LOCALE_SDAYNAME4 - Thursday
        LOCALE_SDAYNAME5 => "Friday",      // LOCALE_SDAYNAME5 - Friday
        LOCALE_SMONTHNAME1 => "January",     // LOCALE_SMONTHNAME1 - January
        LOCALE_SMONTHNAME2 => "February",    // LOCALE_SMONTHNAME2 - February
        LOCALE_SMONTHNAME3 => "March",       // LOCALE_SMONTHNAME3 - March
        LOCALE_SMONTHNAME4 => "April",       // LOCALE_SMONTHNAME4 - April
        LOCALE_SMONTHNAME5 => "May",         // LOCALE_SMONTHNAME5 - May
        LOCALE_SMONTHNAME6 => "June",        // LOCALE_SMONTHNAME6 - June
        LOCALE_SMONTHNAME7 => "July",        // LOCALE_SMONTHNAME7 - July
        LOCALE_SMONTHNAME8 => "August",      // LOCALE_SMONTHNAME8 - August
        LOCALE_SMONTHNAME9 => "September",   // LOCALE_SMONTHNAME9 - September
        LOCALE_SMONTHNAME10 => "October",     // LOCALE_SMONTHNAME10 - October
        LOCALE_SMONTHNAME11 => "November",    // LOCALE_SMONTHNAME11 - November
        LOCALE_SMONTHNAME12 => "December",    // LOCALE_SMONTHNAME12 - December
        LOCALE_SABBREVMONTHNAME1 => "Jan",         // LOCALE_SABBREVMONTHNAME1 - January abbreviated
        LOCALE_SABBREVMONTHNAME2 => "Feb",         // LOCALE_SABBREVMONTHNAME2 - February abbreviated
        LOCALE_SABBREVMONTHNAME3 => "Mar",         // LOCALE_SABBREVMONTHNAME3 - March abbreviated
        LOCALE_SABBREVMONTHNAME4 => "Apr",         // LOCALE_SABBREVMONTHNAME4 - April abbreviated
        LOCALE_SABBREVMONTHNAME5 => "May",         // LOCALE_SABBREVMONTHNAME5 - May abbreviated
        LOCALE_SABBREVMONTHNAME6 => "Jun",         // LOCALE_SABBREVMONTHNAME6 - June abbreviated
        LOCALE_SABBREVMONTHNAME7 => "Jul",         // LOCALE_SABBREVMONTHNAME7 - July abbreviated
        LOCALE_SABBREVMONTHNAME8 => "Aug",         // LOCALE_SABBREVMONTHNAME8 - August abbreviated
        LOCALE_SABBREVMONTHNAME9 => "Sep",         // LOCALE_SABBREVMONTHNAME9 - September abbreviated
        LOCALE_SABBREVMONTHNAME10 => "Oct",         // LOCALE_SABBREVMONTHNAME10 - October abbreviated
        LOCALE_SABBREVMONTHNAME11 => "Nov",         // LOCALE_SABBREVMONTHNAME11 - November abbreviated
        LOCALE_SABBREVMONTHNAME12 => "Dec",         // LOCALE_SABBREVMONTHNAME12 - December abbreviated
        LOCALE_SABBREVDAYNAME1 => "Mon",         // LOCALE_SABBREVDAYNAME1 - Monday abbreviated
        LOCALE_SABBREVDAYNAME2 => "Tue",         // LOCALE_SABBREVDAYNAME2 - Tuesday abbreviated
        LOCALE_SABBREVDAYNAME3 => "Wed",         // LOCALE_SABBREVDAYNAME3 - Wednesday abbreviated
        LOCALE_SABBREVDAYNAME4 => "Thu",         // LOCALE_SABBREVDAYNAME4 - Thursday abbreviated
        LOCALE_SABBREVDAYNAME5 => "Fri",         // LOCALE_SABBREVDAYNAME5 - Friday abbreviated
        LOCALE_SABBREVDAYNAME6 => "Sat",         // LOCALE_SABBREVDAYNAME6 - Saturday abbreviated
        LOCALE_SABBREVDAYNAME7 => "Sun",         // LOCALE_SABBREVDAYNAME7 - Sunday abbreviated
        
        // Additional commonly used constants:
        LOCALE_ILANGUAGE => "0409",        // LOCALE_ILANGUAGE - Language ID (US English)
        LOCALE_IDEFAULTLANGUAGE => "0409",        // LOCALE_IDEFAULTLANGUAGE - Default language
        LOCALE_IDEFAULTCOUNTRY => "1",           // LOCALE_IDEFAULTCOUNTRY - Default country
        LOCALE_IDEFAULTCODEPAGE => "1252",        // LOCALE_IDEFAULTCODEPAGE - Default codepage
        LOCALE_IMEASURE => "1",           // LOCALE_IMEASURE - Measurement system (0=metric, 1=US)
        LOCALE_SGROUPING => "3;0",         // LOCALE_SGROUPING - Number grouping
        LOCALE_IDIGITS => "2",           // LOCALE_IDIGITS - Number of decimal digits
        LOCALE_ILZERO => "1",           // LOCALE_ILZERO - Leading zeros for decimal
        LOCALE_SINTLSYMBOL => "USD",         // LOCALE_SINTLSYMBOL - International currency symbol
        LOCALE_SMONDECIMALSEP => ".",           // LOCALE_SMONDECIMALSEP - Monetary decimal separator
        LOCALE_SMONTHOUSANDSEP => ",",           // LOCALE_SMONTHOUSANDSEP - Monetary thousand separator
        LOCALE_SMONGROUPING => "3;0",         // LOCALE_SMONGROUPING - Monetary grouping
        LOCALE_SPOSITIVESIGN => "",            // LOCALE_SPOSITIVESIGN - Positive sign (usually empty)
        LOCALE_SNEGATIVESIGN => "-",           // LOCALE_SNEGATIVESIGN - Negative sign
        LOCALE_STIMEFORMAT => "h:mm:ss tt", // LOCALE_STIMEFORMAT - Time format string
        LOCALE_IDEFAULTANSICODEPAGE => "1252",      // LOCALE_IDEFAULTANSICODEPAGE - Default ANSI codepage
        LOCALE_SYEARMONTH => "MMMM yyyy", // LOCALE_SYEARMONTH - Year month format
        LOCALE_SENGCURRNAME => "US Dollar", // LOCALE_SENGCURRNAME - English currency name
        LOCALE_SNATIVECURRNAME => "US Dollar", // LOCALE_SNATIVECURRNAME - Native currency name
        LOCALE_INEGNUMBER => "1",         // LOCALE_INEGNUMBER - Negative number format
        LOCALE_IDEFAULTMACCODEPAGE => "10000",     // LOCALE_IDEFAULTMACCODEPAGE - Default MAC codepage
        LOCALE_SSORTNAME => "Default",   // LOCALE_SSORTNAME - Sort name
        LOCALE_IDIGITSUBSTITUTION => "1",         // LOCALE_IDIGITSUBSTITUTION - Digit substitution
        
        _ => {
            log::warn!("{} GetLocaleInfoW unhandled lctype: 0x{:x}", emu.pos, lctype);
            "." // Default fallback
        }
    };

    let required_size = result.len() + 1; // Include null terminator

    // Check if it wants buffer size
    if cch_data == 0 {
        emu.regs_mut().rax = required_size as u64;
        clear_last_error(emu);
        return;
    }

    // Validate buffer pointer
    if lp_lc_data == 0 {
        log::warn!("{} GetLocaleInfoW invalid parameter - null buffer", emu.pos);
        let mut err = LAST_ERROR.lock().unwrap();
        *err = constants::ERROR_INVALID_PARAMETER;
        emu.regs_mut().rax = 0;
        return;
    }

    // Check if buffer is too small
    if cch_data < required_size {
        log::warn!("{} buffer too small for result cch_data: {} required_size: {}", emu.pos, cch_data, required_size);
        let mut err = LAST_ERROR.lock().unwrap();
        *err = constants::ERROR_INSUFFICIENT_BUFFER;
        emu.regs_mut().rax = 0;
        return;
    }

    // Write the result to the buffer
    emu.maps.write_wide_string(lp_lc_data as u64, result);
    emu.regs_mut().rax = required_size as u64;
    clear_last_error(emu);
}