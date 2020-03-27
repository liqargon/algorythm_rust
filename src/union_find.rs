struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).collect::<Vec<usize>>(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let px = self.par[x];
            self.par[x] = self.find(px);
            self.par[x]
        }
    }

    fn unite(&mut self, x: usize, y:usize) {
        let rx = self.find(x);
        let ry = self.find(y);

        if rx != ry{
            if self.rank[rx] > self.rank[ry] {
                self.par[ry] = rx;
            }else{
                self.par[rx] = ry;
                if self.rank[rx] == self.rank[ry] {self.rank[ry] += 1}
            }
        }
    }

    fn same(&mut self, x: usize, y:usize) -> bool {
        self.find(x) == self.find(y)
    }
}


fn main() {
    let mut unf = UnionFind::new(3);
    unf.unite(0, 1);
    if unf.same(0, 2){
        println!("test");
    }

}