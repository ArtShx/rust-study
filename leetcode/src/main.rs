mod rotate;

fn main() {
    let mut matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    rotate::Solution::rotate(&mut matrix);
    println!("{:?}", matrix)
}
