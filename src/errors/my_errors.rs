///
/// MyError实现了：
/// 1：Debug        -> 用来支持显示
/// 2：Display      -> 用来支持显示
/// 3：std::error::Error -> 说明自己是个Error类型
///
///

///如果自定义一个MyTrait，然后想用系统的derive来自动对MyStruct实现MyTrait，那么就需要对MyTrait使用“派生宏”特性
/// 可以参考serde库的Serialize与Deserialize
use thiserror::Error;
use crate::id::instance_id::InstanceIdType;

pub type RetErr = Box<dyn std::error::Error + Send + Sync>;
pub type RetResult<T> = Result<T, RetErr>;

#[derive(Error, Debug)] //让编译器自动实现Debug的trait（相当于接口）
pub enum MyError {
    #[error("地图{0}不存在")]
    MapNotExist(InstanceIdType),
}
