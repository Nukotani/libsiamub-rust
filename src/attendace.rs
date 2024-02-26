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

    }
}
pub struct AttendanceEntry {
    code: String,
    name: String,
    class: String,
    percentage: u8,
    details: Option<AttendanceEntryDetail>,
}

struct AttendanceEntryDetail {

}
