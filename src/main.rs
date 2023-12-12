fn main() {
    /* Fahrenheit to Celcius convert
    let temp = 75;
    fn f_to_c(t: i32) {
        let c_temp = (t - 32) as f64 *(5.0/9.0);
        println!("{c_temp}");
    }
    f_to_c(temp);
    */

    /* Fibonacci value finder 
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
    */

    fn twelve_days() {
        let num_arr = ["first", "second", "third", "fourth", "fith", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
        let quote_arr = ["A partridge in a pear tree", "Two turtle doves and", "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];
        for n in 0..11 {
            let num = num_arr[n];
            println!("On the {num} day of Christmas, my true love sent to me");
            for a in (0..(n+1)).rev() {
                let quote = quote_arr[a];
                println!("{quote}");
            }
            println!()
        }
    }
    twelve_days();
}