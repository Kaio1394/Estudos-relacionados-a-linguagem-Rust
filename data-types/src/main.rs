fn main() {
    // setando o tipo do array e o tamanho
    // não pode ser adicionado novos elementos em um arrays
    // mas podem ser alterados
    const DEFAULT: i32 = 3;
    let numbers: [i32; 5] = [DEFAULT; 5];
    let array: [i32; 4] = [0, 1, 2, 3];
    let _doubles: [f64; 4] = [1.0, 2.0, 3.0, 4.0]; 
    println!("{}", array.len());
    println!("{:?}", array[0]); // pegando índice de um array
    println!("{:?}", numbers);
    
    // somatória dos elementos de um array
    let mut soma = 0;
    for i in 0..numbers.len() {
        soma += numbers[i];
        println!("{} -> {}", i, numbers[i]);
        numbers.iter();
    }
    println!("{}", soma);

    // Vetores
    // Pode ser adicionado novos elementos
    let mut primes: Vec<Vec<i32>> = Vec::new();
    primes = vec![vec![2, 3, 5], vec![2, 3, 5]];
    primes.push(vec![1, 2]);
    println!("{:?}", primes[0]);
}
