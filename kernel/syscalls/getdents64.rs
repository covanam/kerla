use crate::fs::opened_file::Fd;
use crate::{arch::UserVAddr, result::Result};
use crate::{process::current_process, syscalls::SyscallDispatcher};
use core::mem::size_of;
use penguin_utils::alignment::align_up;

use super::UserBufWriter;

impl<'a> SyscallDispatcher<'a> {
    pub fn sys_getdents64(&mut self, fd: Fd, dirp: UserVAddr, len: usize) -> Result<isize> {
        let opened_files = current_process().opened_files.lock();
        let mut dir = opened_files.get(fd)?.lock();
        let mut writer = UserBufWriter::new(dirp);
        while let Some(entry) = dir.readdir()? {
            let alignment = size_of::<u64>();
            let reclen = align_up(
                size_of::<u64>() * 2 + size_of::<u16>() + 1 + entry.name.len() + 1,
                alignment,
            );

            if writer.written_len() + reclen > len {
                break;
            }

            // Fill a `struct linux_dirent64`.
            // d_ino
            writer.write::<u64>(entry.inode_no.as_u64())?;
            // d_off
            writer.write::<u64>(dir.pos() as u64)?;
            // d_reclen
            writer.write::<u16>(reclen as u16)?;
            // d_type
            writer.write::<u8>(entry.file_type as u8)?;
            // d_name
            writer.write_bytes(entry.name.as_bytes())?;
            // d_name (null character)
            writer.write::<u8>(0)?;

            writer.skip_until_alignment(alignment);
        }

        Ok(writer.written_len() as isize)
    }
}