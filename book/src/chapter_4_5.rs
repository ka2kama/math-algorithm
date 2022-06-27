use anyhow::Result;

fn code_4_5_2() {}

#[derive(Debug)]
struct Dfs<'a> {
    graph: &'a [Vec<usize>],
    visited: Vec<bool>,
}

impl<'a> Dfs<'a> {
    fn run(graph: &'a [Vec<usize>]) -> bool {
        let mut self_ = Self {
            graph,
            visited: vec![false; graph.len() + 1],
        };

        self_.dfs(1);

        self_.visited.iter().all(|&has_visited| has_visited)
    }

    fn dfs(&mut self, pos: usize) {
        self.visited[pos] = true;
        for &i in &self.graph[pos] {
            if !self.visited[i] {
                self.dfs(i);
            }
        }
    }
}

#[test]
fn test01() -> Result<()> {
    let a = "👨‍👩‍👧‍👦";
    println!("{}", a);
    println!("{}", a.len());
    println!("{:?}", a.chars());
    println!("{}", a.chars().count());
    Ok(())
}
