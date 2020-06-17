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
        struct BehaviorComponent{
            search: fn(&$name,&EntityManager),
            update: fn(&mut $name),
        }
        impl BehaviorComponent{
            fn new(search: fn(&$name,&EntityManager), update: fn(&mut $name))->Self{
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
            data: $name,
            behavior: Vec<BehaviorComponent>,
        }
        impl Entity{
            pub fn new($($element:fn()->$ty),*,behavior:Vec<BehaviorComponent>)->Self{
                Self{
                    data:$name::new($($element).*),
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
            elements: HashMap<ID,Entity>
        }
        impl EntityManager{
            fn new()->Self{
                EntityManager{
                    elements:HashMap::new()

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
                
            }),*
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
    create_entity!(data,a:u32);
    use super::*;
  //  fn zero()->u32{
  //      0
  //  }
  //  #[test]
  //  fn new(){
  //      let e = data::new(||{0});
  //      assert_eq!(e.a(),0);
  //  }
  //  //#[test]
  //  //fn new_entity(){
  //  //    let mut e = Entity::<u32>::new(||{0},vec![|e,v|{e.a=1}]);
  //  //    e.process();
  //  //    assert_eq!(e.data.a(),1);

  //  //}
  //  #[test]
  //  fn entity_mgr(){
  //      let mut mgr = EntityManager::<u32>::new();
  //      let e = Entity::<u32>::new(||{0},vec![|e,v|{e.a=1}]);
  //      mgr.new_entity(e);


  //  }
}
