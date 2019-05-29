extern crate time;
use std::thread;
use durations::{SECOND as S, MILLISECOND as MS};

pub mod generate {
    fn base_calc() -> f64 {
        let init_time = time::get_time();
        let mills: f64 = init_time.sec as f64 + (init_time.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
        let response = mills*10000000.0;
        response
    }
    #[no_mangle]
    pub fn gen_id(server_id: i32, random_process_id: i32) -> u64 {
        if (random_process_id > 1000) {
            panic!("The Random Process ID can not be above 1000.");
        }
        if (server_id * 3 > random_process_id) {
            panic!("The serverid times three can not be larger then the process id.");
        }
        let serverid = server_id as u64;
        let randomprocessid = random_process_id as u64;
        // 05/29/2019 @ 7:55am (UTC)
        let custom_epoch = self::base_calc() as u64 - 15591165000000000;
        let epoch_scale = custom_epoch * 1000;
        let id = epoch_scale + randomprocessid - serverid * 3;
        std::thread::sleep(3*durations::MILLISECOND);
        id
    }
}