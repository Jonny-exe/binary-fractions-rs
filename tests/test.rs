#[path = "../src/binary.rs"]
mod binary;

// All the *integration* tests go here. 
// The *unit* tests are inside the corresponding files.
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[test]
    fn test_Binary() {
        assert!(true);
    }
}
