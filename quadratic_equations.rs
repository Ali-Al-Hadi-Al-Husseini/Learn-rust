use std::io;
fn main() {
    let indexs: [i64;3] = [get_input(2),get_input(1),get_input(0)];

    let delta = (indexs[1] * indexs[1]) - 4 * (indexs[0] * indexs[2]);
    let  x1 = (-indexs[1] + delta.pow(1/2)) / (2 *  indexs[0]); 
    let  x2 = (-indexs[1] - delta.pow(1/2)) / (2 *  indexs[0]); 

    println!("your first solution is {}",x1);
    println!("your second solution is {}",x2);


}
fn get_input(index : u8) -> i64{
    let mut output = String::new();

    if index != 0{

        println!("give me your x power {index} index :");
        io::stdin().read_line(&mut output).expect("failed to read");

    }else{
        println!("give me your contant:");
        io::stdin().read_line(&mut output).expect("failed to read");
    };

   output.trim().parse().expect("Input not an integer")
}
