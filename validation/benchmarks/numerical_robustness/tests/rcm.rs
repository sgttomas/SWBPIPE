//! RCM equality (K4's brief Q7; plan §11): K4's exported port and
//! `sparse_direct`'s RCM give the same order on every CI model's free–free
//! adjacency and on seeded synthetic graphs.
use piping_numerical_robustness::cases::load_all;
use piping_numerical_robustness::rcm::{both_orders, free_adjacency};

#[test]
fn k4s_rcm_equals_sparse_directs_on_every_ci_models_adjacency() {
    let mut n = 0;
    for c in load_all().iter().filter(|c| !c.is_large()) {
        let adjacency = free_adjacency(c.model.as_ref().unwrap());
        let (k4, sd) = both_orders(&adjacency);
        assert_eq!(k4, sd, "{}", c.id);
        n += 1;
    }
    assert_eq!(n, 201);
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

#[test]
fn k4s_rcm_equals_sparse_directs_on_seeded_synthetic_graphs() {
    let mut rng = Rng(0x564B_5243_4D00_0001);
    let mut shapes = Vec::new();
    // A path, a star, a grid, an empty graph and isolated nodes.
    shapes.push(
        (0..9)
            .map(|k| if k < 8 { vec![k + 1] } else { vec![] })
            .collect::<Vec<_>>(),
    );
    shapes.push(
        (0..7)
            .map(|k| if k == 0 { (1..7).collect() } else { vec![] })
            .collect(),
    );
    shapes.push(
        (0..16)
            .map(|k: usize| {
                let mut v = Vec::new();
                if k % 4 < 3 {
                    v.push(k + 1);
                }
                if k < 12 {
                    v.push(k + 4);
                }
                v
            })
            .collect(),
    );
    shapes.push(vec![vec![]; 5]);
    for _ in 0..400 {
        let n = 1 + (rng.next() % 60) as usize;
        let edges = (rng.next() % (3 * n as u64 + 1)) as usize;
        let mut adjacency = vec![Vec::new(); n];
        for _ in 0..edges {
            let a = (rng.next() % n as u64) as usize;
            let b = (rng.next() % n as u64) as usize;
            // Self edges and duplicates included; one direction only at times.
            adjacency[a].push(b);
            if rng.next() % 2 == 0 {
                adjacency[b].push(a);
            }
        }
        shapes.push(adjacency);
    }
    for (k, adjacency) in shapes.iter().enumerate() {
        let (k4, sd) = both_orders(adjacency);
        assert_eq!(k4, sd, "graph {k}: {adjacency:?}");
    }
}
