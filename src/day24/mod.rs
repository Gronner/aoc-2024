use std::io::Write;
use std::{fs::File, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
#[allow(unused)]
use itertools::Itertools;

type Num = usize;
type Output = Num;
type Output2 = String;
type Input = (FxHashMap<String, Num>, FxHashMap<String, Gate>);

#[derive(Debug)]
pub struct Gate {
    pub out: String,
    pub in1: String,
    pub in2: String,
    pub op_name: String,
    op: fn(Num, Num) -> Num,
}

impl Gate {
    fn execute(&self, inputs: &mut FxHashMap<String, Num>, gates: &FxHashMap<String, Gate>) -> Num {
        if let Some(output) = inputs.get(&self.out) {
            *output
        } else {
            let inp1 = if let Some(inp1) = inputs.get(&self.in1) {
                *inp1
            } else {
                gates
                    .get(&self.in1)
                    .unwrap_or_else(|| panic!("Gate {} not found", self.in1))
                    .execute(inputs, gates)
            };
            let inp2 = if let Some(inp2) = inputs.get(&self.in2) {
                *inp2
            } else {
                gates
                    .get(&self.in2)
                    .unwrap_or_else(|| panic!("Gate {} not found", self.in2))
                    .execute(inputs, gates)
            };
            let output = (self.op)(inp1, inp2);
            inputs
                .entry(self.out.clone())
                .and_modify(|_| {
                    unreachable!("Tried to enter a different value for an existin gate")
                })
                .or_insert(output);
            output
        }
    }

    /// Outputs the gates connection in `dot` format
    pub fn print_inputs(&self, gates: &FxHashMap<String, Gate>) -> String {
        let mut output = if let Some(inp1) = gates.get(&self.in1) {
            inp1.print_inputs(gates)
        } else {
            format!("{}[shape=triangle];\n", self.in1)
        };
        output.push_str(&if let Some(inp2) = gates.get(&self.in2) {
            inp2.print_inputs(gates)
        } else {
            format!("{}[shape=triangle];\n", self.in2)
        });
        output.push_str(&format!(
            "{}[shape={}];\n{} -> {};\n{} -> {};\n",
            self.out,
            self.select_style(),
            self.in1,
            self.out,
            self.in2,
            self.out
        ));
        output
    }

    fn select_style(&self) -> &'static str {
        match self.op_name.as_str() {
            "OR" => "diamond",
            "XOR" => "trapezium",
            "AND" => "circle",
            _ => unreachable!(),
        }
    }
}

fn or(lhs: Num, rhs: Num) -> Num {
    lhs | rhs
}

fn xor(lhs: Num, rhs: Num) -> Num {
    lhs ^ rhs
}

fn and(lhs: Num, rhs: Num) -> Num {
    lhs & rhs
}

impl FromStr for Gate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            regex!(r"(?<in1>[\w\d]{3}) (?<gate>OR|AND|XOR) (?<in2>[\w\d]{3}) -> (?<out>[\w\d]{3})");
        let captured = re.captures(s).unwrap();
        let in1 = captured.name("in1").unwrap().as_str().to_owned();
        let in2 = captured.name("in2").unwrap().as_str().to_owned();
        let out = captured.name("out").unwrap().as_str().to_owned();
        let op_name = captured.name("gate").unwrap().as_str().to_owned();
        let op = match op_name.as_str() {
            "XOR" => xor,
            "OR" => or,
            "AND" => and,
            g => panic!("Unexpected gate type: {g}"),
        };

        Ok(Self {
            out,
            in1,
            in2,
            op_name,
            op,
        })
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Input {
    let (inputs, gates) = input.split_once("\n\n").unwrap();
    let inputs = inputs
        .lines()
        .map(|line| (line.split_once(": ").unwrap()))
        .map(|(gid, inp)| (gid.to_owned(), inp.parse::<Num>().unwrap()))
        .collect();
    let gates = gates
        .lines()
        .map(|line| Gate::from_str(line).unwrap())
        .map(|gate| (gate.out.clone(), gate))
        .collect();
    (inputs, gates)
}

fn compute_bits(inputs: &FxHashMap<String, usize>, gates: &FxHashMap<String, Gate>) -> Vec<Num> {
    let mut inputs = inputs.clone();
    gates
        .keys()
        .filter(|gate| gate.starts_with("z"))
        .sorted()
        .rev()
        .map(|gate| gates[gate].execute(&mut inputs, gates))
        .collect()
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &Input) -> Output {
    let (inputs, gates) = input;
    compute_bits(inputs, gates)
        .iter()
        .fold(0, |out, bit| (out << 1) | bit)
}

fn into_num(prefix: char, inputs: &FxHashMap<String, usize>) -> Num {
    inputs
        .iter()
        .filter(|(key, _)| key.starts_with(prefix))
        .sorted_by(|(key_a, _), (key_b, _)| key_a.cmp(key_b))
        .rev()
        .fold(0, |out, (_, val)| (out << 1) | val)
}

