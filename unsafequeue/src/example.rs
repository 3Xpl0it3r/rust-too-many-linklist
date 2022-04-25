fn opaque_read(val: &i32) {
    println!("[shared-reference-test] -> {}", val);
}
/*
#[cfg(test)]
mod test{
    use std::{f64::consts, cell::{Cell, UnsafeCell}};

    use super::opaque_read;
    #[test]
    fn test_unsafecell(){
        unsafe{
            let mut data = UnsafeCell::new(10);
            let mref1 = data.get_mut();
            let ptr2 = mref1 as *mut i32;
            let sref3 = &*ptr2;

            *ptr2 += 2;

            opaque_read(sref3);
            *mref1 += 1;
            println!("-------> | {}", *data.get());
        }
    }

    #[test]
    fn test_interior_mutali(){
        unsafe{
            let mut data = Cell::new(10);

            let mref1 = &mut data;
            let ptr2 = mref1 as *mut Cell<i32>;

            let sref3 = &*mref1;

            sref3.set(sref3.get() + 3);
            (*ptr2).set((*ptr2).get() + 2);
            mref1.set(mref1.get() + 1);

            println!("->    {}", data.get());
        }
    }

    #[test]
    fn test_share_reference(){

        {

            let mut data = 10;
            let mref1 = &mut data;      // &mut
            let sref2 = &mref1;         // &&mut i32
            let sref3 = sref2;          // &&mut i32
            let sref4 = &*sref2;        // &&mut i32

            opaque_read(sref3);
            opaque_read(sref2);
            opaque_read(sref4);
            opaque_read(sref2);
            opaque_read(sref3);

            *mref1 += 1;
            
            opaque_read(&data);
        }

        unsafe{
                let mut data = 10;
                let mref1 = &mut data;
                let ptr2 = mref1 as *mut i32;
                let sref3= &*mref1;
                let ptr4 = sref3 as *const i32 as *mut i32;

                opaque_read(&*ptr4);
                opaque_read(sref3);

                *ptr2 += 2;
                *mref1 += 1;
                opaque_read(&data);
        }
    }
    #[test]
    fn array_example(){
        unsafe{
            let mut data = [0;10];
            let ref1_at_0 = &mut data[0];               // Reference to 0th element
            let ptr2_at_0 = ref1_at_0 as *mut i32;      // Prt to 0th element
            // let ptr3_at_1 = ptr2_at_0.add(1);           // Prt to 1st element
            let ptr3_at_0 = ptr2_at_0;           // Prt to 0st element
            let ptr4_at_0 = ptr2_at_0.add(0);
            let ptr5_at_0 = ptr3_at_0.add(1).sub(1);
            


            *ptr3_at_0 += 3;
            *ptr2_at_0 += 2;
            *ptr4_at_0 += 4;
            *ptr5_at_0 += 5;
            *ptr3_at_0 += 3;
            *ptr2_at_0 += 2;
            *ref1_at_0 += 1;

            println!("{:?}", &data[..]);
        }
    }
    #[test]
    fn more_complex(){
        unsafe{
            let mut data = 10;
            let ref1 = &mut data;           // &mut
            let ptr2  = ref1 as *mut _;     // *mut _
            let ref3 = &mut *ptr2;          // &mut
            let ptr4 = ref3 as *mut _;      // *mut _


            // access the firsrt raw pointer
            // *ptr2 += 2;

            // then access things in `borrow stack order`
            *ptr4 += 4;
            *ref3 += 3;
            *ptr2 += 2;
            *ref1 += 1;
            
            println!("{}", data);
        }
    }
    #[test]
    fn test_basic_1(){
        /*
        unsafe {
            let mut data = 10;
            let ref1 = &mut data;
            let ptr2 = ref1 as *mut _;

            // ORDER SWAPPED!
            *ref1 += 1;
            *ptr2 += 2;

            println!("{}", data);
        }
        */

    }


}
*/
