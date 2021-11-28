use std::iter::FromIterator;


fn merge(v: &mut Vec<u32>, start: usize, mid: usize, end: usize){
    // print!("{} {}\n",start, end);
    let first = Vec::from_iter(v[start..mid].iter().cloned());
    let second = Vec::from_iter(v[mid..end].iter().cloned());
    
    let mut first_idx = 0;
    let mut second_idx = 0;
    let mut v_idx = start;
    while first_idx < first.len() && second_idx < second.len(){
        if first[first_idx] < second[second_idx] {
            v[v_idx] = first[first_idx];
            first_idx += 1;
        } else {
            v[v_idx] = second[second_idx];
            second_idx += 1;
        }
        v_idx += 1;
    }

    while first_idx < first.len() {
        v[v_idx] = first[first_idx];
        v_idx += 1;
        first_idx += 1;
    }

    while second_idx < second.len() {
        v[v_idx] = second[second_idx];
        v_idx += 1;
        second_idx += 1;
    }

}

fn merge_sort_helper(v: &mut Vec<u32>, start: usize, end: usize){
    if start < end - 1 {
        let mid = (start + end) / 2;
        merge_sort_helper(v, start, mid);
        merge_sort_helper(v, mid, end);
        merge(v, start, mid, end);
    }
}



pub fn merge_sort(v: &mut Vec<u32>){
   merge_sort_helper(v, 0, v.len());  
}

