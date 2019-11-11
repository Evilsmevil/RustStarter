extern crate crossbeam;

fn main() {
    let mut v = vec![0,1,2,3,4,5,6];

    let length = v.len()/2;
    let(mut head, mut tail) = v.split_at_mut(length);

    //let slice2 = &mut v[3..6];
    crossbeam::scope(|scope| {
        scope.spawn(move |_| write_slice(&mut head, 2));
        scope.spawn(move |_| write_slice(&mut tail, 100));
    }).expect("Something went wrong");

    for score in &v {
        println!("{}",score);
    }
}

fn write_slice(arr : &mut [i32], multiplier: i32)
{
    for score in arr {
        *score *= multiplier;
        }
}