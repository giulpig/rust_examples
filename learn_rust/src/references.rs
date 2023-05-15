

pub fn run(){

    // With primitives is just a copy
    let arr1 = [1,2,3];
    let mut arr2 = arr1;

    arr2[2] = 4;

    println!("{:?}", (&arr1, &arr2));


    // Let's see UDT
    let mut vec1 = vec![1,2,3];
    {
        let vec2 = &mut vec1;

        vec2[2] = 4;
    }
    // Vec1 is no longer valid after this call!!! (Move semantic)
    println!("{:?}", vec1);

    // I can drop variables manually like this
    drop(arr1);

}
