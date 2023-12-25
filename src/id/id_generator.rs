use std::sync::{Arc, RwLock};
use chrono::{Datelike, TimeZone, Utc};
use lazy_static::lazy_static;

use crate::id::instance_id::{InstanceIdStruct, InstanceIdType};

lazy_static! {
    pub static ref ID_GENERATOR: Arc<RwLock<IdGenerator>> = IdGenerator::new();
}

pub struct IdGenerator {
    epoch_this_year: i64,

    last_instance_id_time: u64,

    instance_id_value: u32
}

pub const MAX_VALUE_U32: u32 = u32::MAX;

impl IdGenerator {
    pub fn new() -> Arc<RwLock<Self>> {
        let epoch_this_year = Utc.with_ymd_and_hms(Utc::now().year(), 1, 1, 0, 0, 0).unwrap().timestamp();

        let last_instance_id_time = (Utc::now().timestamp() - epoch_this_year) as u64;
        if last_instance_id_time <= 0 {
            panic!()
        }

        Arc::new(RwLock::new(IdGenerator{epoch_this_year, last_instance_id_time, instance_id_value:0}))
    }

    fn time_since_this_year(&self) -> u64 {
        (Utc::now().timestamp() - self.epoch_this_year) as u64
    }

    pub fn generate_instance_id(&mut self) -> InstanceIdType {
        let time = self.time_since_this_year();
        if time > self.last_instance_id_time {
            self.last_instance_id_time = time;
            self.instance_id_value = 0;
        }
        else {
            self.instance_id_value += 1;
            if self.instance_id_value > MAX_VALUE_U32 - 1
            {
                self.last_instance_id_time = self.last_instance_id_time + 1; // 借用下一秒
                self.instance_id_value = 0;

                println!("instance id count per sec overflow: {} {}", time, self.last_instance_id_time);
            }
        }

        InstanceIdStruct::new_from_more(self.last_instance_id_time, 1, self.instance_id_value).into()
    }
}