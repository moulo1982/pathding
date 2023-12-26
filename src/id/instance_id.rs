use crate::id::id_generator::MAX_VALUE_U64;

pub type InstanceIdType = u128;

pub struct InstanceIdStruct {
    time: i64,
    value: u64,
}

//从InstanceIdStruct转换成InstanceIdType
impl From<InstanceIdStruct> for InstanceIdType {
    fn from(value: InstanceIdStruct) -> Self {
        let time = (value.time as InstanceIdType) << 64;
        let value = value.value as InstanceIdType;
        time + value
    }
}

//从InstanceIdType转换成InstanceIdStruct
impl From<InstanceIdType> for InstanceIdStruct {
    fn from(id: InstanceIdType) -> Self {
        let mut result = id;
        let value = (result & (MAX_VALUE_U64 as u128)) as u64;
        result >>= 64;
        let time = result as i64;
        InstanceIdStruct{time, value }
    }
}

impl InstanceIdStruct {

    #[allow(dead_code)]
    fn new_from_id(id: InstanceIdType) -> Self {
        id.into()
    }

    pub(crate) fn new_from_more(time: i64, value: u64) -> Self {
        InstanceIdStruct{time, value}
    }
}