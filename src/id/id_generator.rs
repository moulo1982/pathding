use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{Utc};
use lazy_static::lazy_static;

use crate::id::instance_id::{InstanceIdStruct, InstanceIdType};

lazy_static! {
    static ref ID_GENERATOR: Arc<RwLock<IdGenerator>> = IdGenerator::new();
}

pub struct IdGenerator {
    last_instance_id_time: i64,
    instance_id_value: u64
}

pub const MAX_VALUE_U64: u64 = u64::MAX;

impl IdGenerator {

    pub fn get_instance() -> Arc<RwLock<IdGenerator>> {
        ID_GENERATOR.clone()
    }

    fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(IdGenerator{last_instance_id_time:Utc::now().timestamp(), instance_id_value:0}))
    }


    pub fn generate_instance_id(&mut self) -> InstanceIdType {
        let time = Utc::now().timestamp();
        if time > self.last_instance_id_time {
            self.last_instance_id_time = time;
            self.instance_id_value = 0;
        }
        else {
            self.instance_id_value += 1;
            if self.instance_id_value > MAX_VALUE_U64 - 1
            {
                self.last_instance_id_time = self.last_instance_id_time + 1; // 借用下一秒
                self.instance_id_value = 0;

                println!("instance id count per sec overflow: {} {}", time, self.last_instance_id_time);
            }
        }

        InstanceIdStruct::new_from_more(self.last_instance_id_time, self.instance_id_value).into()
    }
}