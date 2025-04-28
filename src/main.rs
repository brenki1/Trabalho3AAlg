use std::time::Instant;
use rand::Rng;

fn main() {
    let n = 10;
    let mut vetor: Vec<i32> = vec![0;n as usize];

    gera_vetor(&mut vetor, n);

    for j in 0..n {
        println!("{}", vetor[j as usize]);
    }
}

fn gera_vetor(l: &mut Vec<i32>, n: i32){

    let espera = ceil_div2(n);
    let mut cont = 0;
    let mut prox_verif = 1 + espera;
    
    for i in 0..n {
        if(i == prox_verif) {
            l[i as usize] = (10 - cont) + 1;
            cont = 0;
            prox_verif = i + espera;
        }
        else {
            l[i as usize] = rand::rng().random_range(1..=10);
            cont = cont + l[i as usize];
                if(cont > 10) {
                    cont = 0;
                    prox_verif = i + espera;
                }
        }
    }
}

fn ceil_div2(n: i32) -> i32 {
    (n / 2) + (n % 2).signum()
}