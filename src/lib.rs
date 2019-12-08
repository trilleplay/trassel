extern crate time;

pub mod generate {
    fn base_calc() -> f64 {
        let init_time = time::get_time();
        let mills: f64 = init_time.sec as f64 + (init_time.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
        let response = mills*1000.0;
        response
    }
    /// This function calculates a unique scaleable identifier, simply input a interger thats diffrent for each process/server and this function returns a u64 object.
    /// This function should not run in paralell with the same server_id is it can cause conflicts.
    #[no_mangle]
    pub fn gen_id() -> u64 {
        // 05/29/2019 @ 7:55am (UTC)
        let custom_epoch = self::base_calc() as u64 - 1559116500000;
        let id = custom_epoch * 1000;
        std::thread::sleep(1*durations::MILLISECOND);
        id
    }
}
