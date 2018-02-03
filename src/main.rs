extern crate ssh2;
pub mod ssh;
use std::io::Read;

struct SSH_CONNECT_INFO{
    host:String,
    port:u16,
    user:String,
    pass:Option<String>,
    agent_auth:bool
}

fn callback_ls(mut ch:ssh2::Channel)->String{
    ch.exec("ls /").unwrap();
    let mut s = String::new();
    ch.read_to_string(&mut s).unwrap();
    s.replace("\n"," ")
}

fn main() {
    let mut ssh=ssh::SSH2::new("192.168.56.1",22);
    ssh.connect("segfo",Some("password")).unwrap();
    println!("ls / -> {}",ssh.sendcmd(callback_ls).unwrap());
}
