use std::{default, str::Bytes};

use curl::easy::{Easy, Form, WriteError};

const URL: &str = "https://siam.ub.ac.id/index.php";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Session {
    handle: Easy,
    id: String,
}

impl Session {
    pub fn new() -> Session {
        let id = String::new();
        let mut handle = Easy::new();
        handle.useragent(USER_AGENT).unwrap();
        Session {
            handle,
            id,
        }
    }

    //TODO: OPTIOMIZE! 
    pub fn login(&mut self, id: &str, pass: &str) {
        let mut form = Form::new();
        form.part("status_loc").contents("success".as_bytes()).add().unwrap();
        form.part("lat").contents("0".as_bytes()).add().unwrap();
        form.part("long").contents("0".as_bytes()).add().unwrap();
        form.part("username").contents(id.as_bytes()).add().unwrap();
        form.part("password").contents(pass.as_bytes()).add().unwrap();
        form.part("login").contents("masuk.".as_bytes()).add().unwrap();

        self.handle.url(URL).unwrap();
        self.handle.post(true).unwrap();
        self.handle.httppost(form).unwrap();

        let mut buf: String = Default::default();
        {
            let mut transfer = self.handle.transfer();

            transfer.header_function(|header| {
                buf = String::from_utf8(header.to_vec()).unwrap();
                true
            }).unwrap();
            transfer.perform().unwrap();
        }

        self.set_id_from_header(buf.as_bytes());
    }

    //TODO: OPTIMIZE!
    fn set_id_from_header(&mut self, header: &[u8]) {
        let header = String::from_utf8(header.to_vec()).unwrap();

        for mut line in header.split("\n") {
            line = line.strip_prefix("\r").unwrap_or(line);
            if line.to_lowercase().contains("set-cookie") {
                line = line.strip_prefix("set-cookie:").unwrap_or(line);
                let mut id = line.split(";").next().unwrap();
                id = id.strip_prefix(" ").unwrap();
                self.id = id.to_string();
            }
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn connect(&mut self, url: &str) {
        self.handle.cookie(self.id.as_str()).unwrap();
        self.handle.url(url).unwrap();
        self.handle.get(true).unwrap();
    }

    pub fn write_function<F>(&mut self, f: F)
    where
        F: FnMut(&[u8]) -> Result<usize, WriteError>,
    {
        let mut transfer = self.handle.transfer(); 

        transfer.write_function(f).unwrap();
    }
}

