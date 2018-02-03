use ssh2::{Session,Channel};
use std::io::prelude::*;
use std::net::TcpStream;

pub struct SSH2{
    socket:Option<TcpStream>,
    session:Option<Session>,
    host:String,
    port:u16,
}

#[derive(Debug)]
pub enum SSH2Error{
    Io(::std::io::Error),
    Protocol(::ssh2::Error),
}

impl From<::std::io::Error> for SSH2Error {
    fn from(err: ::std::io::Error) -> SSH2Error {
        SSH2Error::Io(err)
    }
}

impl From<::ssh2::Error> for SSH2Error {
    fn from(err: ::ssh2::Error) -> SSH2Error {
        SSH2Error::Protocol(err)
    }
}

impl<'ch> SSH2{
    pub fn new(host:&str,port:u16)->Self{
        let ssh=SSH2{
            socket:None,
            session:None,
            host:host.to_owned(),
            port:port,
        };
        ssh
    }

    fn is_authenticated(&self)->Result<bool,::std::io::Error>{
        if !self.session.as_ref().unwrap().authenticated(){
            Err(::std::io::Error::new(
                ::std::io::ErrorKind::PermissionDenied,
                "Authentication failure."))
        }else{
            Ok(true)
        }
    }

    fn authentication(&self,user:&str,pass:Option<&str>)->Result<bool,SSH2Error>{
        let sess=self.session.as_ref().unwrap();
        if let Some(pass)=pass{
            // password
            sess.userauth_password(user,pass);
        }else{
            // ssh-agent
            println!("ssh-agent");
        }
        let success = self.is_authenticated()?;
        Ok(success)
    }

    pub fn connect(&mut self,user:&str,pass:Option<&str>)->Result<&Self,SSH2Error>{
        let socket=TcpStream::connect(format!("{}:{}",self.host,self.port))?;
        let mut session = Session::new()
                        .ok_or(::std::io::Error::new(
                            ::std::io::ErrorKind::ConnectionAborted,
                            "session-object not generated."))?;
        session.handshake(&socket)?;
        self.socket=Some(socket);
        self.session=Some(session);
        self.authentication(user, pass)?;
        Ok(self)
    }

    pub fn sendcmd(&self,callback:fn(Channel)->String)->Result<String,SSH2Error>{
        let mut channel = self.session.as_ref().unwrap().channel_session()?;
        Ok(callback(channel))
    }
}
