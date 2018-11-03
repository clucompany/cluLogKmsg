
#[macro_use]
extern crate cluLog;
extern crate cluLogKmsg;

use cluLogKmsg::default_open_kmsg;

pub fn main() {
     cluLog::set_logger(default_open_kmsg().unwrap());


     println!("123");
     trace!("Test");
     err!("Warning");
}