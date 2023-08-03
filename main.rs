use nalgebra::*;

fn main() {
    let ic = DVector::from_vec(vec![1,1,1,1,1,1,1,1,1,1]);
    goldgen(ic,[3,10]);
}

#[allow(unused_variables)]
pub fn goldgen(ic: DVector<i32>, taps: [usize;2]) {

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
}
