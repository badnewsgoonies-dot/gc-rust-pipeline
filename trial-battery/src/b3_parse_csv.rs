pub fn parse_csv_simple(input: &str) -> Vec<Vec<String>> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split(',').map(|field| field.trim().to_string()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_csv_simple;

    #[test]
    fn simple_csv() {
        let r = parse_csv_simple("a,b,c\n1,2,3\n");
        assert_eq!(
            r,
            vec![
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                vec!["1".to_string(), "2".to_string(), "3".to_string()],
            ]
        );
    }

    #[test]
    fn empty_lines_skipped() {
        let r = parse_csv_simple("a,b\n\nc,d\n");
        assert_eq!(r.len(), 2);
        assert_eq!(r[1], vec!["c".to_string(), "d".to_string()]);
    }

    #[test]
    fn whitespace_trimmed() {
        let r = parse_csv_simple("  x , y  ,  z  ");
        assert_eq!(
            r,
            vec![vec!["x".to_string(), "y".to_string(), "z".to_string()]]
        );
    }

    #[test]
    fn empty_input() {
        let r = parse_csv_simple("");
        assert!(r.is_empty());
    }
}
