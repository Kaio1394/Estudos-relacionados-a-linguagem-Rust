fn main() {
    //________________________________________Ownership__________________________________________
    // Para tipos primitivos como i32, acontece um copy de i para j. 
    // E i e j são independente uam da outra
    let i = 5;
    let j = i;
    println!("{}", i);
    println!("{}", j);

    // Variáveis do tipo complexo como vetores, não acontece um copy e sim uma tranferência de propriedade
    // de uma variável para outra.
    // Nesse exato caso, existe uma tranferência de C para W, logo  C não será mais acessível, pois fpo passado a propriedade para W.
    let c = vec![1, 2, 3];
    // let w = c;
    // println!("{:?}", c);
    // println!("{:?}", w);

    // Nesse caso a propriedade é transferida de C para a função anònima, e foi retransferida para C através do retorno da função.
    let foo = |c: Vec<i32>| -> Vec<i32>{
        println!("Vetor usado em foo");
        return c
    };
    let c = foo(c);
    println!("{:?}", c); 

    //________________________________________Borrowing__________________________________________

}
