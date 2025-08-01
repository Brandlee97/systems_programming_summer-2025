//temp functions
fn fahrenheit_to_celsius(f: f64) -> f64{
    let c = (5.0/9.0) * (f-32.0);
    c
}
fn celsius_to_fahrenheit(c: f64) -> f64{
    let f = (c * 9.0/5.0) + 32.0;
    f
}


fn main() {

    // tempature function
    let mut Farenheit = 99.0;
    let Celcius = fahrenheit_to_celsius(Farenheit);
    println!("your degrees in Farenheit is {} that converts to {:.2} celcius.", Farenheit, Celcius);
    
    let mut counter = 5;
    while counter != 0
    {
        Farenheit += 1.0;
        let Celcius = fahrenheit_to_celsius(Farenheit);
        println!("your degrees in Farenheit is {} that converts to {:.2} celcius.", Farenheit, Celcius);
        counter -= 1;
    }
        

}






