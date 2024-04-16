pub fn scanner(file_name: &String)
{
	println!("{}", file_name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner() {
        let file_name: String = String::from("test.txt");
        assert_eq!(scanner(&file_name), ());
    }
}
