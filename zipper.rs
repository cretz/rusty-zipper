extern mod std;
use libc::*;

pub mod api {
    /* flags for zip_open */

    pub const ZIP_CREATE           : c_int = 1;
    pub const ZIP_EXCL             : c_int = 2;
    pub const ZIP_CHECKCONS        : c_int = 4;

    /* flags for zip_name_locate, zip_fopen, zip_stat, ... */

    pub const ZIP_FL_NOCASE       : c_int = 1;  /* ignore case on name lookup */
    pub const ZIP_FL_NODIR        : c_int = 2;  /* ignore directory component */
    pub const ZIP_FL_COMPRESSED   : c_int = 4;  /* read compressed data */
    pub const ZIP_FL_UNCHANGED    : c_int = 8;  /* use original data, ignoring changes */
    pub const ZIP_FL_RECOMPRESS   : c_int = 16; /* force recompression of data */
    pub const ZIP_FL_ENCRYPTED    : c_int = 32; /* read encrypted data (implies ZIP_FL_COMPRESSED) */

    /* archive global flags flags */

    pub const ZIP_AFL_TORRENT     : c_int = 1; /* torrent zipped */
    pub const ZIP_AFL_RDONLY      : c_int = 2; /* read only -- cannot be cleared */

    /* flags for compression and encryption sources */

    pub const ZIP_CODEC_ENCODE    : c_int = 1; /* compress/encrypt */

    /* libzip error codes */

    pub const ZIP_ER_OK             : c_int = 0;   /* N No error */
    pub const ZIP_ER_MULTIDISK      : c_int = 1;   /* N Multi-disk zip archives not supported */
    pub const ZIP_ER_RENAME         : c_int = 2;   /* S Renaming temporary file failed */
    pub const ZIP_ER_CLOSE          : c_int = 3;   /* S Closing zip archive failed */
    pub const ZIP_ER_SEEK           : c_int = 4;   /* S Seek error */
    pub const ZIP_ER_READ           : c_int = 5;   /* S Read error */
    pub const ZIP_ER_WRITE          : c_int = 6;   /* S Write error */
    pub const ZIP_ER_CRC            : c_int = 7;   /* N CRC error */
    pub const ZIP_ER_ZIPCLOSED      : c_int = 8;   /* N Containing zip archive was closed */
    pub const ZIP_ER_NOENT          : c_int = 9;   /* N No such file */
    pub const ZIP_ER_EXISTS         : c_int = 10;  /* N File already exists */
    pub const ZIP_ER_OPEN           : c_int = 11;  /* S Can't open file */
    pub const ZIP_ER_TMPOPEN        : c_int = 12;  /* S Failure to create temporary file */
    pub const ZIP_ER_ZLIB           : c_int = 13;  /* Z Zlib error */
    pub const ZIP_ER_MEMORY         : c_int = 14;  /* N Malloc failure */
    pub const ZIP_ER_CHANGED        : c_int = 15;  /* N Entry has been changed */
    pub const ZIP_ER_COMPNOTSUPP    : c_int = 16;  /* N Compression method not supported */
    pub const ZIP_ER_EOF            : c_int = 17;  /* N Premature EOF */
    pub const ZIP_ER_INVAL          : c_int = 18;  /* N Invalid argument */
    pub const ZIP_ER_NOZIP          : c_int = 19;  /* N Not a zip archive */
    pub const ZIP_ER_INTERNAL       : c_int = 20;  /* N Internal error */
    pub const ZIP_ER_INCONS         : c_int = 21;  /* N Zip archive inconsistent */
    pub const ZIP_ER_REMOVE         : c_int = 22;  /* S Can't remove file */
    pub const ZIP_ER_DELETED        : c_int = 23;  /* N Entry has been deleted */
    pub const ZIP_ER_ENCRNOTSUPP    : c_int = 24;  /* N Encryption method not supported */
    pub const ZIP_ER_RDONLY         : c_int = 25;  /* N Read-only archive */ 
    pub const ZIP_ER_NOPASSWD       : c_int = 26;  /* N No password provided */
    pub const ZIP_ER_WRONGPASSWD    : c_int = 27;  /* N Wrong password provided */

    /* type of system error value */

