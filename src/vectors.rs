// resizable arrays
  pub fn vectors_are_arrays () {
      let mut numbers: Vec<i32> = vec![1,2,3];
      println!("The initial numbers in the array {:?}",numbers);

      // add to the array
      numbers.push(4);
      println!("number 4 added to the array {:?}",numbers);

      // remove from the array
      numbers.pop();
      println!("number 4 removed from the array {:?}",numbers);

      for x in numbers.iter() {
          println!("Number {}", x);
      }
  }