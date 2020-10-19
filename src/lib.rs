extern crate time;

pub mod generate {

    const TOTAL_BITS: i64 = 64;
    const EPOCH_BITS: i64 = 42;
    const SEQUENCE_BITS: i64 = 12;
    const NODE_ID_BITS: i64 = 10;

    pub struct Factory {
        seq: i64,
        time: i64,
        max_seq: i64,
        node_id: i64,
        epoch: f64
    }

    impl Factory {

        /// Use the stored struct to generate a unique id
        pub fn generate(&mut self) -> i64 {
            let mut current_time_stamp = get_timestamp(self.epoch);
            if current_time_stamp == self.time {
                self.seq = (self.seq + 1) & self.max_seq;
                if self.seq == 0 {
                    while self.time == current_time_stamp {
                        current_time_stamp = get_timestamp(self.epoch);
                    }
                }
            }
            else {
                self.seq = 0
            }
            self.time = get_timestamp(self.epoch);
            let mut id = self.time << (TOTAL_BITS - EPOCH_BITS);
            id = id | (self.node_id << (TOTAL_BITS - EPOCH_BITS - NODE_ID_BITS));
            id = id | self.seq;
            return id
        }


    }

    fn get_timestamp(epoch: f64) -> i64 {
        let init_time = time::get_time();
        return ((init_time.sec as f64 + (init_time.nsec/1000000000) as f64)*1000.0 - epoch) as i64;
    }

    /// Get a Factory struct which you can use to generate IDs.
    pub fn get_factory(node_id: i64, epoch: f64) -> Result<Factory, String> {
        if node_id.is_negative() {
            return Err("The node_id field can't be negative.".to_string())
        }
        let max_sequence = ((2 as i64).pow(SEQUENCE_BITS as u32))-1;
        if node_id > max_sequence {
            return Err("The specified node_id is invalid.".to_string())
        }
        let f = Factory {
            seq: 0,
            time: 0,
            max_seq: max_sequence,
            node_id,
            epoch
        };
        return Ok(f)
    }
}
