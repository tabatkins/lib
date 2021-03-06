// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use fmt::{Debug, Write};
use cty::{c_ulong};

/// A filesystem type.
///
/// [field, 1]
/// The constant representing the filesystem type.
#[derive(Pod, Eq)]
pub struct FileSystem(pub c_ulong);

macro_rules! create {
    ($($name:ident = ($val:expr, $str:expr),)*) => {
        $(
            #[doc = $str]
            pub const $name: FileSystem = FileSystem($val);
        )*

        impl Debug for FileSystem {
            fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
                let s = match *self {
                    $($name => $str,)*
                    x => return write!(w, "Unknown(0x{:X})", x.0),
                };
                w.write_all(s.as_bytes()).ignore_ok()
            }
        }
    }
}

create! {
    ADFS            = (0xADF5,     "ADFS"),
    AFFS            = (0xADFF,     "AFFS"),
    AFS             = (0x5346414F, "AFS"),
    ANON_INODE_FS   = (0x09041934, "ANON_INODE_FS"),
    AUFS            = (0x61756673, "AUFS"),
    AUTOFS          = (0x0187,     "AUTOFS"),
    BEFS            = (0x42465331, "BEFS"),
    BDEVFS          = (0x62646576, "BDEVFS"),
    BFS             = (0x1BADFACE, "BFS"),
    BINFMTFS        = (0x42494E4D, "BINFMTFS"),
    BTRFS           = (0x9123683E, "Btrfs"),
    CEPH            = (0x00C36400, "CEPH"),
    CGROUP          = (0x0027E0EB, "CGROUP"),
    CIFS            = (0xFF534D42, "CIFS"),
    CODA            = (0x73757245, "CODA"),
    COH             = (0x012FF7B7, "COH"),
    CONFIGFS        = (0x62656570, "CONFIGFS"),
    CRAMFS          = (0x28CD3D45, "CRAMFS"),
    CRAMFS_WEND     = (0x453DCD28, "CRAMFS_WEND"),
    DEBUGFS         = (0x64626720, "DEBUGFS"),
    DEVFS           = (0x1373,     "devfs"),
    DEVPTS          = (0x1CD1,     "DEVPTS"),
    ECRYPTFS        = (0xF15F,     "ECRYPTFS"),
    EFIVARFS        = (0xDE5E81E4, "EFIVARFS"),
    EFS             = (0x00414A53, "EFS"),
    EXOFS           = (0x5DF5,     "EXOFS"),
    EXT             = (0x137D,     "EXT"),
    EXT2            = (0xEF53,     "ext2"),
    EXT2_OLD        = (0xEF51,     "EXT2_OLD"),
    F2FS            = (0xF2F52010, "F2FS"),
    FAT             = (0x4006,     "FAT"),
    FHGFS           = (0x19830326, "FHGFS"),
    FUSEBLK         = (0x65735546, "FUSEBLK"),
    FUSECTL         = (0x65735543, "FUSECTL"),
    FUTEXFS         = (0x0BAD1DEA, "FUTEXFS"),
    GFS             = (0x01161970, "GFS"),
    GPFS            = (0x47504653, "GPFS"),
    HFS             = (0x4244,     "HFS"),
    HFS_PLUS        = (0x482B,     "HFS_PLUS"),
    HFS_X           = (0x4858,     "HFS_X"),
    HOSTFS          = (0x00C0FFEE, "HOSTFS"),
    HPFS            = (0xF995E849, "HPFS"),
    HUGETLBFS       = (0x958458F6, "HUGETLBFS"),
    MTD_INODE_FS    = (0x11307854, "MTD_INODE_FS"),
    INOTIFYFS       = (0x2BAD1DEA, "INOTIFYFS"),
    ISOFS           = (0x9660,     "ISOFS"),
    ISOFS_R_WIN     = (0x4004,     "ISOFS_R_WIN"),
    ISOFS_WIN       = (0x4000,     "ISOFS_WIN"),
    JFFS            = (0x07C0,     "JFFS"),
    JFFS2           = (0x72B6,     "JFFS2"),
    JFS             = (0x3153464A, "JFS"),
    KAFS            = (0x6B414653, "KAFS"),
    LOGFS           = (0xC97E8168, "LOGFS"),
    LUSTRE          = (0x0BD00BD0, "LUSTRE"),
    MINIX           = (0x137F,     "MINIX"),
    MINIX_30        = (0x138F,     "MINIX_30"),
    MINIX_V2        = (0x2468,     "MINIX_V2"),
    MINIX_V2_30     = (0x2478,     "MINIX_V2_30"),
    MINIX_V3        = (0x4D5A,     "MINIX_V3"),
    MQUEUE          = (0x19800202, "MQUEUE"),
    MSDOS           = (0x4D44,     "MSDOS"),
    NCP             = (0x564C,     "NCP"),
    NFS             = (0x6969,     "NFS"),
    NFSD            = (0x6E667364, "NFSD"),
    NILFS           = (0x3434,     "NILFS"),
    NTFS            = (0x5346544E, "NTFS"),
    OPENPROM        = (0x9FA1,     "OPENPROM"),
    OCFS2           = (0x7461636F, "OCFS2"),
    PANFS           = (0xAAD7AAEA, "PANFS"),
    PIPEFS          = (0x50495045, "PIPEFS"),
    PROC            = (0x9FA0,     "proc"),
    PSTOREFS        = (0x6165676C, "PSTOREFS"),
    QNX4            = (0x002F,     "QNX4"),
    QNX6            = (0x68191122, "QNX6"),
    RAMFS           = (0x858458F6, "RAMFS"),
    REISERFS        = (0x52654973, "ReiserFS"),
    ROMFS           = (0x7275,     "ROMFS"),
    RPC_PIPEFS      = (0x67596969, "RPC_PIPEFS"),
    SECURITYFS      = (0x73636673, "SECURITYFS"),
    SELINUX         = (0xF97CFF8C, "SELINUX"),
    SMACK           = (0x43415D53, "SMACK"),
    SMB             = (0x517B,     "SMB"),
    SNFS            = (0xBEEFDEAD, "SNFS"),
    SOCKFS          = (0x534F434B, "SOCKFS"),
    SQUASHFS        = (0x73717368, "SQUASHFS"),
    SYSFS           = (0x62656572, "sysfs"),
    SYSV2           = (0x012FF7B6, "SYSV2"),
    SYSV4           = (0x012FF7B5, "SYSV4"),
    TMPFS           = (0x01021994, "tmpfs"),
    UBIFS           = (0x24051905, "UBIFS"),
    UDF             = (0x15013346, "UDF"),
    UFS             = (0x00011954, "UFS"),
    UFS_BYTESWAPPED = (0x54190100, "UFS_BYTESWAPPED"),
    USBDEVFS        = (0x9FA2,     "USBDEVFS"),
    V9FS            = (0x01021997, "V9FS"),
    VMHGFS          = (0xBACBACBC, "VMHGFS"),
    VXFS            = (0xA501FCF5, "VXFS"),
    VZFS            = (0x565A4653, "VZFS"),
    XENFS           = (0xABBA1974, "XENFS"),
    XENIX           = (0x012FF7B4, "XENIX"),
    XFS             = (0x58465342, "XFS"),
    XIAFS           = (0x012FD16D, "XIAFS"),
    ZFS             = (0x2FC12FC1, "ZFS"),
}
