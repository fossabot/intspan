use intspan;
use intspan::IntSpan;

fn main() {
    let mut intspan = IntSpan::new();
    intspan.add_pair(1, 5);

    println!("{}", intspan);
    println!("{}", intspan.is_empty());
    println!("{}", intspan.edge_size());
    println!("{}", intspan.span_size());
    println!("{}", intspan.cardinality());

    intspan.add_n(9);
    intspan.add_vec(vec![12, 16, 15, 15, 20]);
    println!("{}", intspan);
    println!("{:?}", intspan.ranges());
}
