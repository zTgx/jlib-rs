
#![allow(unused)]

use serde_json::json;
use serde_json::Result;
use std::any::Any;

//command转换相关的trait
pub trait CommandConversion {
    type T;
    fn to_string(&self) -> Result<String>;
    fn box_to_raw(&self) -> &dyn Any;

    //TODO::待实现 Box<T> -> T的转换
    //fn to_concrete<T>(&self, value: Box<dyn Any>) -> T ;
}