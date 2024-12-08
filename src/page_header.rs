enum PageType {
    InteriorIndex,
    InteriorTable,
    LeafIndex,
    LeafTable,
}

impl From<u8> for PageType {
    fn from(flag: u8) -> Self {
        match flag {
            2 => PageType::InteriorIndex,
            5 => PageType::InteriorTable,
            10 => PageType::LeafIndex,
            13 => PageType::LeafTable,
            _ => panic!("Unknown page type: {}", flag),
        }
    }
}
pub struct PageHeader {
    page_type: PageType,
    first_free_block_offset: u16,
    pub cell_count: u16,
    cell_offset: u16,
    free_bytes: u8,
    right_most_pointer: Option<u32> // Only interior pages have a right-most pointer, that is to say leaf pages do not.
}

impl From<&[u8]> for PageHeader {
    fn from(data: &[u8]) -> Self {
        let page_type = PageType::from(data[0]);
        let right_most_pointer  = match page_type {
            PageType::InteriorIndex | PageType::InteriorTable => Some(u32::from_be_bytes(data[8..12].try_into().unwrap())),
            _ => None,
        };
        PageHeader {
            page_type: page_type,
            first_free_block_offset: u16::from_be_bytes([data[1], data[2]]),
            cell_count: u16::from_be_bytes([data[3], data[4]]),
            cell_offset: u16::from_be_bytes([data[5], data[7]]),
            free_bytes: data[7],
            right_most_pointer: right_most_pointer.map(u32::from),
        }
    }
}