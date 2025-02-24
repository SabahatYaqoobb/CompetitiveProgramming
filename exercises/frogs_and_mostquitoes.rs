use std::collections::{BTreeMap, HashMap};

#[derive(Copy, Clone, Debug)]
struct Frog {
    position: u32, 
    reach: u32, 
    index: u32, 
    eaten: u32
}

#[derive(Copy, Clone, Debug)]
struct Mosquito {
    position: u32, 
    value: u32, 
}

fn frogs_and_mosquitoes(mut frogs: Vec<Frog>, mosquitoes: Vec<Mosquito>) {
    let mut axis: BTreeMap<u32, Frog> = BTreeMap::new();
    let mut mosquito_to_eat: BTreeMap<u32, Mosquito> = BTreeMap::new();
    let mut leftover_mosquitoes: Vec<Mosquito> = Vec::new();
    let mut res: HashMap<u32, Frog> = HashMap::new();

    frogs.sort_by_key(|f: &Frog| f.position);

    let mut current_reach = frogs[0].reach;
    axis.insert(frogs[0].position, frogs[0]);

    for frog in frogs.iter().skip(1) {
        if frog.reach > current_reach {
            axis.insert(std::cmp::max(current_reach + 1, frog.position), *frog);
            current_reach = frog.reach;
        } else {
            axis.insert(frog.position, *frog);
        }
    }

    for mosquito in mosquitoes {
        let mut count = 0;
        for (frog_pos, frog) in axis.range(..(mosquito.position + 1)) {
            let mut eating_frog = *frog;
            if eating_frog.reach < mosquito.position {
                count += 1;
                if count == axis.range(..(mosquito.position + 1)).count() {
                    leftover_mosquitoes.push(mosquito);
                }
                continue;
            }

            eating_frog.eaten += 1; 
            eating_frog.reach += mosquito.value;
            mosquito_to_eat.remove(&mosquito.position);

            let mut again = true;
            while again {
                again = false;
                for i in 0..leftover_mosquitoes.len() {
                    if leftover_mosquitoes[i].position <= eating_frog.reach {
                        eating_frog.eaten += 1;
                        eating_frog.reach += leftover_mosquitoes[i].value;
                        leftover_mosquitoes.remove(i);
                        again = true;
                        break; // Break to prevent mutable borrow conflict
                    }
                }
            }

            res.insert(eating_frog.position, eating_frog);
        }
    }

    let mut res_array: Vec<Frog> = res.values().cloned().collect();
    res_array.sort_by_key(|f| f.index);

    for frog in res_array {
        println!("{} {}", frog.eaten, frog.reach - frog.position);
    }
}

fn main() {
    // test 1
    let f1 = Frog { position: 10, reach: 12, index: 0, eaten: 0 };
    let f2 = Frog { position: 15, reach: 15, index: 1, eaten: 0 };
    let f3 = Frog { position: 6, reach: 7, index: 2, eaten: 0 };
    let f4 = Frog { position: 0, reach: 1, index: 3, eaten: 0 };

    let frogs: Vec<Frog> = vec![f1, f2, f3, f4];

    let m1 = Mosquito { position: 110, value: 10 };
    let m2 = Mosquito { position: 1, value: 1 };
    let m3 = Mosquito { position: 6, value: 0 };
    let m4 = Mosquito { position: 15, value: 10 };
    let m5 = Mosquito { position: 14, value: 100 };
    let m6 = Mosquito { position: 12, value: 2 };

    let mosquitoes: Vec<Mosquito> = vec![m1, m2, m3, m4, m5, m6];

    frogs_and_mosquitoes(frogs, mosquitoes);

    // test 2
    let f1 = Frog { position: 10, reach: 12, index: 0, eaten: 0 };
    let frogs1: Vec<Frog> = vec![f1];

    let m1 = Mosquito { position: 20, value: 2 };
    let m2 = Mosquito { position: 12, value: 1 };
    let mosquitoes1: Vec<Mosquito> = vec![m1, m2];

    frogs_and_mosquitoes(frogs1, mosquitoes1);
}
