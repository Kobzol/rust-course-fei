enum Document {
    Text(String),
    Image(Vec<u8>),
}

struct Printer(u32);

struct PrintJob {
    id: u32,
    start: u32,
    document: Document,
    state: JobState,
}
enum JobState {
    Queued,
    Running { printer: Printer },
    Finished { printed_pages: u32 },
    Error { error: String },
}

fn main() {}
