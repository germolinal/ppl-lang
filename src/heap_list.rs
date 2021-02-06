
use crate::value_trait::ValueTrait;
use crate::function::Function;
use crate::token::Token;

struct Element {
    pub value: Box<dyn ValueTrait>,
    pub n_refs: u8,
}

pub struct HeapList {
    elements: [Option<Element>; u8::MAX as usize],
    n_elements: u8,
    first_free : u8,
}



impl HeapList {

    pub fn new()->Self{        
        Self{
            first_free: 0,
            n_elements: 0,

            // We need this because the elements are do not 
            // satisfy the Copy trait requirement... although this
            // should not be considered unsafe because all the elements are
            // initialized as 'None' anyway.
            elements: unsafe {
                let mut arr: [Option<Element>; u8::MAX as usize] = std::mem::MaybeUninit::uninit().assume_init();//std::mem::uninitialized();
                for item in &mut arr[..] {
                    std::ptr::write(item, None);
                }
                arr
            },
        }
    }    

    /// Returns the number of elements in the 
    pub fn len(&self)->u8{
        self.n_elements
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

    /// Sets n element in the HeapList
    pub fn set(&mut self, i: u8, value: Box<dyn ValueTrait>)->Result<(),String>{
        if self.elements.len() > i as usize {
            let old_refs = match &self.elements[i as usize]{
                Some(v)=>v.n_refs,
                None => 0
            };

            self.elements[i as usize] = Some(Element {
                value: value,
                n_refs: old_refs,
            });
            Ok(())
        }else{
            return Err(format!("Element out of bounds in HeapList. Setting element {} but there are only {}", i, self.elements.len()))            
            
        }
    }

    /// Adds a reference to the element.
    pub fn add_reference(&mut self, i: u8) {
        if self.elements.len() as u8 > i {
            match &mut self.elements[i as usize]{
                None => panic!("Trying to add_reference() to 'None' element in HeapStack... element {}",i),
                Some(e)=> e.n_refs += 1 
            }
        }else{
            panic!("Trying to add_reference() to element out of bounds in HeapList... index was {}, length is {}", i, self.len())
        }
    }

    /// Removes a reference to an element in the HeapList.
    /// 
    /// If the number of references becomes Zero, the element
    /// is dropped
    pub fn drop_reference(&mut self, i: u8) {
        if self.elements.len() > i as usize {
            match &mut self.elements[i as usize]{
                None => panic!("Trying to drop_reference() to 'None' element in HeapStack... element {}",i),
                Some(e)=> {                    
                    e.n_refs -= 1;
                    // If references to this object are now Zero, drop it
                    if e.n_refs == 0 {
                        drop(self.elements[i as usize].take());
                        self.n_elements -= 1;
                        // Take note that this is now free.
                        if i < self.first_free {
                            self.first_free = i;
                        }
                    }
                }
            }
        }else{
            panic!("Trying to drop_reference() to element out of bounds in HeapList... index was {}, length is {}", i, self.len())
        }
    }

    /// Adds a new element at the first_free element in the the HeapList. 
    pub fn push(&mut self, v: Box<dyn ValueTrait>) -> u8 {        
                
        // Check if the heap is full
        if self.n_elements >= u8::MAX {
            panic!("The max number of elements in the heap of a Chunk ({}) has been exceeded", self.elements.len());
        }

        // In debug mode, check that the element that will be replaced
        // is None... otherwise, panic
        debug_assert!(self.elements[self.first_free as usize].is_none());

        // If it is not full, and we are replacing a None element, 
        // then push.
        self.elements[self.first_free as usize] = Some(Element{
            n_refs: 0,
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

        return ret;

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

    impl HeapList {
        pub fn n_refs(&self, i: u8)->Option<u8>{
            match self.elements.get(i as usize){
                None => panic!("Trying to get n_refs from element out of bounds in HeapList... index was {}, length is {}", i, self.len()),
                Some(e) => match e{
                    Some(v)=> Some(v.n_refs),
                    None => None
                }
            }
        }
    }

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
        assert_eq!(heap.n_refs(i).unwrap(), 0 as u8);
        assert_eq!(heap.first_free, 1);
        assert_eq!(heap.len(),1);

        let i = heap.push(Box::new(32.0));
        assert_eq!(i, 1);
        assert_eq!(heap.n_refs(i).unwrap(), 0 as u8);
        assert_eq!(heap.first_free, 2);
        assert_eq!(heap.len(),2);

        let i = heap.push(Box::new(39.0));
        assert_eq!(i, 2);
        assert_eq!(heap.n_refs(i).unwrap(), 0 as u8);
        assert_eq!(heap.first_free, 3);
        assert_eq!(heap.len(),3);

        let i1 = heap.get(1).unwrap();        
        assert_eq!(i1.to_string(), format!("32"));
        assert_eq!(heap.len(),3);
    }

    #[test]
    fn test_references(){
        // Create list
        let mut heap = HeapList::new();
        assert_eq!(heap.len(),0);

        // Adda bunch of elements
        let i = heap.push(Box::new(12.0));
        assert_eq!(i, 0);
        assert_eq!(heap.first_free, 1);
        assert_eq!(heap.n_refs(i).unwrap(), 0 as u8);
        assert_eq!(heap.len(),1);

        let i = heap.push(Box::new(32.0));
        assert_eq!(i, 1);
        assert_eq!(heap.first_free, 2);
        assert_eq!(heap.n_refs(i).unwrap(), 0 as u8);
        assert_eq!(heap.len(),2);

        let i = heap.push(Box::new(39.0));
        assert_eq!(i, 2);
        assert_eq!(heap.first_free, 3);
        assert_eq!(heap.n_refs(i).unwrap(), 0 as u8);
        assert_eq!(heap.len(),3);

        // Add references
        let i = 0;
        assert!(heap.get(i).is_some());

        heap.add_reference(i as u8);
        assert_eq!(heap.n_refs(i).unwrap(), 1 as u8);
        assert_eq!(heap.first_free, 3);
        assert_eq!(heap.len(),3);

        heap.add_reference(i as u8);
        assert_eq!(heap.n_refs(i).unwrap(), 2 as u8);
        assert_eq!(heap.first_free, 3);
        assert_eq!(heap.len(),3);

        // Drop all references in element 0
        heap.drop_reference(i);
        assert_eq!(heap.n_refs(i).unwrap(), 1 as u8);
        assert_eq!(heap.first_free, 3);
        assert_eq!(heap.len(),3);

        heap.drop_reference(i);
        assert!(heap.get(i).is_none());
        assert_eq!(heap.first_free, 0);
        assert_eq!(heap.len(),2);

        // Push some more
        let i = heap.push(Box::new(139.0));
        assert_eq!(i, 0);
        assert_eq!(heap.first_free, 3);
        assert_eq!(heap.n_refs(i).unwrap(), 0 as u8);
        assert_eq!(heap.len(),3);

        let i = heap.push(Box::new(239.0));
        assert_eq!(i, 3);
        assert_eq!(heap.first_free, 4);
        assert_eq!(heap.n_refs(i).unwrap(), 0 as u8);
        assert_eq!(heap.len(),4);
    }

}
