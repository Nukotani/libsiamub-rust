use curl::easy::{Easy, Form};

const URL: &str = "https://siam.ub.ac.id/index.php";


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

    pub fn login(&mut self, id: &str, pass: &str, handle: &mut Easy) {
        handle.url(URL).unwrap();

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

        {
            let mut transfer = handle.transfer();
            transfer.header_function(|header| {
                self.get_id_from_header(header);
                true
            }).unwrap();
            transfer.perform().unwrap();
        }
    }

    //TODO: OPTIMIZE!
    fn get_id_from_header(&mut self, header: &[u8]) {
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
}

