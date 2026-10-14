pub mod np;

use pyo3::prelude::*;


#[pymodule]
mod pylib {

    #[pymodule_export]
    use super::np::np;
    use pyo3::prelude::*;
    use std::mem;
    use std::rc::Rc;
    use std::cell::RefCell;


    #[pyclass]
    struct SLL{
        data:i32,
        next:Option<Box<SLL>>
    }
    #[pymethods]
    impl SLL{
        #[new]
        fn new(data:i32)->Self{ //creating the SLL
            SLL{
                data,
                next:None
            }
        }

        fn ins_f(&mut self,data:i32){  //insert to beginning
            let old_next = self.next.take();
            let old_data = mem::replace(&mut self.data, data);
            let nn = Box::new(SLL{
                data:old_data,
                next:old_next
            });
            self.next = Some(nn);
        }
        fn ins_l(&mut self,data:i32){ //insert at end
            let mut n = self;
            while n.next.is_some(){
                n = n.next.as_deref_mut().unwrap();
            }
            let nn = Box::new(SLL::new(data));
            n.next = Some(nn);
        }

        fn del_f(&mut self){ //del first
            match self.next.take(){
                Some(node) => {
                    let _ = mem::replace(&mut self.data, node.data);
                    self.next = node.next;

                }
                None => return
            }
        }
        fn del_l(&mut self){//del last
        let mut t = self;
        while t.next.as_deref().map_or(false, |node| node.next.is_some()){
        t = t.next.as_deref_mut().unwrap();
        }
        t.next = None;
        }

        fn __repr__(&self) -> String { //use print() in py to print the SLL
            let mut str:String = String::new();
            let mut t = self;
            loop{
                str.push_str(&format!("{}", t.data));
                match &t.next{
                    Some(next) => {
                        str.push_str("->");
                        t = next;
                    }
                    None => break
                }

            }
            str


        }


    }


    // #[pyclass]
    // struct Bst{
    //     data:Rc<RefCell<i32>>
    // }

}
