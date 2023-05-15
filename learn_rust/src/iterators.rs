// All iterator objects implement the Trait operator that has the next() method
// and return a Option<type>
//
// There are different kinds of iterators (and thus methods):
// -.iter()
// -.iter_mut()
// -.into_iter() => Copies it



pub fn run(){

    let mut vettore = vec![1, 2, 2];

    let iteratore = vettore.iter_mut();

    for i in iteratore{
        *i += 1;
        println!("{}", i);
    }

    // I can use the map method to apply a closure to every element
    let vettore2: Vec<_> = vettore.clone().into_iter().map(|x| x+1).collect();
    // Here vettore2 contains every element of vettore incremented by one

    // I can use filter method with an iter
    let vettore3: Vec<_> = vettore.clone().into_iter().filter(|x| x%2==0).collect();

    // .skip(i: u32) can skip i elements in an iterator
    let complicated_thing: u32 = vettore.clone()
        .into_iter()
        .skip(1).skip(1)
        .zip(vettore.clone().into_iter())
        .map(|(a, b)| a*b)
        .filter(|x| x%3==0)
        .sum();

}
