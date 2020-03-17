An implementation of Union-Find in Rust. Includes the path-compression and
join-by-rank optimizations, making it a linear time-algorithm in the real
world.

Create a new UnionFind data structure with the new keyword.  In this example,
the capacity is set to 10 items:

    let mut v = UnionFind::new(10);

Tell the data structure that two nodes are in the same network. The algorithm
doesn't assign an ID automatically or anything like that. So if your nodes are,
for instance, strings, then you'll need to hash and mod the string to get an ID
between 0 and your capacity, or assign the IDs individually and store the
result in a hashmap.

        v.union(0, 1);

Return whether two nodes are in the same set:

        v.cnx(5, 0);
