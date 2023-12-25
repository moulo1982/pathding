use crate::id::id_generator::MAX_VALUE_U32;

pub type InstanceIdType = u128;

pub struct InstanceIdStruct {
    time: u64,
    process: u32,
    value: u32,
}

//从InstanceIdStruct转换成InstanceIdType
impl From<InstanceIdStruct> for InstanceIdType {
    fn from(value: InstanceIdStruct) -> Self {
        let time = (value.time as InstanceIdType) << 64;
        let process = (value.process as InstanceIdType) << 32 ;
        let value = value.value as InstanceIdType;
        time + process + value
    }
}

//从InstanceIdType转换成InstanceIdStruct
impl From<InstanceIdType> for InstanceIdStruct {
    fn from(id: InstanceIdType) -> Self {
        let mut result = id as u64;
        let value = (result & MAX_VALUE_U32 as u64) as u32;
        result >>= 32;
        let process = (result & MAX_VALUE_U32 as u64) as u32;
        result >>= 32;
        let time = result;
        InstanceIdStruct{time, process, value }
    }
}

impl InstanceIdStruct {

    #[allow(dead_code)]
    fn new_from_id(id: InstanceIdType) -> Self {
        id.into()
    }

    pub(crate) fn new_from_more(time: u64, process: u32, value: u32) -> Self {
        InstanceIdStruct{time, process, value}
    }
}