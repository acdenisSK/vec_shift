pub trait Shift<T> {
    fn shift(&mut self) -> Option<T>;
}

impl<T> Shift<T> for Vec<T> {
    fn shift(&mut self) -> Option<T> {
        if !self.is_empty() {
            Some(self.remove(0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shift() {
        use ::Shift;
        
        let mut vec = vec![1, 2, 3];
        assert_eq!(vec.len(), 3);
        let value = vec.shift().unwrap();
        assert_eq!(value, 1);
        assert_eq!(vec.len(), 2);
    }
}
