use std::fmt::{Debug, Formatter};
use savefile_derive::{Savefile, savefile_abi_exportable};



#[derive(Savefile)]
pub struct MyStuff {
    pub x: u64,
    pub y: [u64;10_000]
}


/*
#[savefile_abi_exportable(version=0)]
pub trait AdderCallback {
    fn set(&self, value: u32) -> ();
    fn get(&self) -> u32;
}
*/

#[savefile_abi_exportable(version=0)]
pub trait AdderInterface {
    fn add(&self, x: u32, y: &u32, z: &MyStuff) -> u32;
    fn sub(&self, x: u32, y: u32/*, cb: Box<dyn AdderCallback>*/) -> u32;
}
impl Debug for dyn AdderInterface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AdderInterface")
    }
}