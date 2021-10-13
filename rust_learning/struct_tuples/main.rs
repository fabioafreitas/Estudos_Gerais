struct Teste1(u8, u8, u8);

struct Teste2 {
    nome: String,
    idade: u8,
    tupla: (u8, u8, u8)
}

struct Teste3 {
    nome: String,
    idade: u8,
    tupla: Teste1
}

fn main() {
    let test1 = Teste1(1,2,3);
    println!("{}{}{}",test1.0,test1.1,test1.2);

    let mut test2 = Teste2 {
        nome: "fabio".to_string(),
        idade: 23,
        tupla: (4,5,6)
    };
    test2.tupla.0 = 10;
    println!("{}\n{}\n{},{},{}",
        test2.nome,
        test2.idade,
        test2.tupla.0,
        test2.tupla.1,
        test2.tupla.2
    );

    let mut test3 = Teste3 {
        nome: "fabio".to_string(),
        idade: 23,
        tupla: Teste1(4,5,6)
    };
    test3.tupla.0 = 10;
    println!("{}\n{}\n{},{},{}",
        test3.nome,
        test3.idade,
        test3.tupla.0,
        test3.tupla.1,
        test3.tupla.2
    );

}