fn save_as_dot(
    inputs: &FxHashMap<String, usize>,
    gates: &FxHashMap<String, Gate>,
    rewire: &FxHashSet<&String>,
) {
    let gate_plan = gates
        .keys()
        .filter(|gate| gate.starts_with("z"))
        .map(|gate| gates[gate].print_inputs(gates))
        .collect::<String>();
    let mut gate_plan = gate_plan.lines().unique().join("\n");

    let x = into_num('x', inputs);

    let y = into_num('y', inputs);

    let expected_result = x + y;
    let actual_result = compute_bits(inputs, gates)
        .iter()
        .fold(0, |out, val| (out << 1) | val);

    let broken_outs = (0..46)
        .enumerate()
        .filter(|(_, shift)| ((expected_result >> shift) & 1) != ((actual_result >> shift) & 1))
        .map(|(gate, _)| format!("z{gate:0>2}"))
        .collect::<Vec<String>>();

    gate_plan.push_str(
        &broken_outs
            .iter()
            .map(|broken_gate| format!("{}[color=red]", broken_gate))
            .join("\n"),
    );

    gate_plan.push_str(
        &rewire
            .iter()
            .map(|wire| format!("{wire}[style=filled];"))
            .join("\n"),
    );
    let mut dot = String::from("digraph {\n");
    dot.extend(gate_plan.lines().map(|line| format!("\t{}\n", line)));
    dot.push('}');

    let mut file = File::create("gate_network.dot").unwrap();
    file.write_all(&dot.into_bytes()).unwrap();
}

#[aoc(day24, part2)]
/// Check whether everything is wired up to be a
/// [Carry-Ripple-Adder](https://de.wikipedia.org/wiki/Carry-Ripple-Addierer) with a half adder
/// for the last bit.
/// Addition is $y \oplus{} x \oplus{} c$
/// Looking at the input the following happens should happen for each bit of y and x:
///
/// * $x_n$ = current bit of x
/// * $y_n$ = current bit of y
/// * $c_n$ = carry bit of $x_n$ + $y_n$ and stems from an
/// * $c_{np}$ = Carry bit of the previous iteration
/// * $z_n$ = current bit of z
///
/// $$z_n = ((x_n \oplus{} y_n) \oplus{} c_{np})$$
///
/// $$c_n = ((c_{np} \wedge{} (x_n \oplus{} y_n)) \lor{} (x_n \wedge{} y_n))$$
///
/// $$z_n + c_n = ( c_n << 1) \lor{} z_n $$
///
/// Check for computations where these rules are violated
///
/// A representation of the gate network is stored in the
/// [dot](https://graphviz.org/doc/info/lang.html) language in
/// `./target/aoc/aoc-autobuild/gate_network.dot` file. Using dot it can be rendered to a file
/// (e.g., by calling `dot -Tsvg gate_network.dot > gate_network.map`) where:
/// * $\oplus{}$ is rendered as a trapezoid
/// * $\lor{}$ is rendered as a diamond
/// * $\wedge{}$ is rendered as a circle
/// * Output (`zxx`) nodes with red borders are nodes outputing the wrong bit for the computation
/// of $x + y$.
/// * Nodes that are filled (either grey or red) are nodes which need to be swapped for the correct
/// result.
pub fn solve_part2(input: &Input) -> Output2 {
    let (inputs, gates) = input;

    let mut rewire = FxHashSet::default();

    for (name, gate) in gates {
        // All but the last output gate (which is the last carry bit and should be an OR) need to be XORs
        if name.starts_with('z') && gate.op_name != "XOR" && name != "z45" {
            rewire.insert(name);
        }
        // All XOR gates that output z must not be fed by x or y as there needs to be an
        // intermediate XOR with the carry bit
        if gate.op_name == "XOR"
            && !name.starts_with('z')
            // Does not happen in my input
            && !['x', 'y'].contains(&gate.in1.chars().next().unwrap())
            // Does not happen in my input
            && !['x', 'y'].contains(&gate.in2.chars().next().unwrap())
        {
            rewire.insert(name);
        }
        // All AND gates except the one accepting the initial x_0 value (there is no carry bit yet)
        // must feed into an OR
        if gate.op_name == "AND" && ![&gate.in1, &gate.in2].contains(&&"x00".to_owned()) {
            for other_gate in gates.values() {
                if [&other_gate.in1, &other_gate.in2].contains(&name) && other_gate.op_name != "OR"
                {
                    rewire.insert(name);
                }
            }
        }
        // All XOR gates must feed into either an XOR or an AND
        if gate.op_name == "XOR" {
            for other_gate in gates.values() {
                if [&other_gate.in1, &other_gate.in2].contains(&name)
                    && !(other_gate.op_name == "XOR" || other_gate.op_name == "AND")
                {
                    rewire.insert(name);
                }
            }
        }
    }

    save_as_dot(inputs, gates, &rewire);
    rewire.iter().sorted().join(",")
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(&input_generator(input))
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(&input_generator(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample1() -> &'static str {
        "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"
    }

    fn sample2() -> &'static str {
        "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
    }

    #[test]
    fn samples1_part1() {
        assert_eq!(4, solve_part1(&input_generator(sample1())));
    }

    #[test]
    fn samples2_part1() {
        assert_eq!(2024, solve_part1(&input_generator(sample2())));
    }

    #[test]
    fn samples1_part2() {
        assert!(true)
    }
}
