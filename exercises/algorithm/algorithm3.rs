/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

/// 使用冒泡排序算法对切片进行排序。
/// 
/// # 参数
/// - `array`: 一个可变切片，其元素实现了 `PartialOrd` （部分排序）trait。
/// 
/// # 说明
/// 此函数通过冒泡排序算法对切片中的元素进行排序，从小到大排列。
/// 冒泡排序是一种简单的排序算法，它重复地遍历要排序的元素列，
/// 依次比较相邻的两个元素，如果它们的顺序错误就把它们交换过来。
/// 遍历元素的工作是重复进行的，直到没有再需要交换的元素为止。
/// 
/// # 例子
/// ```
/// let mut nums = [4, 2, 6, 1, 5];
/// sort(&mut nums);
/// assert_eq!(nums, [1, 2, 4, 5, 6]);
/// ```
fn sort<T>(array: &mut [T])
    where T: PartialOrd,
{
    // 获取数组的长度
    let len = array.len();
    // 外层循环，控制遍历的轮次
    for i in 0..len {
        // 内层循环，负责相邻元素的比较和交换
        for j in 0..len - 1 - i {
            // 如果当前元素大于下一个元素，则交换它们的位置
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}