
fn vec_loop(vec: Vec<i32>, results: &mut Vec<i32>, target: i32) {

    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            if vec[i] + vec[j] == target {
                results.push(i.try_into().unwrap());
                results.push(j.try_into().unwrap());
            }
        }
    }

    // for &i in vec.iter() {
    //     for &j in vec.iter() {
    //         if i + j == target {
    //             results.push(vec[i]);
    //             results.push(vec[j]);
    //         }
    //     }
    // }
}

fn main() -> Vec<i32> {
    let vec = vec![2,7,11,15];
    let mut results: Vec<i32> = Vec::new();
    let target = 9;
    vec_loop(vec, &mut results, target);
    println!("{:?}", results);
    return results;
}

