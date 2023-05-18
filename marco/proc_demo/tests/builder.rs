use test_macro_proc::Builder;

// cargo expand
#[allow(dead_code)]
#[derive(Builder, Default)]
pub struct Command {
    executable: String,
    // #[builder(each = "arg")]
    args: Vec<String>,
    current_dir: String,
}

#[test]
pub fn test_builder() {
    println!("builder: {:?}", Command::builder())
}
