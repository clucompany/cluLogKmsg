
#[macro_use]
extern crate cluLog;
extern crate cluLogKmsg;

use cluLogKmsg::default_open_kmsg;
use cluLog::log_addition::union::LogUnionConst;

pub fn main() {
     //Opening /dev/kmsg
     //In case of an error, the log is output to the standard output.
     match default_open_kmsg() {
          Ok(kmsg) => {
               //Combining a standard output system with an output system in a file /dev/kmsg
               cluLog::set_logger(
                    cluLog::log::default::LogDefault::default().default_union(kmsg)
               );
          },
          _ => {
               //Install Standard Output System
               cluLog::set_logger(
                    cluLog::log::default::LogDefault::default()
               );
          },
     }
     
     println!("123");
     eprintln!("e123\n10");
     trace!("Test");
     err!("Warning");
     inf!("My info");
}