    pub const ZIP_ET_NONE       : c_int = 0;  /* sys_err unused */
    pub const ZIP_ET_SYS        : c_int = 1;  /* sys_err is errno */
    pub const ZIP_ET_ZLIB       : c_int = 2;  /* sys_err is zlib error code */

    /* compression methods */

    pub const ZIP_CM_DEFAULT         : c_int = -1;  /* better of deflate or store */
    pub const ZIP_CM_STORE           : c_int = 0;   /* stored (uncompressed) */
    pub const ZIP_CM_SHRINK          : c_int = 1;   /* shrunk */
    pub const ZIP_CM_REDUCE_1        : c_int = 2;   /* reduced with factor 1 */
    pub const ZIP_CM_REDUCE_2        : c_int = 3;   /* reduced with factor 2 */
    pub const ZIP_CM_REDUCE_3        : c_int = 4;   /* reduced with factor 3 */
    pub const ZIP_CM_REDUCE_4        : c_int = 5;   /* reduced with factor 4 */
    pub const ZIP_CM_IMPLODE         : c_int = 6;   /* imploded */
    /* 7 - Reserved for Tokenizing compression algorithm */
    pub const ZIP_CM_DEFLATE         : c_int = 8;   /* deflated */
    pub const ZIP_CM_DEFLATE64       : c_int = 9;   /* deflate64 */
    pub const ZIP_CM_PKWARE_IMPLODE  : c_int = 10;  /* PKWARE imploding */
    /* 11 - Reserved by PKWARE */
    pub const ZIP_CM_BZIP2           : c_int = 12;  /* compressed using BZIP2 algorithm */
    /* 13 - Reserved by PKWARE */
    pub const ZIP_CM_LZMA            : c_int = 14;  /* LZMA (EFS) */
    /* 15-17 - Reserved by PKWARE */
    pub const ZIP_CM_TERSE           : c_int = 18;  /* compressed using IBM TERSE (new) */
    pub const ZIP_CM_LZ77            : c_int = 19;  /* IBM LZ77 z Architecture (PFS) */
    pub const ZIP_CM_WAVPACK         : c_int = 97;  /* WavPack compressed data */
    pub const ZIP_CM_PPMD            : c_int = 98;  /* PPMd version I, Rev 1 */

    /* encryption methods */

    pub const ZIP_EM_NONE        : c_int = 0;       /* not encrypted */
    pub const ZIP_EM_TRAD_PKWARE : c_int = 1;       /* traditional PKWARE encryption */
    pub const ZIP_EM_UNKNOWN     : c_int = 0xffff;  /* unknown algorithm */

    pub enum zip_source_cmd {
        ZIP_SOURCE_OPEN,    /* prepare for reading */
        ZIP_SOURCE_READ,    /* read data */
        ZIP_SOURCE_CLOSE,   /* reading is done */
        ZIP_SOURCE_STAT,    /* get meta information */
        ZIP_SOURCE_ERROR,   /* get error information */
        ZIP_SOURCE_FREE     /* cleanup and free resources */
    }

    pub const ZIP_SOURCE_ERR_LOWER    : c_int = -2;

    pub const ZIP_STAT_NAME               : c_int = 0x0001;
    pub const ZIP_STAT_INDEX              : c_int = 0x0002;
    pub const ZIP_STAT_SIZE               : c_int = 0x0004;
    pub const ZIP_STAT_COMP_SIZE          : c_int = 0x0008;
    pub const ZIP_STAT_MTIME              : c_int = 0x0010;
    pub const ZIP_STAT_CRC                : c_int = 0x0020;
    pub const ZIP_STAT_COMP_METHOD        : c_int = 0x0040;
    pub const ZIP_STAT_ENCRYPTION_METHOD  : c_int = 0x0080;
    pub const ZIP_STAT_FLAGS              : c_int = 0x0100;

    pub struct zip_stat_struct {
        valid: uint64_t,                /* which fields have valid values */
        name: *c_char,                  /* name of the file */
        index: uint64_t,                /* index within archive */
        size: uint64_t,                 /* size of file (uncompressed) */
        comp_size: uint64_t,            /* size of file (compressed) */
        mtime: time_t,                  /* modification time */
        crc: uint32_t,                  /* crc of file data */
        comp_method: uint16_t,          /* compression method used */
        encryption_method: uint16_t,    /* encryption method used */
        flags: uint32_t,                /* reserved for future use */
    }

