use std::time::Instant;
use rand::Rng;

fn main() {
    let n = 20;
    let c = 10;
    let mut vetor: Vec<i32> = vec![0;n as usize];

    let inicio_gvetor = Instant::now();
    gera_vetor_trabalhos(&mut vetor, n);
    let fim_gvetor = Instant::now();

    println!("Tempo para gerar o vetor de trabalhos: {:?}", fim_gvetor - inicio_gvetor);
    print!("Vetor de trabalhos: ");
    for itn in &vetor {
        print!("{} ", itn);
    }

    println!("");

    
    let inicio = Instant::now();
    let (min_caixas, empacot_min) = empacotamento_minimo(vetor.clone(), c);
    let fim = Instant::now();

    println!("Mínimo de caixas necessárias: {}", min_caixas);
    for (i, caixa) in empacot_min.iter().enumerate() {
        println!("Caixa {}: {:?}", i + 1, caixa);
    }

    println!("Tempo de execução algoritmo exponencial: {:?}", fim-inicio);
    

    let inicio = Instant::now();
    let (bins, total_bins) = algoritmo_nextfit_aproximado(&vetor, c);
    let fim = Instant::now();

    println!("Total de bins usados: {}", total_bins);
    for (i, (restante, conteudo)) in bins.iter().enumerate() {
        println!("Bin {}: itens = {:?}, capacidade restante = {}", i + 1, conteudo, restante);
    }

    println!("Tempo exec alg aprox: {:?}", fim-inicio);


}

fn gera_vetor_trabalhos(l: &mut Vec<i32>, n: i32){

    let espera = ceil_div2(n);
    let mut cont = 0;
    let mut prox_verif = 1 + espera;
    
    for i in 0..n {
        if i == prox_verif {
            l[i as usize] = (10 - cont) + 1;
            cont = 0;
            prox_verif = i + espera;
        }
        else {
            l[i as usize] = rand::rng().random_range(1..=10);
            cont = cont + l[i as usize];
                if cont > 10 {
                    cont = 0;
                    prox_verif = i + espera;
                }
        }
    }
}

fn ceil_div2(n: i32) -> i32 {
    (n / 2) + (n % 2).signum()
}

fn encontra_empacotamento(p: &Vec<i32>, c: i32, peso_caixas: &mut Vec<i32>, itens_caixa: &mut Vec<Vec<i32>>, n: usize, i: usize, k: usize, min_caixas: &mut usize, empacot_min: &mut Vec<Vec<i32>>,) {
    if i >= n {
        if *min_caixas > k {
            *min_caixas = k;
            *empacot_min = itens_caixa[0..k].to_vec();
        }
        return;
    }

    for j in 0..k {
        if peso_caixas[j] >= p[i] {
            peso_caixas[j] -= p[i];
            itens_caixa[j].push(p[i]);

            encontra_empacotamento(p, c, peso_caixas, itens_caixa, n, i + 1, k, min_caixas, empacot_min);

            itens_caixa[j].pop();
            peso_caixas[j] += p[i];
        }
    }

    peso_caixas[k] -= p[i];
    itens_caixa[k].push(p[i]);

    encontra_empacotamento(p, c, peso_caixas, itens_caixa, n, i + 1, k + 1, min_caixas, empacot_min);

    itens_caixa[k].pop();
    peso_caixas[k] += p[i];
}

fn empacotamento_minimo(p: Vec<i32>, c: i32) -> (usize, Vec<Vec<i32>>) {
    let n = p.len();
    let mut peso_caixas = vec![c; n];
    let mut itens_caixa: Vec<Vec<i32>> = vec![Vec::new(); n];

    let mut min_caixas = n;
    let mut empacot_min: Vec<Vec<i32>> = Vec::new();

    encontra_empacotamento(&p, c, &mut peso_caixas, &mut itens_caixa, n, 0, 0, &mut min_caixas, &mut empacot_min);

    (min_caixas, empacot_min)
}

fn algoritmo_nextfit_aproximado(pesos: &Vec<i32>, capacidade: i32) -> (Vec<(i32, Vec<i32>)>, usize) {
    let mut bins: Vec<(i32, Vec<i32>)> = Vec::new();
    let mut k = 0;
    let mut cap_restante = capacidade;

    
    bins.push((capacidade, Vec::new()));

    for &peso in pesos {
        if peso <= cap_restante {
            
            bins[k].1.push(peso);
            cap_restante -= peso;
            
        } else {
            // Atualiza capacidade restante do bin atual
            bins[k].0 = cap_restante;

            // Cria novo bin
            k += 1;
            cap_restante = capacidade - peso;
            bins.push((cap_restante, vec![peso]));
            
        }
    }

   
    if cap_restante < capacidade {
        bins[k].0 = cap_restante;
    }

    (bins, k + 1) // k+1 é o total de bins usados
}
