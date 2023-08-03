use nalgebra::*;

fn main() {
    let prn = gen_prn(1, true);
    println!("{}",prn.fixed_rows::<10>(0));
}

#[allow(unused_variables)]
pub fn gold_gen(ic: DVector<i32>, taps: Vec<usize>) -> DMatrix<i32> {

    let n = ic.len();
    let num = 2_i32.pow(n as u32) - 1;

    let mut g: DMatrix<i32> = DMatrix::zeros(num as usize,n);
    g.set_row(0,&ic.transpose());
    
    for i in 0..(num-1) as usize {

        let mut temp_val = g[(i,taps[0]-1)];
        for j in 1..taps.len() {
            temp_val = temp_val ^ g[(i,taps[j]-1)];
        }
        let temp_row = g.fixed_view::<1,9>(i,0).clone_owned();
        g.fixed_view_mut::<1,9>(i+1,1).copy_from(&temp_row);
        g[(i+1,0)] = temp_val;
    }
    g
}

pub fn bpsk_map(vector: &mut DVector<i32>) {
    for i in 0..vector.len(){
        vector[i] = if vector[i] == 0 {1} else {-1};
    }
}

pub fn cycle_add(g1: DVector<i32>, g2: DMatrix<i32>, prn_taps: Vec<usize>) -> DVector<i32> {
    let mut s = g1.clone_owned();
    for i in 0..g1.len() {
        let mut temp = g2[(i,prn_taps[0]-1)];
        for j in 1..prn_taps.len() {
            temp = temp ^ g2[(i,prn_taps[j]-1)];
        }
        s[i] = g1[i] ^ temp;
    }
    s
}

pub fn gen_prn(prn_num: usize, bpsk_flag: bool) -> DVector<i32> {

    let prn_lib = [vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![1, 9], vec![2, 10],
        vec![1, 8], vec![2, 9], vec![3, 10], vec![2, 3], vec![3, 4], vec![5, 6], vec![6, 7], 
        vec![7, 8], vec![8, 9], vec![9, 10], vec![1, 4], vec![2, 5], vec![3, 6], vec![4, 7], 
        vec![5, 8], vec![6, 9], vec![1, 3], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9], 
        vec![8, 10], vec![1, 6], vec![2, 7], vec![3, 8], vec![4, 9], vec![5, 10], vec![4, 10],
        vec![1, 7], vec![2, 8]];

    let prn_taps = prn_lib[prn_num-1].clone();
    let g1_taps = vec![3,10]; 
    let g2_taps = vec![2,3,6,8,9,10];
    let ic = DVector::from(vec![1,1,1,1,1,1,1,1,1,1]);
    
    let g1 = gold_gen(ic.clone(),g1_taps);
    let g2 = gold_gen(ic.clone(),g2_taps);
    let g1 = g1.column(9).clone_owned();

    let mut prn_code = cycle_add(g1,g2,prn_taps);
    if bpsk_flag {bpsk_map(&mut prn_code)};

    prn_code
}
