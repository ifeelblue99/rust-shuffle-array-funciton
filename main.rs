use rand::Rng;

fn main() {
  let data = vec!(1, 2, 3, 4, 5, 6, 7, 8);
  println!("shuffled. {:?}", shuffle_array(data));
}

fn shuffle_array(mut arr: Vec<usize>) -> Vec<usize> {
  let length = arr.len();
  
  if length <=1 {return arr}

  let mut rng = rand::thread_rng();
  let mid = length/2;

  for x in 0..mid {
    let rand_index = rng.gen_range(0..length);
    let temp = arr[rand_index];
    arr[rand_index] = arr[x];
    arr[x] = temp;
  }
  arr
}
