use curl::easy::{Easy, Form};

const DOMAIN: &str = "https://siam.ub.ac.id";

pub struct Session {
    cookies: Vec<String>,
}

impl Session {
    pub fn new() -> Session {
        let cookies = Vec::<String>::new();
        Session {
            cookies
        }
    }

    pub fn add_cookie(&mut self, cookie: String) {
        self.cookies.push(cookie);
    }

    //TODO: OPTIMIZE!
    pub fn get_cookeis_from_header(&mut self, header: &[u8]) {
        let header = String::from_utf8(header.to_vec()).unwrap();

        for mut line in header.split("\n") {
            line = line.strip_prefix("\r").unwrap_or(line);
            if line.to_lowercase().contains("set-cookies") {
                line = line.strip_prefix("set-cookies:").unwrap_or(line);
                for mut cookie in line.split(";") {
                    cookie = cookie.strip_prefix(" ").unwrap_or(cookie); 
                    self.add_cookie(cookie.to_string());
                }
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
            session.get_cookeis_from_header(header);
            true
        }).unwrap();
        transfer.perform().unwrap();
    }

    return session;
}
