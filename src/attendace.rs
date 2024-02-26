use curl::easy::Easy;

use crate::session::{self, Session};


const URL: &str = "https://siam.ub.ac.id/absensi.php";

pub struct AttendanceList {
    list: Vec<AttendanceEntry>,
}

impl AttendanceList {
    pub fn new() -> AttendanceList {
        let list = Vec::<AttendanceEntry>::new();
        AttendanceList {
            list
        }
    }

    pub fn fetch_list(&mut self, session: &Session, handle: &mut Easy) {
        handle.url(URL).unwrap();
        handle.get(true).unwrap();
        handle.cookie(session.get_id()).unwrap();
        
        {
            let mut transfer = handle.transfer();

            transfer.write_function(|body|
                Ok(body.len())
            ).unwrap();
        }
    }
}

pub struct AttendanceEntry {
    code: String,
    name: String,
    class: String,
    stat: AttendanceStat,
    details: Option<AttendanceDetail>,
}

struct AttendanceStat {

}

struct AttendanceDetail {
}
