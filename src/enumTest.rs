// This file contains the code for the enum test
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}
impl FileSize {
    fn format_size(&self) -> String {
        match self {
            Self::Bytes(bytes) => format!("{} bytes", bytes),
            Self::Kilobytes(kilobytes) => format!("{} KB", kilobytes),
            Self::Megabytes(megabytes) => format!("{} MB", megabytes),
            Self::Gigabytes(gigabytes) => format!("{} GB", gigabytes),
        }
    }
}

pub fn test() {
    let size = FileSize::Kilobytes(1024);
    println!("Size: {}", size.format_size());
}



