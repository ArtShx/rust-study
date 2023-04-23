pub struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // https://leetcode.com/problems/rotate-image/description/
        // You are given an n x n 2D matrix representing an image, 
        // rotate the image by 90 degrees (clockwise).
        // You have to rotate the image in-place, which means you have to modify 
        // the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.
        // Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
        // Output: [[7,4,1],[8,5,2],[9,6,3]]

        let n = matrix.len();
        let mut i = 0;  // goes internally the square
        let mut step; // convolution from 0..n-1 making the swaps
        
        while i < n - 1 {
            step = 0;

            while (i as i8 + step as i8) - (n as i8 - 1 - i as i8) < 0 {
                let t = matrix[i][i+step];
                let r = matrix[i+step][n-1-i];
                let b = matrix[n-1-i][n-1-i-step];
                let l = matrix[n-1-i-step][i];

                matrix[i][i+step] = l;
                matrix[i+step][n-1-i] = t;
                matrix[n-1-i][n-1-i-step] = r;
                matrix[n-1-i-step][i] = b;

                step += 1;
            }
            i += 1;
        }        

    }
}