    pub enum zip {}
    pub enum zip_file {}
    pub enum zip_source {}

    pub extern mod zip {
        pub fn zip_add(archive: *zip, name: *c_char, source: *zip_source) -> int64_t;
        pub fn zip_add_dir(archive: *zip, name: *c_char) -> int64_t;
        pub fn zip_close(archive: *zip) -> c_int;
        pub fn zip_delete(archive: *zip, index: uint64_t) -> c_int;
        pub fn zip_error_clear(archive: *zip);
        pub fn zip_error_get(arhive: *zip, zep: *c_int, sep: *c_int);
        pub fn zip_error_get_sys_type(ze: c_int) -> c_int;
        pub fn zip_error_to_str(buf: *c_char, len: uint64_t, ze: c_int, se: c_int) -> c_int;
        pub fn zip_fclose(file: *zip_file) -> c_int;
        pub fn zip_fdopen(fd: c_int, flags: c_int, errorp: *c_int) -> *zip;
        pub fn zip_file_error_clear(file: *zip_file);
        pub fn zip_file_error_get(file: *zip_file, zep: *c_int, sep: *c_int);
        pub fn zip_file_strerror(file: *zip_file) -> *c_char;
        pub fn zip_fopen(archive: *zip, fname: *c_char, flags: c_int) -> *zip_file;
        pub fn zip_fopen_encrypted(archive: *zip, fname: *c_char, 
            flags: c_int, password: *c_char) -> *zip_file;
        pub fn zip_fopen_index(archive: *zip, index: uint64_t, flags: c_int) -> *zip_file;
        pub fn zip_fopen_index_encrypted(archive: *zip, index: uint64_t, 
            flags: c_int, password: *c_char) -> *zip_file;
        pub fn zip_fread(file: *zip_file, buf: *c_void, nbytes: uint64_t) -> int64_t;
        pub fn zip_get_archive_comment(archive: *zip, lenp: *c_int, flags: c_int) -> *c_char;
        pub fn zip_get_archive_flag(archive: *zip, flag: c_int, flags: c_int) -> c_int;
        pub fn zip_get_file_comment(archive: *zip, index: uint64_t,
            lenp: *c_int, flags: c_int) -> *c_char;
        pub fn zip_get_file_extra(archive: *zip, index: uint64_t,
            lenp: *c_int, flags: c_int) -> *c_char;
        pub fn zip_get_name(archive: *zip, index: uint64_t, flags: c_int) -> *c_char;
        pub fn zip_get_num_entries(archive: *zip, flags: c_int) -> uint64_t;
        pub fn zip_name_locate(archive: *zip, fname: *c_char, flags: c_int) -> c_int;
        pub fn zip_open(path: *c_char, flags: c_int, errorp: *c_int) -> *zip;
        pub fn zip_rename(archive: *zip, index: uint64_t, name: *c_char) -> c_int;
        pub fn zip_replace(archive: *zip, index: uint64_t, source: *zip_source) -> c_int;
        pub fn zip_set_archive_comment(archive: *zip, comment: *c_char, len: c_int) -> c_int;
        pub fn zip_set_archive_flag(archive: *zip, flag: c_int, value: c_int) -> c_int;
        pub fn zip_set_default_password(archive: *zip, password: *c_char) -> c_int;
        pub fn zip_set_file_comment(archive: *zip, index: uint64_t,
            password: *c_char, len: c_int) -> c_int;
        pub fn zip_set_file_extra(archive: *zip, index: uint64_t,
            extra: *c_char, len: c_int) -> c_int;
        pub fn zip_source_buffer(archive: *zip, data: *c_void,
            len: uint64_t, freep: c_int) -> *zip_source;
        pub fn zip_source_file(archive: *zip, fname: *c_char,
            start: uint64_t, len: int64_t) -> *zip_source;
        pub fn zip_source_filep(archive: *zip, file: *libc::FILE,
            start: uint64_t, len: int64_t) -> *zip_source;
        pub fn zip_source_free(source: *zip_source);
        //pub fn zip_source_function(archive: *zip,
        //    fn: zip_source_callback, userdata: *c_void) -> *zip_source;
        pub fn zip_source_zip(archive: *zip, srcarchive: *zip,
            srcidx: uint64_t, flags: c_int, start: uint64_t, len: int64_t) -> *zip_source;
        pub fn zip_stat(archive: *zip, fname: *c_char, flags: c_int, sb: *zip_stat_struct) -> c_int;
        pub fn zip_stat_index(archive: *zip, index: uint64_t, flags: c_int,
            sb: *zip_stat_struct) -> c_int;
        pub fn zip_stat_init(sb: *zip_stat_struct);
        pub fn zip_strerror(archive: *zip) -> *c_char;
        pub fn zip_unchange(archive: *zip, index: uint64_t) -> c_int;
        pub fn zip_unchange_all(archive: *zip) -> c_int;
        pub fn zip_unchange_archive(archive: *zip) -> c_int;
    }

