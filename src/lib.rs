/// Data structure to implement the Union Find
/// algorithm. UnionFind.id stores a tree: the
/// id[n] stores the parent node of n.
///
/// The parent of id[n] is stored in id[id[n]]. The
/// root node is the node where the parent is itself.
/// That is, if the ID for node 0 is 0, then 0 must
/// be the root because it has no other parent. The
/// set ID is the the ID of the root node.
///
/// UnionFind.sz stores the number of hops until the root
/// node is found. This data is kept in order to ensure
/// that when nodes are being combined into a single set,
/// the resulting trees are as flat as possible (fewest
/// hops to the root).
#[derive(Debug)]
pub struct UnionFind{
    id: Vec<usize>,  // id of parent node
    sz: Vec<usize>,  // number of hops to root
}


impl UnionFind {
    /// Create a brand new UnionFind data structure where
    /// every item is in its own, discrete set by default.
    #[allow(dead_code)]
    pub fn new(n: usize) -> UnionFind {
        let mut ids = Vec::with_capacity(n);
        for i in 0..n {
            ids.push(i);
        }
        let szs = vec![0; n];
        return UnionFind{id: ids, sz: szs};
    }

    /// Determine whether two nodes, p and q, are connected.
    #[allow(dead_code)]
    pub fn cnx(&mut self, p: usize, q: usize) -> bool {
        return self.find(p) == self.find(q);
    }

    /// Find the set ID of a node (the root of the tree).
    #[allow(dead_code)]
    pub fn find(&mut self, p: usize) -> usize {
        let mut i = p;
        while self.id[i] != i {
            self.id[i] = self.id[self.id[i]]; // flatten root while we're at it
            i = self.id[i];
        }
        return i;
    }

    /// Set the two nodes to be in the same set.
    #[allow(dead_code)]
    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);
        if i == j {
            return;
        } else if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += i;
        } else {
            self.id[j] = i;
            self.sz[i] += j;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = UnionFind::new(10);
        for i in 0..10 {
            assert_eq!(v.id[i], i);
        }
    }

    #[test]
    fn test_cnx_union() {
        let mut v = UnionFind::new(10);
        assert!(!v.cnx(0, 1));
        v.union(0, 1);
        assert!(v.cnx(0, 1));
        assert!(!v.cnx(0, 2));
        v.union(1, 2);
        assert!(v.cnx(0, 1));
        assert!(v.cnx(0, 2));
        v.union(3, 4);
        assert!(v.cnx(3, 4));
        v.union(4, 5);
        v.union(3, 2);
        assert!(v.cnx(5, 0));
    }
}
