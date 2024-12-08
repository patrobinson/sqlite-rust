use crate::page_header::PageHeader;
use byteorder::{BigEndian, ByteOrder};

enum FileFormat {
    Legacy,
    WAL,
}

impl TryFrom<&u8> for FileFormat {
    type Error = ();
    fn try_from(value: &u8) -> Result<Self, ()> {
        match value {
            1 => Ok(FileFormat::Legacy),
            2 => Ok(FileFormat::WAL),
            _ => Err(()),
        }
    }
}

enum SchemaFormat {
    One,
    Two,
    Three,
    Four,
}

impl TryFrom<u32> for SchemaFormat {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, ()> {
        match value {
            1 => Ok(SchemaFormat::One),
            2 => Ok(SchemaFormat::Two),
            3 => Ok(SchemaFormat::Three),
            4 => Ok(SchemaFormat::Four),
            _ => Err(()),
        }
    }
}

enum EncodingFormat {
    UTF8,
    UTF16le,
    UTF16be,
}

impl TryFrom<u32> for EncodingFormat {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, ()> {
        match value {
            1 => Ok(EncodingFormat::UTF8),
            2 => Ok(EncodingFormat::UTF16le),
            3 => Ok(EncodingFormat::UTF16be),
            _ => Err(()),
        }
    }
}

const MAGIC_HEADER: &str = "SQLite format 3\0";

// https://www.sqlite.org/fileformat.html
pub struct DatabaseHeader {
    pub page_size: u16,
    file_write_format: FileFormat,
    file_read_format: FileFormat,
    reserved_space: u8,
    max_embedded_payload_fraction: u8,
    min_embedded_payload_fraction: u8,
    leaf_payload_fraction: u8,
    file_change_counter: u32,
    pages_count: u32,
    first_freelist_page_number: u32,
    number_freelist_pages: u32,
    schema_cookie: u32,
    schema_format: SchemaFormat,
    page_cache_size: u32,
    largest_root_page_number: u32,
    encoding: EncodingFormat,
    user_version: u32,
    incremental_vacuum: bool,
    application_id: u32,
    version_valid_for: u32,
    sqlite_version_number: u32,
}

impl TryFrom<&[u8; 100]> for DatabaseHeader {
    type Error = ();
    fn try_from(data: &[u8; 100]) -> Result<Self, ()> {
        if data[0..16] != MAGIC_HEADER.as_bytes()[0..16] {
            return Err(());
        }
        Ok(DatabaseHeader {
            page_size: BigEndian::read_u16(&data[16..18]),
            file_write_format: FileFormat::try_from(&data[18])?,
            file_read_format: FileFormat::try_from(&data[19])?,
            reserved_space: data[20],
            max_embedded_payload_fraction: data[21],
            min_embedded_payload_fraction: data[22],
            leaf_payload_fraction: data[23],
            file_change_counter: BigEndian::read_u32(&data[24..28]),
            pages_count: BigEndian::read_u32(&data[28..32]),
            first_freelist_page_number: BigEndian::read_u32(&data[32..36]),
            number_freelist_pages: BigEndian::read_u32(&data[36..40]),
            schema_cookie: BigEndian::read_u32(&data[40..44]),
            schema_format: SchemaFormat::try_from(BigEndian::read_u32(&data[44..48]))?,
            page_cache_size: BigEndian::read_u32(&data[48..52]),
            largest_root_page_number: BigEndian::read_u32(&data[52..56]),
            encoding: EncodingFormat::try_from(BigEndian::read_u32(&data[56..60]))?,
            user_version: BigEndian::read_u32(&data[60..64]),
            incremental_vacuum: match BigEndian::read_u32(&data[64..68]) {
                0 => false,
                _ => true,
            },
            application_id: BigEndian::read_u32(&data[68..72]),
            version_valid_for: BigEndian::read_u32(&data[92..96]),
            sqlite_version_number: BigEndian::read_u32(&data[96..100]),
        })
    }
}
