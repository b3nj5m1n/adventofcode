/*
* Was too lazy to actually implement [Kargers](https://en.wikipedia.org/wiki/Karger%27s_algorithm)
* So instead I generated the graph, exported it to DOT, created the SVG from that, used inkscape
* to look for which nodes are connected to the 3 edges in the middle, then manually removed those
* edges in the code, finally counted how many nodes are in each of the sub-graphs.

                                             ,:;iii;iii;;:,
                                         .:ii;:,. ....,:;;;ii;,
                                       .ii:.    ,::,.  ,,::i;:tt:
                                      it:     .i;:,   ,.. .,;,,:ft
                                     1C.     ,i;,,,.  .,:;;:.1;: if.
                                    1C;::   ;:.           .:i1:;: :f.
                                   :f,;;   ;;,:;;.           ,     ,t
                                   1::;,. :, .  ,.          .:ii,   1:
                                   t. :i::;1080L:         ;L0@@@@t  ,t
                                   t   .,f0@@@@@@Gt;,. .1G@@@@@@@@t  t
                                   t    C@@@@@@@@@@@80G0@@@@@@@@@@@; 1
                                   ;.  ,@@@@@@@@@8000888088@@@@@@@@G 1
                                   .1  :@@@@@@@@88008008008@@@@@@@@@:i
                                   i0t .@@@@@@@@@@@@8@8@@@@0Lt11L8@@L:
                     ,tt.       ...;80i C0000C111ttC@@08@@L;i1tft0@@0.
                    .0@@0;,::;;;;;;:G80LGGGGLtii;;::C8@@@8CL1t08@@@@f
                     0@@81,,.       ;8@@@@@@@800fC8fC0@8@G88CG8@@@@@;
                     i@@C,..,,:::;1;:1ffLC0888@@@@8LG@08@C@@@@@@@@@8,
                      t@L,..     :L,.t1,.....,;tf0@LC@GG@G808@@@@@@8.
                      18C..,:;iii. ..,:i.         ;iC888@@@G000@@@@0
                     .L0008@@@@@@0. .              G@CG00G08@80C8@@L
                    i0@@@@@@@88GC1               .L8880GG088G8@88@8,
                   ,@@80@@@0Ct1i;:ii1fCCG0C.   ,1080GG0088000@@@@L,
                    i@@@@8G00888@@@@@@@888L1f;L@@@@@@@@00008@@@0G;
                  .::8@@@@@@@@@88@@@80CCLLG8@C;fG@@@@@80G08@@@G.G@t
                 :i;if8@@8@@@@@8GCt:L00@@@@G1,,;;;1LG88@@@@8G1 .0@8L,
              ,;L1 1iC00@8@@@@@88@0,,G0LLf,    ;88L1:;iii;;:,;i;:, C8i,.
            .;;L; ;i;0@GG8@@@@8@@0Gtf1 ,i,      ,00L ....;;..:t    i@8i:;:,.
           :i:t:  f.;0@@808@@@@@@@@t;i :i.       .Lf     ii  .t    :@@8, .,::::,,.
          :i:t,   C..C@@@@@@@@@@@8fiit.,1.        ,C; .:;:;;;;i;;;iL@@@0.     .,::;:.
         .t:1.    it 1@@@@@@@@@0GL: .t,.1.         .fCt,.      ...1@@@@@f          ,;;.
         ,L;       t:.L@@@880L;ti1.  ,:.1:           L@.          C@@@@@@,           ,1,
         1i        .t ,G@@8f, i:,i   :,.:ti.          t0.        :@@@@@@@t            .1,
        1;          :i 1@0i .1: 1,..,,.  ,if:..        1C        C@@@@@@@G             .t
       t;            i,.Gi ,1. ,::,,,,,,,,,,:::,        iC.     ;@@@@@@@@0              ;;
      t;             1i,:;:i  .i,,,..........,1:         iG,    0@@@@@@@@0.             .1
     1i            .1; ,, .   :i ............ ,1;         ;G,  t@@@@@@@@@0               1
    ;f            ,f: :,      t,                i1.        ;G,:@@@@@@@@@@G        .;:    1.
   ,C.           ,1,i;.      :L                  ,i;        :G8@@@@@@@@@@C        :88G   1.
   f:           ,1;,,.       C:                    :i:       :8@@@@@@@@@@L      .,f80L,  1
  it           ,i ,:        t1                      .;i,      ,8@@@@@@@@@f     .;;i:    .1
 .L.           .  .        if                         ,1i.     :8@@@@@@@@:              .1
 t;                       ;1                            :1;     :@@@@@@@C               ,i
,f                       i;                               :1:    i@@@@@@,             . ,i
i:                     .t;                                  ;i,   t@@@@1              : .i
t,                    ;t:                                    .i;.  L@@G               ,  1
f;                  :f1.                                       ,;:  G@:               ,. 1.
,C,               ,ff,                                           ;i ,f                ,. i,
 ,fi.          ,itLf                                              ti,,                ,, ::
   ;iii;;;;i1fff1, t                                              i:i:                ..  ,

*/

