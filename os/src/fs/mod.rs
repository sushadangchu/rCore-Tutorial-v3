mod pipe;
mod stdio;
mod inode;

use memory::UserBuffer;
use core::any::Any;

pub trait File : Any + Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    fn as_any_ref(&self) -> &dyn Any;
}

impl dyn File {
    #[allow(unused)]
    pub fn downcast_ref<T: File>(&self) -> Option<&T> {
        self.as_any_ref().downcast_ref::<T>()
    }
}

pub use pipe::{Pipe, make_pipe};
pub use stdio::{Stdin, Stdout};
pub use inode::{OSInode, open_file, OpenFlags, list_apps};