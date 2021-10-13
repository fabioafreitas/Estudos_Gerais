struct Color1(u8, u8, u8);

#[derive(Copy, Clone)]
struct Color2(u8, u8, u8);

fn foo(x: Color1) {
    println!("{},{},{}",x.0,x.1,x.2);
}

fn foo2(x: &Color1) {
    println!("{},{},{}",x.0,x.1,x.2);
}

fn foo3(x: Color2) {
    println!("{},{},{}",x.0,x.1,x.2);
}

// no copy trait, no reference
/**
 * em rust, quando uma struct é passada como argumento
 * de uma funcao, seu conteudo é literalmente movido
 * para o escopo desta funcao (outras linguagens criam uma cópia
 * da struct), logo, caso haja uma
 * referencia a variaveis dessa struct no escopo anteror
 * ocorrerá um erro, pois eles não estão mais no escopo inicial
 */
fn example1() {
    // let x = Color1(255,255,255);
    // foo(x); // proxima linha gera erro
    // foo(x);
}

// no copy trait, with reference
/**
 * ao passar a struct como referencia, estamos lidando
 * diretamente com o conteudo da mesma através de ponteiros, 
 * logo sua referencia no escopo anterior não será afetada
 */
fn example2() {
    let x = Color1(255,255,255);
    foo2(&x);
    foo2(&x);
}

// with copy trait
/**
 * a derivativa #[derive(Copy, Clone)], é o copy clone trait
 * ela indica ao compilador que ao passar por referencia esta
 * struct, uma cópia deve ser enviada como argumento, ao invés
 * da referencia original da mesma. Isto previne o erro do exemplo1,
 * entretanto aumenta a utilização da memória
 */
fn example3() {
    let x = Color2(255,255,255);
    foo3(x);
    foo3(x);
}


fn main() {
    //example1();
    //example2();
    example3();
}
