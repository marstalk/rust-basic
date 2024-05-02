/**
100278. Right Triangles 显示英文描述
通过的用户数1133
尝试过的用户数1284
用户总通过次数1144
用户总提交次数1990
题目难度Medium
You are given a 2D boolean matrix grid.
Return an integer that is the number of right triangles that can be made with the 3 elements of grid such that all of them have a value of 1.
Note:
A collection of 3 elements of grid is a right triangle if one of its elements is in the same row with another element and in the same column with the third element. The 3 elements do not have to be next to each other.


Example 1:
0	1	0
0	1	1
0	1	0
0	1	0
0	1	1
0	1	0
Input: grid = [[0,1,0],[0,1,1],[0,1,0]]
Output: 2
Explanation:
There are two right triangles.

Example 2:
1	0	0	0
0	1	0	1
1	0	0	0
Input: grid = [[1,0,0,0],[0,1,0,1],[1,0,0,0]]
Output: 0
Explanation:
There are no right triangles.

Example 3:
1	0	1
1	0	0
1	0	0
1	0	1
1	0	0
1	0	0
Input: grid = [[1,0,1],[1,0,0],[1,0,0]]
Output: 2
Explanation:
There are two right triangles.

Constraints:
1 <= grid.length <= 1000
1 <= grid[i].length <= 1000
0 <= grid[i][j] <= 1
 */
pub struct Solution {}
impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let m = grid.len();
        let n = grid[0].len();

        let mut res = 0;

        // on direction: left to right, top to botton
        let mut one_cnt_grid_2 = vec![vec![vec![0; 2]; n]; m];
        for i in 1..m {
            for j in 1..n{
                let mut x = 0;
                let mut y = 0;

                if i + 1 < m {
                    x = match grid[i - 1][j] {
                        0 => one_cnt_grid_2[i - 1][j][0],
                        _ => 1 + one_cnt_grid_2[i - 1][j][0],
                    }
                }

                if j + 1 < n {
                    y = match grid[i][j - 1] {
                        0 => one_cnt_grid_2[i][j - 1][1],
                        _ => 1 + one_cnt_grid_2[i][j - 1][1],
                    }
                }
                one_cnt_grid_2[i][j] = vec![x, y];
            }
        }
        println!("{:?}", one_cnt_grid_2);

        // another direction: right to left, botton to top
        let mut one_cnt_grid = vec![vec![vec![0; 2]; n]; m];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let mut x = 0;
                let mut y = 0;

                if i + 1 < m {
                    x = match grid[i + 1][j] {
                        0 => one_cnt_grid[i + 1][j][0],
                        _ => 1 + one_cnt_grid[i + 1][j][0],
                    }
                }

                if j + 1 < n {
                    y = match grid[i][j + 1] {
                        0 => one_cnt_grid[i][j + 1][1],
                        _ => 1 + one_cnt_grid[i][j + 1][1],
                    }
                }
                one_cnt_grid[i][j] = vec![x, y];
            }
        }

        println!("{:?}", one_cnt_grid);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 && one_cnt_grid[i][j][0] > 0 && one_cnt_grid[i][j][1] > 0 {
                    println!("{}, {}", i, j);
                    res += 1;
                }

                if grid[i][j] == 1 && one_cnt_grid_2[i][j][0] > 0 && one_cnt_grid_2[i][j][1] > 0 {
                    println!("{}, {}", i, j);
                    res += 1;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_right_triangles_1() {
        assert_eq!(
            //[0,1,0],
            //[0,1,1],
            //[0,1,0]]
            Solution::number_of_right_triangles(vec![
                vec![0, 1, 0], 
                vec![0, 1, 1], 
                vec![0, 1, 0]
                ]),
            2
        );
    }

    #[test]
    fn test_number_of_right_triangles() {
        assert_eq!(
            Solution::number_of_right_triangles(vec![
                vec![0, 1, 0], 
                vec![0, 1, 1], 
                vec![0, 1, 0], 
                vec![0, 1, 0], 
                vec![0, 1, 1], 
                vec![0, 1, 0]
                ]),
            2
        );

        assert_eq!(
            Solution::number_of_right_triangles(vec![
                vec![1, 0, 0, 0], 
                vec![0, 1, 0, 1], 
                vec![1, 0, 0, 0]
                ]),
            0);
        
        assert_eq!(
            Solution::number_of_right_triangles(vec![
                vec![1, 0, 1], 
                vec![1, 0, 0], 
                vec![1, 0, 0], 
                vec![1, 0, 1], 
                vec![1, 0, 0], 
                vec![1, 0, 0]
                ]),
            2);
    }
}
