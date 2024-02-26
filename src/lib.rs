use curl::easy::{Easy, Form};

const DOMAIN: &str = "https://siam.ub.ac.id";

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

#[derive(Debug)]
pub struct Session {
    id: String,
}

impl Session {
    pub fn new() -> Session {
        let id = String::new();
        Session {
            id,
        }
    }

    //TODO: OPTIMIZE!
    pub fn get_id_from_header(&mut self, header: &[u8]) {
        let header = String::from_utf8(header.to_vec()).unwrap();

        for mut line in header.split("\n") {
            line = line.strip_prefix("\r").unwrap_or(line);
            if line.to_lowercase().contains("set-cookies") {
                line = line.strip_prefix("set-cookies:").unwrap_or(line);
                let mut id = line.split(";").next().unwrap();
                id = id.strip_prefix(" ").unwrap();
                self.id = id.to_string();
            }
        }
    }
}

pub fn login(id: &str, pass: &str, handle: &mut Easy) -> Session {
    handle.url(format!("{DOMAIN}/index.php").as_str()).unwrap();

    let mut form = Form::new();
    form.part("status_loc").contents("success".as_bytes()).add().unwrap();
    form.part("lat").contents("0".as_bytes()).add().unwrap();
    form.part("long").contents("0".as_bytes()).add().unwrap();
    form.part("username").contents(id.as_bytes()).add().unwrap();
    form.part("password").contents(pass.as_bytes()).add().unwrap();
    form.part("login").contents("masuk.".as_bytes()).add().unwrap();

    handle.post(true).unwrap();
    handle.httppost(form).unwrap();

    handle.verbose(true).unwrap();
    
    let mut session = Session::new();

    {
        let mut transfer = handle.transfer();
        transfer.header_function(|header| {
            session.get_id_from_header(header);
            true
        }).unwrap();
        transfer.perform().unwrap();
    }

    return session;
}

#[cfg(test)]
mod tests {
    #[test]
    fn login() {
    }
}
