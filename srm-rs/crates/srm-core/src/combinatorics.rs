//! Outils combinatoires sur des ensembles représentés par des bitmasks `u64`.
//!
//! Convention dans tout le crate : le bit `i` représente le numéro `i + 1`.
//! Les 5-sous-ensembles sont indexés par leur rang dans l'ordre colexicographique,
//! qui coïncide avec l'ordre croissant des entiers (et donc avec l'ordre de Gosper).

/// Nombre de numéros dans une grille.
pub const GRID_SIZE: usize = 5;

/// Plus grand numéro jouable.
pub const MAX_NUMBER: u8 = 50;

/// Triangle de Pascal `C(n, k)` pour `n ≤ 64` et `k ≤ 5`, calculé à la compilation.
static BINOM: [[u64; GRID_SIZE + 1]; 65] = build_binom();

const fn build_binom() -> [[u64; GRID_SIZE + 1]; 65] {
    let mut t = [[0u64; GRID_SIZE + 1]; 65];
    let mut n = 0;
    while n < 65 {
        t[n][0] = 1;
        let mut k = 1;
        while k <= GRID_SIZE {
            if n > 0 {
                t[n][k] = t[n - 1][k - 1] + t[n - 1][k];
            }
            k += 1;
        }
        n += 1;
    }
    t
}

/// Coefficient binomial `C(n, k)` quelconque (calcul exact en `u128`).
pub fn binomial(n: u64, k: u64) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut r: u128 = 1;
    for i in 0..k {
        // r * (n - i) est toujours divisible par (i + 1) : produit de i+1 entiers consécutifs.
        r = r * u128::from(n - i) / u128::from(i + 1);
    }
    r
}

/// `C(n, k)` rapide pour `k ≤ 5`, `n ≤ 64`.
#[inline]
pub fn small_binomial(n: usize, k: usize) -> u64 {
    BINOM[n][k]
}

/// Rang colexicographique d'un 5-sous-ensemble : `Σ C(p_i, i + 1)` pour les positions triées `p_i`.
#[inline]
pub fn rank(mask: u64) -> u32 {
    debug_assert_eq!(mask.count_ones() as usize, GRID_SIZE);
    let mut m = mask;
    let mut r = 0u64;
    let mut i = 1;
    while m != 0 {
        r += BINOM[m.trailing_zeros() as usize][i];
        i += 1;
        m &= m - 1;
    }
    r as u32
}

/// Inverse de [`rank`].
pub fn unrank(mut r: u64) -> u64 {
    let mut mask = 0u64;
    let mut p = 64usize;
    for i in (1..=GRID_SIZE).rev() {
        p -= 1;
        while BINOM[p][i] > r {
            p -= 1;
        }
        r -= BINOM[p][i];
        mask |= 1 << p;
    }
    mask
}

/// Successeur de `x` parmi les entiers ayant le même nombre de bits à 1 (astuce de Gosper).
#[inline]
pub fn next_combination(x: u64) -> u64 {
    let c = x.isolate_lowest_one();
    let r = x + c;
    (((r ^ x) >> 2) / c) | r
}

/// Itère sur tous les 5-sous-ensembles de `{1..n}`, dans l'ordre des rangs.
pub fn all_grids(n: u8) -> impl Iterator<Item = u64> {
    let limit = 1u64 << n;
    let first = if usize::from(n) >= GRID_SIZE {
        (1u64 << GRID_SIZE) - 1
    } else {
        limit
    };
    std::iter::successors(Some(first), |&x| Some(next_combination(x)))
        .take_while(move |&x| x < limit)
}

/// Masque `{1..n}`.
#[inline]
pub fn full_mask(n: u8) -> u64 {
    if n >= 64 { u64::MAX } else { (1u64 << n) - 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binomial_known_values() {
        assert_eq!(binomial(50, 5), 2_118_760);
        assert_eq!(binomial(20, 5), 15_504);
        assert_eq!(binomial(30, 5), 142_506);
        assert_eq!(binomial(5, 6), 0);
        assert_eq!(binomial(0, 0), 1);
        assert_eq!(binomial(49, 6), 13_983_816);
        for n in 0..=64 {
            for k in 0..=5 {
                assert_eq!(
                    u128::from(small_binomial(n, k)),
                    binomial(n as u64, k as u64)
                );
            }
        }
    }

    #[test]
    fn enumeration_matches_binomial() {
        for n in 5..=20u8 {
            assert_eq!(all_grids(n).count() as u128, binomial(u64::from(n), 5));
        }
        assert_eq!(all_grids(4).count(), 0);
    }

    #[test]
    fn rank_is_position_in_enumeration_and_unrank_inverts_it() {
        for n in [5u8, 8, 12] {
            for (i, mask) in all_grids(n).enumerate() {
                assert_eq!(rank(mask) as usize, i);
                assert_eq!(unrank(i as u64), mask);
            }
        }
        let last = unrank(binomial(50, 5) as u64 - 1);
        assert_eq!(last, 0b11111 << 45);
    }
}