    use zip::*;

    #[cfg(test)]
    mod api_tests {

        #[test]
        fn open() {
            let err: c_int = 0;
            let z = str::as_c_str("test_resources/test.zip", { |fname| zip_open(fname, 0, ptr::addr_of(&err)) });
            if ptr::is_null(z) { fail fmt!("Error while opening zip - %?", err); }
            let num = zip_get_num_entries(z, 0);
            assert num == 3;
            assert zip_close(z) == 0;
        }
    }
}

use api::*;

pub struct ZipError {
    priv err: i32
}

impl ZipError: ToStr {
    pure fn to_str() -> ~str unsafe {
        let mut s = ~"";
        str::reserve(&mut s, 1024);
        do os::as_c_charp(s) |cstr| { 
            zip_error_to_str(cstr, 1024, self.err, 0);
            str::raw::from_c_str(cstr)
        }
    }
}

pub struct ZipArchive {
    priv archive: *zip,
    priv mut open: bool,

    drop {
        if self.open {
            zip_close(self.archive);
            self.open = false;
        }
    }
}

pub fn open_archive(path: ~str, flags: i32) -> Result<ZipArchive, ZipError> {
    let err: c_int = 0;
    let z = do os::as_c_charp(path) |fname| { zip_open(fname, flags, ptr::addr_of(&err)) };
    if ptr::is_null(z) {
        result::Err(ZipError {err: err})
    } else {
        result::Ok(ZipArchive {archive: z, open: true})
    }
}

pub fn open_archive_fd(fd: i32, flags: i32) -> Result<ZipArchive, ZipError> {
    let err: c_int = 0;
    let z = zip_fdopen(fd, flags, ptr::addr_of(&err));
    if ptr::is_null(z) {
        result::Err(ZipError {err: err})
    } else {
        result::Ok(ZipArchive {archive: z, open: true})
    }
}

fn empty_stat() -> zip_stat_struct {
    zip_stat_struct {
        valid: 0,
        name: ptr::null(),
        index: 0,
        size: 0,
        comp_size: 0,
        mtime: 0,
        crc: 0,
        comp_method: 0,
        encryption_method: 0,
        flags: 0
    }
}

impl ZipArchive {
    pub fn isOpen() -> bool { self.open }

    pub fn close() -> bool {
        if !self.open || zip_close(self.archive) == 0 {
            self.open = false;
            true
        } else {
            false
        }
    }

    pub fn locate(fname: ~str, flags: i32) -> Option<i32> {
        let res = do os::as_c_charp(fname) |fname| { zip_name_locate(self.archive, fname, flags) };
        if res == -1 {
            None
        } else {
            Some(res)
        }
    }
    
    pub fn open_name(fname: ~str, flags: i32) -> Option<ZipFile> {
        let file = do os::as_c_charp(fname) |fname| { zip_fopen(self.archive, fname, flags) };
        if ptr::is_null(file) {
            None
        } else {
            Some(ZipFile {file: file, open: true})
        }
    }

    pub fn open_index(index: u64, flags: i32) -> Option<ZipFile> {
        let file = zip_fopen_index(self.archive, index, flags);
        if ptr::is_null(file) {
            None
        } else {
            Some(ZipFile {file: file, open: true})
        }
    }

