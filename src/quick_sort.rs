fn quick_sort_helper(v: &mut Vec<u32>, start: usize , end: usize){
    if start >= end || start >= v.len() || end >= v.len() {
        return;
    }

    let pivot = v[start];
    let mut left = start + 1;
    let mut right = end;

    while left <= right {
        if  v[left] <= pivot {
            left += 1;
        } else if v[right] > pivot {
            right -= 1;
        } else {
            v.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    if left > 0 { 
        v.swap(start, left - 1); 
    }
    
    if left >= 2 {
        quick_sort_helper(v, start, left - 2); 
    }
    quick_sort_helper(v, right + 1, end); 
    
}


pub fn quick_sort(v: &mut Vec<u32>){
    quick_sort_helper(v, 0, v.len() - 1); 

}



