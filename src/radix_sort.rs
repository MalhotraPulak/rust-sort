use std::vec::Vec;


pub fn radix_sort(v: &mut Vec<u32>) -> &mut Vec<u32>{
    let radix = 256; // 0 to 2^8 -1 or 8 bits
    let mut new_vec = vec![0 as u32; v.len()];

    for i in (0..32).step_by(8) {

        let mut count_vec = vec![0; radix];  

        for ele in v.iter() {
            let idx = (ele>>i)&0xff;
            count_vec[idx as usize] += 1;
        } 

        for i in 1..count_vec.len(){
            count_vec[i] += count_vec[i - 1];
        }
        
        for j in (0..v.len()).rev() {
            let ele = v[j];
            let idx = ((ele>>i)&0xff) as usize;
            new_vec[count_vec[idx] - 1] = ele;
            count_vec[idx] -= 1;
        }


        std::mem::swap(v, &mut new_vec);

    }
    return v;
}
