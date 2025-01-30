use std::collections::HashSet;

pub type Clique<Node> = Vec<Node>;

pub fn bron_kerbosch_impl<'a, 'b, Node, F>(
    r: &mut Clique<Node>,
    p: HashSet<Node>,
    x: HashSet<Node>,
    neighbours: F,
) -> Vec<Clique<Node>>
where
    'a: 'b,
    Node: Clone + Eq + std::hash::Hash + 'a,
    F: for<'n> Fn(&'n Node) -> &'b HashSet<Node> + Clone,
{
    if p.is_empty() && x.is_empty() {
        return vec![r.clone()];
    }
    let mut cliques = Vec::new();
    for v in p.iter() {
        r.push(v.clone());
        let n = neighbours(v);
        let new_p = if n.len() < p.len() {
            n.intersection(&p).cloned().collect()
        } else {
            p.intersection(&n).cloned().collect()
        };
        let new_x = if n.len() < x.len() {
            n.intersection(&x).cloned().collect()
        } else {
            x.intersection(&n).cloned().collect()
        };
        cliques.extend(bron_kerbosch_impl(r, new_p, new_x, neighbours.clone()));
        r.pop();
    }
    cliques
}

pub fn bron_kerbosch<'a, 'b, Node, F>(nodes: HashSet<Node>, neighbours: F) -> Vec<Clique<Node>>
where
    'a: 'b,
    Node: Clone + Eq + std::hash::Hash + 'a,
    F: for<'n> Fn(&'n Node) -> &'b HashSet<Node> + 'a,
{
    let cliques = bron_kerbosch_impl(&mut vec![], nodes, HashSet::new(), &neighbours);
    cliques
}

pub fn max_bron_kerbosch_impl<'a, 'b, Node, F>(
    r: &mut Clique<Node>,
    p: HashSet<Node>,
    x: HashSet<Node>,
    neighbours: F,
) -> Option<Clique<Node>>
where
    'a: 'b,
    Node: Clone + Eq + std::hash::Hash + 'a,
    F: for<'n> Fn(&'n Node) -> &'b HashSet<Node> + Clone,
{
    if p.is_empty() && x.is_empty() {
        return Some(r.clone());
    }
    let mut max_clique: Option<Clique<Node>> = None;
    // let checked = p.iter();
    let pivot_neighs = p
        .iter()
        .chain(x.iter())
        .map(|v| neighbours(v))
        .max_by_key(|n| n.len())?;
    for v in p.difference(pivot_neighs) {
        r.push(v.clone());
        let n = neighbours(v);
        let new_p = n.intersection(&p).cloned().collect();
        let new_x = n.intersection(&x).cloned().collect();
        if let Some(clique) = max_bron_kerbosch_impl(r, new_p, new_x, neighbours.clone()) {
            match max_clique {
                Some(ref c) if c.len() >= clique.len() => {}
                _ => max_clique = Some(clique),
            }
        }
        r.pop();
    }
    max_clique
}

pub fn max_bron_kerbosch<'a, 'b, Node, F>(
    nodes: HashSet<Node>,
    neighbours: F,
) -> Option<Clique<Node>>
where
    'a: 'b,
    Node: Clone + Eq + std::hash::Hash + 'a,
    F: for<'n> Fn(&'n Node) -> &'b HashSet<Node> + 'a,
{
    max_bron_kerbosch_impl(&mut vec![], nodes, HashSet::new(), &neighbours)
}
