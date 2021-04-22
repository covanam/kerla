use alloc::sync::Arc;

use crate::{
    arch::UserVAddr,
    ctypes::*,
    fs::{
        inode::{FileLike, INode},
        opened_file::{OpenOptions, PathComponent},
    },
    pipe::Pipe,
    result::Result,
};
use crate::{process::current_process, syscalls::SyscallDispatcher};

use super::UserBufWriter;

impl<'a> SyscallDispatcher<'a> {
    pub fn sys_pipe(&mut self, fds: UserVAddr) -> Result<isize> {
        let options = OpenOptions::empty();

        let pipe = Pipe::new();
        let read_fd = current_process().opened_files.lock().open(
            PathComponent::new_anonymous(
                INode::FileLike(pipe.read_end() as Arc<dyn FileLike>).into(),
            ),
            options.into(),
        )?;

        let write_fd = current_process().opened_files.lock().open(
            PathComponent::new_anonymous(
                INode::FileLike(pipe.write_end() as Arc<dyn FileLike>).into(),
            ),
            options.into(),
        )?;

        let mut fds_writer = UserBufWriter::new(fds);
        fds_writer.write::<c_int>(read_fd.as_int())?;
        fds_writer.write::<c_int>(write_fd.as_int())?;
        Ok(0)
    }
}