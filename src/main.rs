extern crate crossbeam;

fn main() {
    let mut v = vec![];

    for x in 0..100000000 {
        v.push(x);
    }

    let length = v.len()/2;
    let(head, tail) = v.split_at_mut(length);
    let(mut head_head, mut headTail) = head.split_at_mut(length/2);
    let(mut tailHead, mut tailTail) = tail.split_at_mut(length/2);
    //let slice2 = &mut v[3..6];
    crossbeam::scope(|scope| {
        scope.spawn(move |_| write_slice(&mut head_head, 2));
        scope.spawn(move |_| write_slice(&mut headTail, 100));
        scope.spawn(move |_| write_slice(&mut tailHead, 2));
        scope.spawn(move |_| write_slice(&mut tailTail, 100));
    }).expect("Something went wrong");

    println!("Finished threads");
    for score in &v {
        println!("{}",score);
    }
}

fn write_slice(arr : &mut [i64], multiplier: i64)
{
    for score in arr {
        for _x in 0..10000
        {
            *score += multiplier;
            *score -= multiplier;
        }

        }
}