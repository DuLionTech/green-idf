fn main() {
    for (name, value) in dotenv::vars() {
        println!("cargo:rustc-env={}={}", name, value);
    }
    embuild::espidf::sysenv::output();
}
