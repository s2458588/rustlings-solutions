// TODO: Fix the compiler error about calling a private function.
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        println!("print secret receipe");
        String::from("Ginger")
    }

    pub fn make_sausage() {
        let sr = get_secret_recipe();
        println!("{}", sr);
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
