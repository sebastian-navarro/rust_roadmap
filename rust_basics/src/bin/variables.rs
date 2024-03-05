/* Variables and mutability */


fn main(){
    // Variables by default are inmutable
    let a = 10;
    println!("Initial value of a is {}", a);
    // This is calling shadowing, because we define a new variable with the same name
    let a = 20;

    let mut b = 10; // We have to use reserved word mut to do it mutable an variable

    println!("Initial value of a is {} and b is {}", a,b);
    
    // Here we use mutability and change de value of b;
    b = 20;

    println!("El valor de a es {} y de b es {}", a ,b);

    // We can define a variable like a constant
    const PI:f32 = 3.141517;
    println!("Valor de la constante inmutable PI es {}", PI);
}