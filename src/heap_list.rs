
use crate::value_trait::ValueTrait;
use crate::function::Function;
use crate::token::Token;

struct Element {
    pub value: Box<dyn ValueTrait>,
    reachable: bool,
}

pub struct HeapList {
    elements: [Option<Element>; u8::MAX as usize],
    n_elements: u8,
    first_free : u8,
}



impl HeapList {

    pub fn new()->Self{     
        // A copyable None element
        const NONE: Option<Element> = None;   
        Self{
            first_free: 0,
            n_elements: 0,            
            elements: [NONE; u8::MAX as usize],
        }
    }    

    /// Returns the number of elements in the HeapList
    pub fn len(&self)->u8{
        self.n_elements
    }

    /// Checks if len() is zero
    pub fn is_empty(&self)->bool{
        self.n_elements == 0
    }

    /// Borrows an element from the HeapList
    pub fn get(&self, i: u8)->Option<&Box<dyn ValueTrait>>{
        if self.elements.len() > i as usize {
            match &self.elements[i as usize]{
                None => None,
                Some(e)=>Some(&e.value)
            }            
        }else{
            None
        }
    }

    /// Borrows a mutable element from the HeapList
    pub fn get_mut(&mut self, i: u8)->Option<&mut Box<dyn ValueTrait>>{
        if self.elements.len() > i as usize {
            match &mut self.elements[i as usize]{
                None => None,
                Some(e)=>Some(&mut e.value)
            }            
        }else{
            None
        }
    }

    /// Sets n element in the HeapList
    pub fn set(&mut self, i: u8, value: Box<dyn ValueTrait>)->Result<(),String>{
        if self.elements.len() > i as usize {
            
            self.elements[i as usize] = Some(Element {
                value,
                reachable: true, // This is irrelevant... it will be erased during Garbage Collection
            });
            Ok(())
        }else{
            Err(format!("Element out of bounds in HeapList. Setting element {} but there are only {}", i, self.elements.len()))           
            
        }
    }

    

    /// Marks an element as reachable.        
    pub fn mark_as_reachable(&mut self, i: u8) {        
        if self.elements.len() > i as usize {
            match &mut self.elements[i as usize]{
                None => panic!("Trying to mark_as_reachable() to 'None' element in HeapList... element {}",i),
                Some(e)=> {
                    e.reachable=true;
                    /*
                    let v = &e.value;
                    // Propagate
                    if v.is_object(){
                        self.mark_object_as_reachable(v);
                    }else if v.is_array(){
                        self.mark_array_as_reachable(v);
                    }else if v.is_function(){
                        self.mark_function_as_reachable(v);
                    }else if v.is_number() || v.is_bool() || v.is_nil() {
                        return
                    }else{
                        panic!("Marking '{}' has not been implemented", v.type_name());
                    }                    
                    */
                }
            }
                        
        }else{
            panic!("Trying to mark_as_reachable() to element out of bounds in HeapList... index was {}, length is {}", i, self.len())
        }
    }

    /// Adds a new element at the first_free element in the the HeapList. 
    pub fn push(&mut self, v: Box<dyn ValueTrait>) -> u8 {        
                
        // Check if the heap is full
        if self.n_elements == u8::MAX {
            panic!("The max number of elements in the heap of a Chunk ({}) has been exceeded", self.elements.len());
        }

        // In debug mode, check that the element that will be replaced
        // is None... otherwise, panic
        debug_assert!(self.elements[self.first_free as usize].is_none());

        // If it is not full, and we are replacing a None element, 
        // then push.
        self.elements[self.first_free as usize] = Some(Element{
            reachable: true, // This is irrelevant... it is changed during Garbage Collection
            value: v
        });

        // Take note of the location of the inserted element
        let ret = self.first_free; 

        // Increase count
        self.n_elements += 1;
        
        // Update next free
        let end = self.elements.len() as u8;
        for i in ret..end {
            if self.elements[i as usize].is_none() {
                self.first_free = i;
                break;
            }
        }

        ret

    }

    /// Returns the index of the function in the hap with the corresponding name
    pub fn get_global_function<'a>(&self, fn_name_token: &Token<'a>)->Option<u8>{
        let fn_name = fn_name_token.source_text();
        for i in 0..self.elements.len(){
            let element = &self.elements[i];

            if let Some(e) = element{                
                let v = &e.value;
                if v.is_function(){
                    let function = match v.as_any()
                        .downcast_ref::<Function>(){
                            Some(f)=>f.clone_rc(),
                            None => panic!("Not sure what happened... but it was on HeapList, trying to get a global function")
                        };

                    if function.get_name() == fn_name {
                        return Some(i as u8);
                    }
                }
            }
                
        }
        None
    }
    
}





/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::number::Number;

    

    #[test]
    fn test_new(){
        let heap = HeapList::new();
        assert_eq!(heap.first_free, 0);
        assert_eq!(heap.n_elements, 0);
    }

    #[test]
    fn test_push_get(){
        let mut heap = HeapList::new();
        assert_eq!(heap.len(),0);

        let i = heap.push(Box::new(12.0));
        assert_eq!(i, 0);        
        assert_eq!(heap.first_free, 1);
        assert_eq!(heap.len(),1);

        let i = heap.push(Box::new(32.0));
        assert_eq!(i, 1);        
        assert_eq!(heap.first_free, 2);
        assert_eq!(heap.len(),2);

        let i = heap.push(Box::new(39.0));
        assert_eq!(i, 2);        
        assert_eq!(heap.first_free, 3);
        assert_eq!(heap.len(),3);

        let i1 = heap.get(1).unwrap();        
        assert_eq!(i1.to_string(), format!("32"));
        assert_eq!(heap.len(),3);
    }

    
}
