use scraper::{Html, Selector};

use crate::session::Session;


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

    //TODO: OPTIMIZE!
    pub fn fetch_list(&mut self, session: &mut Session) {
        session.connect(URL);

        let mut buf = String::new();
        session.write_function(|body| {
            buf = String::from_utf8(body.to_vec()).unwrap();
            Ok(body.len())
        });
        
    }

    fn parse_body(body: &str) {
        let document = Html::parse_document(&body);
        let selector = Selector::parse("table.table-bordered").unwrap();

        let table = document.select(&selector).next().unwrap();
        let inner_table = table.first_child().unwrap().first_child().unwrap()
            .first_child().unwrap();
        let mut inner_table_iterator = inner_table.children();
        inner_table_iterator.next();

        for entry in inner_table_iterator {
            let mut entry_iterator = entry.children();
            let code: String = entry_iterator.next().unwrap().first_child().unwrap().value()
                .as_text().unwrap().to_string();

            let name: String = entry_iterator.next().unwrap().first_child().unwrap().value()
                .as_text().unwrap().to_string();

            let class: String = entry_iterator.next().unwrap().first_child().unwrap().value()
                .as_text().unwrap().to_string();

            let present: u32 = entry_iterator.next().unwrap().first_child().unwrap().value()
                .as_text().unwrap().to_string().parse::<u32>().unwrap();

            let unexcused: u32 = entry_iterator.next().unwrap().first_child().unwrap().value()
                .as_text().unwrap().to_string().parse::<u32>().unwrap();

            let excused: u32 = entry_iterator.next().unwrap().first_child().unwrap().value()
                .as_text().unwrap().to_string().parse::<u32>().unwrap();

            let medical: u32 = entry_iterator.next().unwrap().first_child().unwrap().value()
                .as_text().unwrap().to_string().parse::<u32>().unwrap();

            let stat = AttendanceStat::new(present, unexcused, excused, medical);
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

impl AttendanceEntry {
    pub fn new(code: &str, name: &str, class: &str, stat: AttendanceStat) -> AttendanceEntry {
        let code = code.to_string();
        let name = name.to_string();
        let class = class.to_string();
        let details = None;
        let stat = stat;

        AttendanceEntry {
            code,
            name,
            class,
            details,
            stat,
        }
    }
} 

struct AttendanceStat {
    present: u32,
    unexcused: u32,
    excused: u32,
    medical: u32,
}

impl AttendanceStat {
    pub fn new(present: u32, unexcused: u32, excused: u32, medical: u32) -> AttendanceStat {
        AttendanceStat {
            present,
            unexcused,
            excused,
            medical,
        }
    }
}

struct AttendanceDetail {
    lesson: String,
    //TODO: use a more suited type!
    note: String,
    status: AttendanceStatus,
}

enum AttendanceStatus {
    Present,
    Unexcused,
    Excused,
    Medical,
}
