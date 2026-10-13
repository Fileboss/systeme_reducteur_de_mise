//! Glouton « meilleure couverture » : à chaque étape, on prend la grille qui couvre le plus
//! de tirages encore non couverts.
//!
//! Les gains ne font que décroître ; on les range dans une file à seaux (structure de
//! Batagelj–Zaveršnik) : tableau trié par gain croissant, chaque décrément est un échange
//! en O(1) et le maximum est toujours le dernier élément.

use rand::Rng;
use rand::seq::SliceRandom;

use crate::space::Space;

/// Renvoie les masques des grilles choisies. Les égalités de gain sont départagées par `rng`.
pub fn greedy<R: Rng>(space: &Space, rng: &mut R) -> Vec<u64> {
    let len = space.len();
    let degree = space.degree();

    let mut gain = vec![degree as u32; len];
    let mut order: Vec<u32> = (0..len as u32).collect();
    order.shuffle(rng);
    let mut pos = vec![0u32; len];
    for (i, &v) in order.iter().enumerate() {
        pos[v as usize] = i as u32;
    }
    // bucket_start[g] = premier indice de `order` dont le gain vaut g.
    let mut bucket_start = vec![0u32; degree + 2];
    bucket_start[degree + 1] = len as u32;

    let mut covered = vec![false; len];
    let mut chosen = Vec::new();
    let mut newly_covered = Vec::with_capacity(degree);

    loop {
        let best = order[len - 1];
        if gain[best as usize] == 0 {
            break;
        }
        let best_mask = space.mask(best);
        chosen.push(best_mask);

        newly_covered.clear();
        space.for_each_neighbor(best_mask, |d, m| {
            if !covered[d as usize] {
                covered[d as usize] = true;
                newly_covered.push(m);
            }
        });
        for &dm in &newly_covered {
            space.for_each_neighbor(dm, |c, _| {
                let c = c as usize;
                let g = gain[c] as usize;
                let first = bucket_start[g];
                let w = order[first as usize];
                let pc = pos[c];
                order.swap(first as usize, pc as usize);
                pos[w as usize] = pc;
                pos[c] = first;
                bucket_start[g] += 1;
                gain[c] -= 1;
            });
        }
    }
    chosen
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;
    use crate::cover::verify;
    use crate::grid::Grid;

    fn to_grids(masks: &[u64]) -> Vec<Grid> {
        masks.iter().map(|&m| Grid::from_mask(m).unwrap()).collect()
    }

    #[test]
    fn greedy_produces_valid_covers() {
        for n in [6usize, 8, 11, 14] {
            for t in 2..=4u8 {
                let space = Space::new(n, t).unwrap();
                let sol = greedy(&space, &mut ChaCha8Rng::seed_from_u64(1));
                verify(n, t, &to_grids(&sol)).unwrap_or_else(|e| panic!("n={n} t={t}: {e}"));
            }
        }
    }

    #[test]
    fn greedy_is_reproducible() {
        let space = Space::new(12, 3).unwrap();
        let a = greedy(&space, &mut ChaCha8Rng::seed_from_u64(42));
        let b = greedy(&space, &mut ChaCha8Rng::seed_from_u64(42));
        assert_eq!(a, b);
    }

    #[test]
    fn greedy_beats_legacy_on_20_3() {
        // L'ancienne version Java produisait 60 grilles pour 20 numéros / garantie 3.
        let space = Space::new(20, 3).unwrap();
        let sol = greedy(&space, &mut ChaCha8Rng::seed_from_u64(7));
        verify(20, 3, &to_grids(&sol)).unwrap();
        assert!(sol.len() <= 60, "{} grilles", sol.len());
    }
}
