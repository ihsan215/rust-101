#[derive(Debug)]
struct Reactangle {
    width:u32,
    height:u32,
}

impl Reactangle {
    fn can_hold(&self, other:&Reactangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller(){

        let larger = Reactangle {
            width: 8,
            height:7,
        };

        let smaller = Reactangle {
            width: 5,
            height:1
        };

        
        assert!(larger.can_hold(&smaller));
    
    }
}
