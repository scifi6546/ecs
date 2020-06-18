/// What is Left:
/// InterEntity Communication
use std::collections::HashMap;
use rand::Rng;
///Usage:
/// create entity with create_entity.
/// ```
/// create_entity!(BasicData,wealth:u32);
/// let mut e = Entity::new(||{0},vec![|entity,msg|{entity.wealth++}]);
/// e.process();
/// ```
macro_rules! create_entity {
    ($($element: ident: $ty: ty),*) => {
    
        struct Data{
            $($element: $ty),*
        }
        impl Data{
            pub fn new($($element:fn()->$ty),*)->Self{
                return Self{
                    $(
                        $element: $element()
                    ),*
                }
            }
            $(pub fn $element (&self) ->$ty {
                self.$element.clone()
            }
            )*
        }
        struct BehaviorComponent{
            search: fn(&Data,&EntityManager),
            update: fn(&mut Data),
        }
        impl BehaviorComponent{
            fn new(search: fn(&Data,&EntityManager), update: fn(&mut Data))->Self{
                Self{
                    search:search,
                    update:update
                }
            }

        }
        type ID = u32;
        enum Message{
            Die,
            Message(ID)
        }
        struct Entity{
            data: Data,
            behavior: Vec<BehaviorComponent>,
        }
        impl Entity{
            pub fn new($($element:fn()->$ty),*,behavior:Vec<BehaviorComponent>)->Self{
                Self{
                    data:Data::new($($element),*),
                    behavior: behavior
                }
            }
            pub fn search(&self,manager: &EntityManager){
                for b in self.behavior.iter(){
                    (b.search)(&self.data,manager)
                }

            }
            pub fn update(&mut self){
                //let mut v = vec![];
                for b in self.behavior.iter(){
                    (b.update)(&mut self.data)
                }
                //return v
            }
        }
        struct EntityManager{
            elements: std::collections::HashMap<ID,Entity>
        }
            use rand::prelude::*;
            use rand::Rng;
        impl EntityManager{
            fn new()->Self{
                EntityManager{
                    elements:std::collections::HashMap::new()

                }
            }
            fn get_entity(&self,id:ID)->Option<&Entity>{
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
                
            })*
            fn get_id(&self)->ID{
                let mut rng = rand::thread_rng();
                let val = rng.gen();
                if self.elements.contains_key(&val){
                    return self.get_id();
                }else{
                    return val
                }

            }
            pub fn new_entity(&mut self,entity:Entity)->ID{
                let id = self.get_id();
                self.elements.insert(id,entity);
                return id;
            }
            pub fn process(&mut self){
                ///wrong but the general idea is that entities will be processed and given access
                for (id,e) in self.elements.iter(){
                    e.search(self);
                }
                ///to collection with entire list
                for (id,e) in self.elements.iter_mut(){
                    e.update();
                }

            }
        }
    }
}
mod test{
    macro_rules! create_entity_t {
        ($($element: ident: $ty: ty),*) => {
        
        struct Data{
            $($element: $ty),*
        }
        impl Data{
            pub fn new($($element:fn()->$ty),*)->Self{
                return Self{
                    $(
                        $element: $element()
                    ),*
                }
            }
            $(pub fn $element (&self) ->$ty {
                self.$element.clone()
            }
            )*
        }
        }
    }
    create_entity!(a:u32);
    mod t2{
      create_entity!(a:u32,b:f32);
    }
    use super::*;
}
