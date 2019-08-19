extern crate open62541_sys;

fn main() {
    let server = unsafe { open62541_sys::UA_Server_new() };
    println!("Narf!");
}