use std::collections::HashMap;
use std::env;
use std::io::Read;

use petgraph::algo::connected_components;
use petgraph::dot::{Config, Dot};
use petgraph::prelude::UnGraph;
use petgraph::visit::Dfs;
use petgraph::Graph;

// Function to output the solutions to both parts
fn output(result: &Result) {
    println!("Part 1: {}", &result.part_1);
    println!("Part 2: {}", &result.part_2);
}

fn main() {
    // Vector of the command line arguments
    let args: Vec<String> = env::args().collect();

    // Read in the input
    let mut file_handle = std::fs::File::open(&args[1]).unwrap();
    let mut inp = String::new();
    file_handle.read_to_string(&mut inp).unwrap();
    let inp: Vec<&str> = inp.split("\n").filter(|line| !line.is_empty()).collect();

    // Struct storing the resulting values
    let mut result: Result = Result {
        part_1: 0,
        part_2: 0,
    };

    // Solve
    solve(inp, &mut result);
    // Output the solutions
    output(&result);
}

// Struct for solution values
struct Result {
    part_1: usize,
    part_2: usize,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    // let mut graph = Graph::<&str, u8>::new();
    let mut graph = UnGraph::<&str, u8>::new_undirected();
    let mut nodes = HashMap::new();
    for line in inp {
        let (cur, rest) = line.split_once(": ").expect("fuck");
        let cur_idx = match nodes.get(cur) {
            Some(idx) => *idx,
            None => {
                let idx = graph.add_node(cur);
                nodes.insert(cur, idx.clone());
                idx
            }
        };
        for neighbour in rest.split_whitespace() {
            let neigh_idx = match nodes.get(neighbour) {
                Some(idx) => *idx,
                None => {
                    let idx = graph.add_node(neighbour);
                    nodes.insert(neighbour, idx.clone());
                    idx
                }
            };
            graph.add_edge(cur_idx, neigh_idx, 1);
        }
    }

    let solutions_example = [["hfx", "pzl"], ["bvb", "cmg"], ["nvd", "jqt"]];
    let solutions_input = [["hvm", "grd"], ["zfk", "jmn"], ["pmn", "kdc"]];

    let solutions = solutions_input;

    let mut start_1 = None;
    let mut start_2 = None;
    for [to_remove_a, to_remove_b] in solutions {
        let to_remove_a = nodes.get(&to_remove_a).expect("Fucked up to_remove_a");
        let to_remove_b = nodes.get(&to_remove_b).expect("Fucked up to_remove_b");
        let (edge, _) = graph
            .find_edge_undirected(*to_remove_a, *to_remove_b)
            .expect("Didn't find edge");
        graph.remove_edge(edge);
        start_1 = Some(to_remove_a);
        start_2 = Some(to_remove_b);
    }

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let start_1 = start_1.unwrap();
    let start_2 = start_2.unwrap();

    let mut dfs = Dfs::new(&graph, *start_1);
    let mut len_graph_1 = 0;
    while let Some(_) = dfs.next(&graph) {
        len_graph_1 += 1;
    }

    let mut dfs = Dfs::new(&graph, *start_2);
    let mut len_graph_2 = 0;
    while let Some(nx) = dfs.next(&graph) {
        len_graph_2 += 1;
    }

    // dbg!(connected_components(&graph));

    res.part_1 = len_graph_1 * len_graph_2;
}
