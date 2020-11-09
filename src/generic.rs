use crate::value_trait::ValueTrait;
use std::rc::Rc;

pub trait Generic : ValueTrait {

    fn clone_heap(&self)->Rc<dyn Generic>;

}