use std::io::Bytes;

struct Directory {
    page_paths: Vec<(String, String)> // (page_id, FilePath)
}

struct Page {
    // 4KB
    header: PageHeader,
    tuple_num: usize,
    data: Data,
}

struct PageHeader {
    id: usize,
    page_size: usize,
    check_sum: String,
    version: usize,
    transaction_visibility: TransactionState,
    compression_form: Option<CompressionForm>,
}

struct Data {
    slot_array: Vec<usize>,
    tuples: Vec<Tuple>,
}

struct OverflowPage {
    // 2KB
    data: Bytes<u8>
}

struct Blob {
    file_path: String
}

struct Tuple();

struct TupleHeader {
    id: usize,
    // page_id + offset / slot
    transaction_visibility: TransactionState,
}


enum TransactionState {}

enum CompressionForm {}