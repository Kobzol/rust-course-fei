enum PrintType {
    Text,
    MsgPack,
    Json { indent: u32 },
}

fn print_data(a: u32, print_type: PrintType) {}

fn main() {
    print_data(1, PrintType::Json { indent: 2 });
}
