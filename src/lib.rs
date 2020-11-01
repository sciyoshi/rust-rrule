//! A partial implementation of recurrence rules as defined in the iCalendar RFC.
//! 
//!
//!
//! # Examples
//!
//! RRule quickstart from rrulestring
//!
//! ```
//! extern crate rrule;
//!
//! use rrule::build_rrule;
//!
//! // Parse a RRule string, return a RRule type
//! let mut rrule = build_rrule("DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR").unwrap();
//! assert_eq!(rrule.all().len(), 21);
//! ```
//!
//!
//! RRuleSet quickstart from rrulestring
//!
//! ```
//! extern crate rrule;
//!
//! use rrule::build_rruleset;
//!
//! // Parse a RRuleSet string, return a RRuleSet type
//! let mut rrule = build_rruleset("DTSTART:20120201T023000Z\nRRULE:FREQ=MONTHLY;COUNT=5\nRDATE:20120701T023000Z,20120702T023000Z\nEXRULE:FREQ=MONTHLY;COUNT=2\nEXDATE:20120601T023000Z").unwrap();
//! assert_eq!(rrule.all().len(), 4);
//! ```
//!
//!
//!
//! Using `Options` instead of rrule strings to build RRule and RRuleSet
//!
//! ```
//! extern crate rrule;
//! extern crate chrono;
//! extern crate chrono_tz;
//!
//! use chrono::prelude::*;
//! use chrono_tz::UTC;
//! use rrule::{RRule, RRuleSet, Options, Frequenzy, Weekday};
//!
//! // Build options that starts first day in 2020 at 9:00AM and occurs daily 5 times
//! let mut options = Options::new()
//!     .dtstart(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
//!     .count(5)
//!     .freq(Frequenzy::Daily)
//!     .build()
//!     .unwrap();
//!
//! // Construct `RRule` from options
//! let mut rrule = RRule::new(options);
//! let recurrences = rrule.all();
//! for i in 0..5 {
//!     assert_eq!(recurrences[i].year(), 2020);
//!     assert_eq!(recurrences[i].month(), 1);
//!     assert_eq!(recurrences[i].day(), 1 + i as u32);
//!     assert_eq!(recurrences[i].hour(), 9);
//! }
//! assert_eq!(recurrences.len(), 5);
//!
//!
//!
//!
//!
//! // Construct RRuleSet from one rrule and exrule
//! // The rrule will occur weekly on Tuesday and Wednesday and the exrule
//! // will occur weekly on Wednesday, and therefore the end result will contain
//! // weekly recurrences just on Wednesday.
//!
//!
//! // Build options for rrule that occurs weekly on Tuesday and Wednesday
//! let mut rrule_options = Options::new()
//!     .dtstart(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
//!     .count(4)
//!     .freq(Frequenzy::Weekly)
//!     .byweekday(vec![Weekday::Tue, Weekday::Wed])
//!     .build()
//!     .unwrap();
//!
//! // Construct `RRule` from options
//! let mut rrule = RRule::new(rrule_options);
//!
//!
//! // Build options for exrule that occurs weekly on Wednesday
//! let mut exrule_options = Options::new()
//!     .dtstart(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
//!     .count(4)
//!     .freq(Frequenzy::Weekly)
//!     .byweekday(vec![Weekday::Wed])
//!     .build()
//!     .unwrap();
//!
//! // Construct `RRule` from options
//! let mut exrule = RRule::new(exrule_options);
//!
//! // Now create the RRuleSet and add rrule and exrule
//! let mut rrule_set = RRuleSet::new();
//! rrule_set.rrule(rrule);
//! rrule_set.exrule(exrule);
//!
//! let recurrences = rrule_set.all();
//!
//! for occurence in &recurrences {
//!     assert_eq!(occurence.weekday(), Weekday::Tue);
//! }
//!
//! assert_eq!(recurrences.len(), 2);
//! ```
//!
//!
//!
//!
//! Timezone support
//!
//! ```
//! extern crate rrule;
//! extern crate chrono;
//! extern crate chrono_tz;
//!
//! use chrono::prelude::*;
//! use chrono_tz::{UTC, Tz};
//! use chrono_tz::Europe::Berlin;
//! use rrule::{RRule, RRuleSet, Options, Frequenzy, Weekday};
//!
//! // SOME NOTES:
//! // recurrences produced by an rrule will be in the same timezone
//! // as the start datetime provided (dtstart). The `until` datetime MUST
//! // always be specified with the UTC timezone if it is specified.
//!
//! // Example:
//! // The following examples uses the RRuleSet type with an RRule that yields recurrences
//! // in the Europe/Berlin timezone, and one EXDATE that is specified
//! // in UTC and collides (and therefore filters away) with one of those recurrences.  
//!
//!
//! // Build options for rrule that occurs daily at 9 oclock for 4 times
//! let mut rrule_options = Options::new()
//!     .dtstart(Berlin.ymd(2020, 1, 1).and_hms(9, 0, 0))
//!     .count(4)
//!     .freq(Frequenzy::Daily)
//!     .build()
//!     .unwrap();
//!
//! let mut rrule = RRule::new(rrule_options);
//!
//! // Exdate in the UTC at 8 oclock which is 9 oclock in Berlin and therefore
//! // collides with the second rrule occurence.
//! let exdate = UTC.ymd(2020, 1, 2).and_hms(8, 0, 0);
//!
//! // Now create the RRuleSet and add rrule and exdate
//! let mut rrule_set = RRuleSet::new();
//! rrule_set.rrule(rrule);
//! rrule_set.exdate(exdate);
//!
//! let recurrences = rrule_set.all();
//! // RRule contained 4 recurrences but 1 was filtered away by the exdate
//! assert_eq!(recurrences.len(), 3);
//!
//! // If you want to get back the DateTimes in another timezone you can just iterate over the result
//! // and convert them to another timezone by using the with_timzone method provided by the DateTime type.
//! // Refer to the chrono and chrono-tz crates for more documenation on working with the DateTime type.
//! 
//! // Example of converting to mocow timezone
//! use chrono_tz::Europe::Moscow;
//!
//! let recurrences_in_moscow_tz: Vec<DateTime<Tz>> = recurrences.iter()
//!     .map(|d| d.with_timezone(&Moscow)).collect();
//!
//!
//! // Example of converting to local timezone (Local comes from chrono::prelude::*)
//! let recurrences_in_local_tz: Vec<DateTime<Local>> = recurrences.iter()
//!     .map(|d| d.with_timezone(&Local)).collect();
//!
//!
//! ```

extern crate chrono;
extern crate chrono_tz;
extern crate once_cell;
extern crate regex;

mod datetime;
mod iter;
mod options;
mod parse_options;
mod rrule;
mod rrule_iter;
mod rruleset;
mod rruleset_iter;
mod rrulestr;

pub use crate::options::{Frequenzy, Options, ParsedOptions};
pub use crate::rrule::RRule;
pub use crate::rruleset::RRuleSet;
pub use crate::rrulestr::{build_rrule, build_rruleset};
pub use chrono::Weekday;
