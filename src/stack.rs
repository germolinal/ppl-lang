use std::ops::{Index, IndexMut};


pub struct Stack<T> where T : Copy{
    data : [T; 1024],
    top: usize
}

impl <T>Stack<T> where T: Copy {
    pub fn new(v: T) -> Self {
        Stack{
            data: [v; 1024],
            top: 0
        }
    }

    pub fn push(&mut self, v: T){
        self.data[self.top]=v;
        self.top+=1;
    }

    pub fn pop(&mut self)->Option<T>{
        if self.top == 0 {
            return None;
        }
        self.top-=1;        
        Some(self.data[self.top])
    }    

    pub fn len(&self)->usize{
        self.top
    }
}

impl <T>Index<usize> for Stack<T> where T: Copy {
    type Output = T;

    fn index(&self, i : usize ) -> &Self::Output {
        &self.data[i]
    }
}

impl <T>IndexMut<usize> for Stack<T> where T: Copy {
    fn index_mut(&mut self, i : usize ) -> &mut Self::Output {
        &mut self.data[i]
    }
}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]    
    fn test_pop_empty(){
        let mut s = Stack::new(0);
        assert!(s.pop().is_none());
    }

}// End of tests