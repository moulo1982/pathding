

pub type InstanceIdType = u128;

pub struct InstanceIdStruct {
    time: u64,
    process: u32,
    value: u32,
}

impl Into<InstanceIdType> for InstanceIdStruct {
    fn into(self) -> InstanceIdType {
        let time = (self.time as InstanceIdType) << 64;
        let process = (self.process as InstanceIdType) << 32 ;
        let value = self.value as InstanceIdType;
        time + process + value
    }
}

impl InstanceIdStruct {

    #[allow(dead_code)]
    fn new_from_u64(id: InstanceIdType) -> Self {
        let mut result = id;
        let value = (result & 0xffffffff) as u32;
        result >>= 64;
        let process = (result & 0xffff) as u32;
        result >>= 64;
        let time = result;
        InstanceIdStruct{time: time as u64, process, value }
    }

    pub(crate) fn new_from_more(time: u64, process: u32, value: u32) -> Self {
        InstanceIdStruct{time, process, value}
    }

}