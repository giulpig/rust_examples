use std::rc::{Rc, Weak};
use std::cell::RefCell;

// Smart pointers implement Deref and Drop traits
// Most common are:
//  -Box
//  -Rc
//  -Ref, RefMut, both under RefCell
//  (Watch out: String and Vec are smart pointers too)

#[derive(Debug)]
enum ListBox{
    ConsBox(i32, Box<ListBox>),
    Nil,
}


#[derive(Debug)]
enum ListRc{
    ConsRc(i32, Rc<ListRc>),
    Nil,
}


#[derive(Debug)]
enum ListRcRefCell{
    ConsRcRefCell(Rc<RefCell<i32>>, Rc<ListRcRefCell>),
    Nil,
}


use ListBox::ConsBox;
use ListRc::ConsRc;
use ListRcRefCell::ConsRcRefCell;

pub fn run(){

// BOXES <-> Box<T>
// When you only have to store data of unknown size at compilation on the heap
// When you have lots of data
// When you want ownership of some data with various type but implementing some trait => this is a
//                                                                                      trait object

    let b: Box<u32> = Box::new(5);
    //println!("{}", b);

    // Recursive types are types that are unknown size at compile time
    let list: Box<ListBox> = Box::new(ConsBox(2, Box::new(ConsBox(3, Box::new(ListBox::Nil)))));

    // Deref trait links to deref operator "*"
    // This makes possible to write same code for refs and smart pointers
    // IN THE MANUAL YOU CAN FIND A MYBOX IMPLEMENTATION (starting at listing 15-8)
    // Coercion is when the compiler auto deferences (and thus obtais kind of generic behaviour)


// REFERENCE COUNTED SMART POINTERS <=> Rc<T>
// Only for single thread scenarios
// I cannot modify data inside

    let list: Rc<ListRc> = Rc::new(ConsRc(1, Rc::new(ConsRc(2, Rc::new(ConsRc(3, Rc::new(ListRc::Nil)))))));

    // Owned list (I have to clone it to append it to another list)
    let list2: ListRc = ConsRc(7, Rc::clone(&list)); //Rc::clone does not actually clone, it counts refs

    // Get reference count with this method
    //println!("{}", Rc::strong_count(&list));


// REFERENCE CELLS <=> RefCell<T>, used for internal mutability (immutable ref that contains
//                                                               mutable data)
// Single threaded
// Borrowing rules at runtime => I can mutate data inside RefCell as I want if I don't fuck up
//      Exmaple of fucking up: multiple mutable borrows

    let my_ref: RefCell<Vec<i32>> = RefCell::new(vec![2, 3, 4]);
    my_ref.borrow_mut().push(4);

    let some_ref: &Vec<i32> = &*my_ref.borrow();

// COMBINING RC AND REFCELL
// I can achieve multiple access to a mutable thing

    let value: Rc<RefCell<i32>> = Rc::new(RefCell::new(4));

    let list: Rc<ListRcRefCell> = Rc::new(ConsRcRefCell(Rc::clone(&value), Rc::new(ListRcRefCell::Nil)));

    let b: ListRcRefCell = ConsRcRefCell(Rc::clone(&value), Rc::clone(&list));

    // Value is set to 5 (so the two list items are modified)
    *value.borrow_mut() += 1;

    println!("This is the list: {:?}", b);


// WEAK POINTER <=> Weak<T>
// I can tranforam a Rc<T> into a Weak<T> by calling Rc::downgrade
// This way I will make use of a weak counter (parallel to the strong counter)
// Weak counter does not express ownership => it won't drop his value when it is 0
// To access it I have to call the Rc::upgrade() method (returns a Option<Rc>)


   #[derive(Debug)]
   struct Node{
       value: i32,
       parent: RefCell<Weak<Node>>,
       children: RefCell<Vec<Rc<Node>>>,
   }

   let leaf = Rc::new(Node{
       value: 5,
       children: RefCell::new(vec![]),
       parent: RefCell::new(Weak::new()),
    });

   let branch = Rc::new(Node{
       value: 5,
       children: RefCell::new(vec![Rc::clone(&leaf)]),
       parent: RefCell::new(Weak::new()),
    });

   // I have to use Rc::downgrade() on a Rc to get the Weak
   *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

   // I have to do Rc::upgrade() to get the Rc
   println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());

}
