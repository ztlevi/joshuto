extern crate libc;

pub const BITMASK  : u32 = 0o170000;

pub const S_IFSOCK : u32 = 0o140000;   /* socket */
pub const S_IFLNK  : u32 = 0o120000;   /* symbolic link */
pub const S_IFREG  : u32 = 0o100000;   /* regular file */
pub const S_IFBLK  : u32 = 0o060000;   /* block device */
pub const S_IFDIR  : u32 = 0o040000;   /* directory */
pub const S_IFCHR  : u32 = 0o020000;   /* character device */
pub const S_IFIFO  : u32 = 0o010000;   /* FIFO */

pub fn is_executable(mode : u32) -> bool
{
    const LIBC_PERMISSION_VALS : [(u32, char) ; 3] = [
            (libc::S_IXUSR, 'x'),
            (libc::S_IXGRP, 'x'),
            (libc::S_IXOTH, 'x'),
        ];
    for val in LIBC_PERMISSION_VALS.iter() {
        if mode & val.0 != 0 {
            return true;
        }
    }
    return false;
}

pub fn stringify_mode(mode : u32) -> String
{
    let mut mode_str : String = String::with_capacity(10);

    const LIBC_FILE_VALS : [(u32, char) ; 7] = [
        (S_IFSOCK, 's'),
        (S_IFLNK, 'l'),
        (S_IFREG, '-'),
        (S_IFBLK, 'b'),
        (S_IFDIR, 'd'),
        (S_IFCHR, 'c'),
        (S_IFIFO, 'f'),
    ];

    for val in LIBC_FILE_VALS.iter() {
        if mode & val.0 != 0 {
            mode_str.push(val.1);
            break;
        }
    }

    const LIBC_PERMISSION_VALS : [(u32, char) ; 9] = [
            (libc::S_IRUSR, 'r'),
            (libc::S_IWUSR, 'w'),
            (libc::S_IXUSR, 'x'),
            (libc::S_IRGRP, 'r'),
            (libc::S_IWGRP, 'w'),
            (libc::S_IXGRP, 'x'),
            (libc::S_IROTH, 'r'),
            (libc::S_IWOTH, 'w'),
            (libc::S_IXOTH, 'x'),
    ];

    for val in LIBC_PERMISSION_VALS.iter() {
        if mode & val.0 != 0 {
            mode_str.push(val.1);
        } else {
            mode_str.push('-');
        }
    }
    mode_str
}