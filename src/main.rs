use to_enum::txt_to_enum;

pub fn main() {
    txt_to_enum!("test.txt");
    let _x = Barks::Hallo;
}