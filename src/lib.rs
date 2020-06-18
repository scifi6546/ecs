/// What is Left:
/// InterEntity Communication
///Usage:
/// create entity with create_entity.
/// ```
/// create_entity!(BasicData,wealth:u32);
/// let mut e = Entity::new(||{0},vec![|entity,msg|{entity.wealth++}]);
/// e.process();
/// ```
macro_rules! create_entity {
    ($($element: ident: $ty: ty),*) => {
    
        #[allow(dead_code)]
        struct Data{
            $($element: $ty),*
        }
        impl Data{
            #[allow(dead_code)]
            pub fn new($($element:fn()->$ty),*)->Self{
                return Self{
                    $(
                        $element: $element()
                    ),*
                }
            }
            $(
                #[allow(dead_code)]
                pub fn $element (&self) ->$ty {
                self.$element.clone()
            }
            )*
        }
        #[allow(dead_code)]
        struct BehaviorComponent{
            search: fn(&Data,&EntityManager),
            update: fn(&mut Data),
        }
        impl BehaviorComponent{
            #[allow(dead_code)]
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
        #[allow(dead_code)]
        struct Entity{
            data: Data,
            behavior: Vec<BehaviorComponent>,
        }
        impl Entity{
            #[allow(dead_code)] 
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
        #[allow(dead_code)]
        struct EntityManager{
            elements: std::collections::HashMap<ID,Entity>
        }
        use rand::Rng;
        impl EntityManager{
            #[allow(dead_code)]
            fn new()->Self{
                EntityManager{
                    elements:std::collections::HashMap::new()

                }
            }
            #[allow(dead_code)]
            fn get_entity(&self,id:ID)->Option<&Entity>{
                self.elements.get(&id)
            }
            ///Function to get elements in Entity With id
            $(
                #[allow(dead_code)]
                pub fn $element(&self,id:ID)->Option<$ty>{
                let entity = self.get_entity(id);
                if entity.is_some(){
                    Some(entity.unwrap().data.$element())
                }else{
                    None
                }
                
            })*
            #[allow(dead_code)]
            fn get_id(&self)->ID{
                let mut rng = rand::thread_rng();
                let val = rng.gen();
                if self.elements.contains_key(&val){
                    return self.get_id();
                }else{
                    return val
                }

            }
            #[allow(dead_code)]
            pub fn new_entity(&mut self,entity:Entity)->ID{
                let id = self.get_id();
                self.elements.insert(id,entity);
                return id;
            }
            #[allow(dead_code)]
            pub fn process(&mut self){
                for (_id,e) in self.elements.iter(){
                    e.search(self);
                }
                //to collection with entire list
                for (_id,e) in self.elements.iter_mut(){
                    e.update();
                }

            }
        }
    }
}
mod test{
    create_entity!(a:u32);
    mod t2{
      create_entity!(a:u32,b:f32);
    }
}
