
#[macro_use]
extern crate cluLog;
extern crate cluLogKmsg;

use cluLogKmsg::default_open_kmsg;
use cluLog::log_addition::union::LogUnionConst;

pub fn main() {     
     match default_open_kmsg() {
          Ok(kmsg) => {
               cluLog::set_logger(
                    kmsg.default_union(cluLog::log::default::LogDefault::default())
               );
          },
          _ => {
               cluLog::set_logger(
                    cluLog::log::default::LogDefault::default()
               );
          },
     }

     println!("123");
     trace!("Test");
     err!("Warning");
}