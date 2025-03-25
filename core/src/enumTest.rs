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

/// 
/// This function is used to test the enum
/// Enum is a type that can have a few values.
/// Every enum value is called a variant.
/// Variants can store data.
/// 
pub fn test() {
    let size = FileSize::Kilobytes(1024);
    println!("Size: {}", size.format_size());
}