    pub fn stat_name(fname: ~str, flags: i32) -> Option<ZipStat> {
        let sb = empty_stat();
        zip_stat_init(ptr::addr_of(&sb));
        let sb_res = do os::as_c_charp(fname) |fname| { zip_stat(self.archive, fname, flags, ptr::addr_of(&sb)) };
        if sb_res == -1 {
            None
        } else {
            Some(ZipStat {stat: sb})
        }
    }

}

pub struct ZipFile {
    priv file: *zip_file,
    priv mut open: bool,

    drop {
        if self.open {
            zip_fclose(self.file);
            self.open = false;
        }
    }
}

impl ZipFile: io::Reader {
    pub fn read(bytes: &[mut u8], len: uint) -> uint {
        if !self.open {
            0
        } else {
            do vec::as_mut_buf(bytes) |buf_p, buf_len| {
                assert buf_len >= len;
                let count = zip_fread(self.file, buf_p as *c_void, len as uint64_t);
                if count <= 0 { self.open = false; }
                count as uint
            }
        }
    }

    pub fn read_byte() -> int {
        let bytes: ~[mut u8] = ~[mut];
        let count = self.read(bytes, 1);
        if count != 1 {
            EOF
        } else {
            bytes[0] as int
        }
    }

    pub fn unread_byte(_byte: int) { fail ~"Unsupported" }

    pub fn eof() -> bool { !self.open }

    pub fn seek(_offset: int, _whence: io::SeekStyle) { fail ~"Unsupported" }

    pub fn tell() -> uint { fail ~"Unsupported" }
}

pub struct ZipStat {
    priv stat: zip_stat_struct
}

macro_rules! get_stat(
    ($field:ident $flag:ident $conv:ty) => (
        if self.stat.valid & $flag as uint64_t != 0 {
            Some(self.stat.$field as $conv)
        } else {
            None
        }
    )
)

impl ZipStat {
    pub fn name() -> Option<~str> { 
        match get_stat!(name ZIP_STAT_NAME *c_char) {
            Some(x) => Some(unsafe { str::raw::from_c_str(x) }),
            None => None
        }
    }
    pub fn index() -> Option<u64> { get_stat!(index ZIP_STAT_INDEX u64) }
    pub fn size() -> Option<u64> { get_stat!(size ZIP_STAT_SIZE u64) }
    pub fn comp_size() -> Option<u64> { get_stat!(comp_size ZIP_STAT_COMP_SIZE u64) }
    pub fn mod_time() -> Option<i64> { get_stat!(mtime ZIP_STAT_MTIME i64) }
    pub fn crc() -> Option<u32> { get_stat!(index ZIP_STAT_CRC u32) }
    pub fn comp_method() -> Option<u16> { get_stat!(index ZIP_STAT_COMP_METHOD u16) }
    pub fn enc_method() -> Option<u16> { get_stat!(index ZIP_STAT_ENCRYPTION_METHOD u16) }
}

#[cfg(test)]
mod tests {
    fn get_archive() -> ZipArchive {
        let archive_res = open_archive(~"test_resources/test.zip", 0);
        result::unwrap(archive_res)
    }

    #[test]
    fn locate() {
        let archive = get_archive();
        assert option::unwrap(archive.locate(~"test2", ZIP_FL_NODIR)) == 2;
    }

    #[test]
    fn simple_read() {
        let archive = get_archive();
        let file = option::unwrap(archive.open_name(~"test", 0));
        let bytes = (file as io::ReaderUtil).read_whole_stream();
        assert bytes.len() == 5;
        assert str::from_bytes(bytes).trim() == ~"test";
    }

    #[test]
    fn simple_stat() {
        let archive = get_archive();
        let stat = option::unwrap(archive.stat_name(~"test", 0));
        assert option::unwrap(stat.name()) == ~"test";
        assert option::unwrap(stat.index()) == 0;
        assert option::unwrap(stat.size()) == 5;
        assert option::unwrap(stat.comp_size()) == 5;
        assert option::unwrap(stat.mod_time()) == 1065473202;
        assert option::unwrap(stat.comp_method()) == ZIP_CM_STORE as u16;
        assert option::unwrap(stat.enc_method()) == ZIP_EM_NONE as u16;
    }
}
