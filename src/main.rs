extern crate hyper;

use hyper::*;

use std::io::prelude::*;
use std::fs::File;

fn main() {

    let mut client = Client::new();
    let mut res = client.get("http://www.amazon.com.br/registry/wishlist/3DA4I0ZLH8ADW/ref=cm_sw_r_tw_ws_9hJzwb06V29HS").send().unwrap();
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    let mut buffer = try!(File::create("wishlist.html"));
    try!(buffer.write_all(s.as_bytes()));
}
