
#[macro_use]
extern crate cluLog;
extern crate cluLogKmsg;

use cluLogKmsg::default_open_kmsg;
use cluLog::log_addition::union::LogUnionConst;

pub fn main() {     
     cluLog::set_boxed_logger({
          let kmsg = default_open_kmsg().unwrap();

          kmsg.default_union(cluLog::log::default::LogDefault::default()).to_box()
     });


     println!("123");
     trace!("Test");
     err!("Warning");
}