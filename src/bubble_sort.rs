pub fn bubble_sort(v: &mut Vec<u32>){

    let mut swapped: bool = true;

    while swapped{
        swapped = false;
        for i in 0..(v.len() - 1) {
            if v[i + 1] < v[i] {
                v.swap(i + 1, i);
                swapped = true;
            }
        }
    }

}




