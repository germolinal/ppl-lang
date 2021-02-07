use core::ptr::{self};
use std::ops::{Index, IndexMut};

/// A Stack of length 255 (i.e. i8::MAX) allocated in the Stack.
/// 
/// Its src has been inspired by Rust's Vec src code
pub struct Stack<T> {
    elements: [Option<T>;u8::MAX as usize],    
    len: u8
}

impl <T>Stack<T>{
    
    /// Creates a new Stack, full of None and length 0        
    pub fn new()->Self{
        Self{

            len: 0,
            // We need this because the elements are do not 
            // satisfy the Copy trait requirement... although this
            // should not be considered unsafe because all the elements are
            // initialized as 'None' anyway.
            elements: unsafe {
                let mut arr: [Option<T>; u8::MAX as usize] = std::mem::MaybeUninit::uninit().assume_init();//std::mem::uninitialized();
                for item in &mut arr[..] {
                    std::ptr::write(item, None);
                }
                arr
            }
        }
    }

    /// Returns the number of elements in the Stack    
    pub fn len(&self)->u8{
        self.len
    }

    

    /// Pops (and returns) the last element 
    #[inline]
    pub fn pop(&mut self)->Option<T>{
        if self.len == 0 {
            None
        }else{
            unsafe {
                self.len -= 1;
                ptr::read(self.elements.as_ptr().add(self.len as usize))
            }
        }
    }

    /// Drops the last element 
    /// 
    /// It is like Pop, but does not return anything
    #[inline]
    pub fn drop_last(&mut self)->Result<(),String>{
        if self.len == 0 {
            Err(format!("Trying to drop last element of an empty stack"))
        }else{            
            self.len -= 1;                            
            Ok(())
        }
    }

    /// Drops N elements at the end
    /// 
    /// It is like Pop, but drops several and does not return anything
    #[inline]
    pub fn drop_n(&mut self, n : u8)->Result<(),String>{
        if self.len == 0 {
            Err(format!("Trying to drop last element of an empty stack"))
        }else{            
            self.len -= n;                            
            Ok(())
        }
    }

    /// Pushes an element to the stack
    /// 
    #[inline]
    pub fn push(&mut self, value: T) -> Result<(),String>{
        // Check if it is full
        if self.len == self.elements.len() as u8 {
            return Err(format!("Stack Overflow!"));
        }

        unsafe {
            let end = self.elements.as_mut_ptr().add(self.len as usize);                        
            ptr::write(end, Some(value));
            self.len += 1;
        };

        Ok(())
    }

    /// Borrows the last element in the Stack
    pub fn last(&self)->&T{
        match &self.elements[self.len as usize - 1]{
            Some(v)=>v,
            None=>panic!("Trying to borrow last element of empty stack")
        }
    }

    pub fn last_n(&self, n: u8)->&[Option<T>]{
        let fin = self.len as usize;
        let ini = fin - n as usize;
        &self.elements[ini..fin]
    }

    


}


impl <T>Index<u8> for Stack<T> {
    type Output = T;

    fn index(&self, i: u8) -> &Self::Output {
        if i >= self.len(){
            panic!("i out of bounds when indexing Stack. i is {} while len() is {}", i, self.len());
        }
        match &self.elements[i as usize]{
            None => panic!("Trying to index an empty Stack element"),
            Some(v)=>v
        }
    }
}

impl <T>IndexMut<u8> for Stack<T> {
    fn index_mut(&mut self, i: u8) -> &mut Self::Output {
        
        if i >= self.len(){
            panic!("i out of bounds when indexing Stack. i is {} while len() is {}", i, self.len());
        }
        match &mut self.elements[i as usize]{
            None => panic!("Trying to index an empty Stack element"),
            Some(v)=>v
        }
    }
}




/* TESTS */

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_new(){
        let stack : Stack<u8> = Stack::new();
        assert_eq!(stack.len(), 0);

        for i in 0..u8::MAX{
            assert!(stack.elements[i as usize].is_none());
        }
    }

    #[test]
    fn test_pop_empty(){
        let mut stack : Stack<u8> = Stack::new();
        assert!(stack.pop().is_none());
        assert!(stack.drop_last().is_err());
    }

    #[test]
    fn test_stack_overflow(){
        let mut stack : Stack<usize> = Stack::new();        
        for i in 0..u8::MAX{
            stack.push(i as usize).unwrap();
        }

        for i in 0..u8::MAX{
            assert_eq!(stack.elements[i as usize], Some(i as usize));
        }


        assert!(stack.push(0).is_err());
    }


    #[test]
    fn test_push_pop(){
        let n = 132;

        let mut stack : Stack<usize> = Stack::new();

        // Push n
        for i in 0..n{
            stack.push(i as usize).unwrap();
        }       

        // Check the value in the first N
        for i in 0..n{
            assert_eq!(stack.elements[i as usize], Some(i as usize));
        }
        // Check that the rest are None
        for i in n..u8::MAX{
            assert!(stack.elements[i as usize].is_none());
        }

        // Pop last
        assert_eq!(stack.len(), n);
        assert_eq!(stack.pop(), Some( (n-1)as usize));        
        assert_eq!(stack.len(), n-1);

        
    }


    #[test]
    fn test_push_drop(){
        let n = 132;

        let mut stack : Stack<usize> = Stack::new();
        for i in 0..n{
            stack.push(i as usize).unwrap();
        }       

        for i in 0..n{
            assert_eq!(stack.elements[i as usize], Some(i as usize));
        }
        for i in n..u8::MAX{
            assert!(stack.elements[i as usize].is_none());
        }

        // Drop last
        assert_eq!(stack.len(), n);
        stack.drop_last().unwrap();        
        assert_eq!(stack.len(), n-1);

        // Drop N
        stack.drop_n(7).unwrap();        
        assert_eq!(stack.len(), n-8);


    }



}