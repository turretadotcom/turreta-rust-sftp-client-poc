#[no_mangle]
pub extern "C" fn process(file: String) -> bool {

    println!("Processing file {:?}", file);

    true
}
