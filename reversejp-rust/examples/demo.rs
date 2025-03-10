use reversejp::ReverseJp;

fn main() {
    let reverse_jp = ReverseJp::with_embedded_data().unwrap();
    let props = reverse_jp.find_properties(139.7670, 35.6812);

    for prop in props {
        println!(
            "Code: {}, Name: {}, English Name: {}",
            prop.code, prop.name, prop.en_name
        );
    }
}
