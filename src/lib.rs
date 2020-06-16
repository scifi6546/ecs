/// What is Left:
/// InterEntity Communication
use std::collections::HashMap;
///Usage:
/// create entity with create_entity.
/// ```
/// create_entity!(BasicData,wealth:u32);
/// let mut e = Entity::new(||{0},vec![|entity,msg|{entity.wealth++}]);
/// e.process();
/// ```
macro_rules! create_entity {
    ($name:ident,$($element: ident: $ty: ty),*) => {
    
        struct $name{
            $($element: $ty),*
        }
        impl $name{
            pub fn new($($element:fn()->$ty),*)->Self{
                Self{
                    $($element: $element()),*
                }
            }
            $(pub fn $element (&self) ->$ty{self.$element.clone()}),*
            
        }
        type ID = u32;
        enum Message<Message>{
            Die,
            Message(ID,Message)
        }
        struct Entity<MessageData>{
            data: $name,
            behivor: Vec<fn(&mut $name,&mut Vec<Message<MessageData>>)->()>,
        }
        impl<MessageData> Entity<MessageData>{
            pub fn new($($element:fn()->$ty),*,behivors:Vec<fn(&mut $name,&mut Vec<Message<MessageData>>)>)->Self{
                Self{
                    data:$name::new($($element).*),
                    behivor: behivors
                }
            }
            pub fn process(&mut self)->Vec<Message<MessageData>>{
                let mut v = vec![];
                for b in self.behivor.iter(){
                    b(&mut self.data,&mut v)
                }
                return v
            }
        }
        struct EntityManager<Message>{
            elements: HashMap<ID,Entity<Message>>
        }
        impl<Message> EntityManager<Message>{
            fn get_entity(&self,id:ID)->Option<&Entity<Message>>{
                self.elements.get(&id)
            }
            ///Function to get elements in Entity With id
            $(pub fn $element(&self,id:ID)->Option<$ty>{
                let entity = self.get_entity(id);
                if entity.is_some(){
                    Some(entity.unwrap().data.$element())
                }else{
                    None
                }
                
            }),*
        }
    }
}
mod test{
    create_entity!(data,a:u32);
    use super::*;
    fn zero()->u32{
        0
    }
    #[test]
    fn new(){
        let e = data::new(||{0});
        assert_eq!(e.a(),0);
    }
    #[test]
    fn new_entity(){
        let mut e = Entity::<u32>::new(||{0},vec![|e,v|{e.a=1}]);
        e.process();
        assert_eq!(e.data.a(),1);

    }
}