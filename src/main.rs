fn main() {
    /* Fahrenheit to Celcius convert
    let temp = 75;
    fn f_to_c(t: i32) {
        let c_temp = (t - 32) as f64 *(5.0/9.0);
        println!("{c_temp}");
    }
    f_to_c(temp);
    */

    fn fibonacci(n: i32){
        let mut prev = 0;
        let mut cur = 1;
        for _num in 1..n {
            let a = prev + cur;
            prev = cur;
            cur = a;
        }
        println!("{cur}");
    }
    fibonacci(19)
}