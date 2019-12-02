use std::collections::HashMap;

//Not really object oriented. 
//Supports creating structs and adding 'member' functions
//with impls.
struct Node {
   value: i32,
   next_element: usize,
   previous_elements: Vec<usize>
}

impl Node {
   fn new(new_value: i32, position: usize) -> Node {
      Node {
         value: new_value,
         next_element: position,
         previous_elements: Vec::new()
      }
   }
}

struct DisjointSet {
   map: HashMap<i32, usize>,
   elements: Vec<Node>,
   empty_positions: Vec<usize>
}

impl DisjointSet {
   fn new() -> DisjointSet {
      DisjointSet {
         map: HashMap::new(),
         elements: Vec::new(),
         empty_positions: Vec::new()
      }
   }
   
   fn add(&mut self, item: i32) {
      match self.empty_positions.pop() {
         Some(p) => {   //Functional-style lambda / closure
            let node = Node::new(item, p);
            self.map.insert(item, p);
            self.elements[p] = node;
         },
         None => {
            let node = Node::new(item, self.elements.len());
            self.map.insert(item, self.elements.len());
            self.elements.push(node);
         }
      }
   }

   //Optional is used as a method of error handling. 
   fn find(&mut self, item: i32) -> Option<i32> {
      match self.map.get(&item) {
         Some(p) => {   //Returns value if the key does exist. 
            let mut pos: usize = *p;

            while(self.elements[pos].next_element != pos) {
               pos = self.elements[pos].next_element;
            }

            return Some(self.elements[pos].value);
         },
         None => {   //Returns None if the key doesn't exist. 
            return None;
         }
      }
   }

   fn union(&mut self, item1: i32, item2: i32) {
      //An example of passing ownership of an object. 
      //item1 becomes borrowed by the map.get function.
      //After it is done executing, ownership is given back
      //to this function.
      let positionOption1 = self.map.get(&item1);
      let positionOption2 = self.map.get(&item2);

      if(positionOption1.is_none() || positionOption2.is_none()) {
         return;
      }

      let position1 = *(positionOption1.unwrap()); //Not a borrow because the value of unwrap is copied.
      let position2 = *(positionOption2.unwrap());

      let rootVal = self.find(self.elements[position1].value).unwrap();
      let root = *(self.map.get(&rootVal).unwrap());

      self.elements[root].next_element = position2;
      self.elements[position2].previous_elements.push(root);
   }
}

fn main() {
   println!("Before Union of 2 and 3: ");

   let mut disjoint_set = DisjointSet::new();
   disjoint_set.add(1);
   disjoint_set.add(2);
   disjoint_set.add(3);

   let mut res = disjoint_set.find(1);
   println!("{}", res.get_or_insert(-1));
   
   let mut res = disjoint_set.find(2);
   println!("{}", res.get_or_insert(-1));

   let mut res = disjoint_set.find(3);
   println!("{}", res.get_or_insert(-1));

   println!("\nAfter Union of 2 and 3: ");

   disjoint_set.union(2, 3);
   
   let mut res = disjoint_set.find(1);
   println!("{}", res.get_or_insert(-1));
   
   let mut res = disjoint_set.find(2);
   println!("{}", res.get_or_insert(-1));

   
   let mut res = disjoint_set.find(3);
   println!("{}", res.get_or_insert(-1));
}
