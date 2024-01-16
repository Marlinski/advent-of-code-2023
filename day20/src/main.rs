/*
--- Day 20: Pulse Propagation ---
With your help, the Elves manage to find the right parts and fix all of the machines. Now, they just need to send the command to boot up the machines and get the sand flowing again.

The machines are far apart and wired together with long cables. The cables don't connect to the machines directly, but rather to communication modules attached to the machines that perform various initialization tasks and also act as communication relays.

Modules communicate using pulses. Each pulse is either a high pulse or a low pulse. When a module sends a pulse, it sends that type of pulse to each module in its list of destination modules.

There are several different types of modules:

Flip-flop modules (prefix %) are either on or off; they are initially off. If a flip-flop module receives a high pulse, it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, it flips between on and off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.

Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules; they initially default to remembering a low pulse for each input. When a pulse is received, the conjunction module first updates its memory for that input. Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.

There is a single broadcast module (named broadcaster). When it receives a pulse, it sends the same pulse to all of its destination modules.

Here at Desert Machine Headquarters, there is a module with a single button on it called, aptly, the button module. When you push the button, a single low pulse is sent directly to the broadcaster module.

After pushing the button, you must wait until all pulses have been delivered and fully handled before pushing it again. Never push the button if modules are still processing pulses.

Pulses are always processed in the order they are sent. So, if a pulse is sent to modules a, b, and c, and then module a processes its pulse and sends more pulses, the pulses sent to modules b and c would have to be handled first.

The module configuration (your puzzle input) lists each module. The name of the module is preceded by a symbol identifying its type, if any. The name is then followed by an arrow and a list of its destination modules. For example:

broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
In this module configuration, the broadcaster has three destination modules named a, b, and c. Each of these modules is a flip-flop module (as indicated by the % prefix). a outputs to b which outputs to c which outputs to another module named inv. inv is a conjunction module (as indicated by the & prefix) which, because it has only one input, acts like an inverter (it sends the opposite of the pulse type it receives); it outputs to a.

By pushing the button once, the following pulses are sent:

button -low-> broadcaster
broadcaster -low-> a
broadcaster -low-> b
broadcaster -low-> c
a -high-> b
b -high-> c
c -high-> inv
inv -low-> a
a -low-> b
b -low-> c
c -low-> inv
inv -high-> a
After this sequence, the flip-flop modules all end up off, so pushing the button again repeats the same sequence.

Here's a more interesting example:

broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
This module configuration includes the broadcaster, two flip-flops (named a and b), a single-input conjunction module (inv), a multi-input conjunction module (con), and an untyped module named output (for testing purposes). The multi-input conjunction module con watches the two flip-flop modules and, if they're both on, sends a low pulse to the output module.

Here's what happens if you push the button once:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -high-> output
b -high-> con
con -low-> output
Both flip-flops turn on and a low pulse is sent to output! However, now that both flip-flops are on and con remembers a high pulse from each of its two inputs, pushing the button a second time does something different:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output
Flip-flop a turns off! Now, con remembers a low pulse from module a, and so it sends only a high pulse to output.

Push the button a third time:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -low-> output
b -low-> con
con -high-> output
This time, flip-flop a turns on, then flip-flop b turns off. However, before b can turn off, the pulse sent to con is handled first, so it briefly remembers all high pulses for its inputs and sends a low pulse to output. After that, flip-flop b turns off, which causes con to update its state and send a high pulse to output.

Finally, with a on and b off, push the button a fourth time:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output
This completes the cycle: a turns off, causing con to remember only low pulses and restoring all modules to their original states.

To get the cables warmed up, the Elves have pushed the button 1000 times. How many pulses got sent as a result (including the pulses sent by the button itself)?

In the first example, the same thing happens every time the button is pushed: 8 low pulses and 4 high pulses are sent. So, after pushing the button 1000 times, 8000 low pulses and 4000 high pulses are sent. Multiplying these together gives 32000000.

In the second example, after pushing the button 1000 times, 4250 low pulses and 2750 high pulses are sent. Multiplying these together gives 11687500.

Consult your module configuration; determine the number of low pulses and high pulses that would be sent after pushing the button 1000 times, waiting for all pulses to be fully handled after each push of the button. What do you get if you multiply the total number of low pulses sent by the total number of high pulses sent?

--- Part Two ---
The final machine responsible for moving the sand down to Island Island has a module attached named rx. The machine turns on when a single low pulse is sent to rx.

Reset all modules to their default states. Waiting for all pulses to be fully handled after each button press, what is the fewest number of button presses required to deliver a single low pulse to the module named rx?


*/

use std::fs::read_to_string;
use std::collections::HashMap;
use std::process::Output;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Pulse {
    Low,
    High
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Broadcast(),
}

type OutputTable = HashMap<String, Vec<String>>;
type ModuleTable = HashMap<String, Module>;

fn parse(input: &str) -> (OutputTable, ModuleTable) {
    let mut output_table  = HashMap::new();
    let mut module_table  = HashMap::new();

    read_to_string(input).unwrap().split("\n")
     .for_each(|s| {
        let s = s.split(" -> ").collect::<Vec<_>>();
        let outputs = s[1].split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
        let mut name = s[0].to_string();

        if name.starts_with("%") {
            name = s[0][1..].to_string();
            module_table.insert(name.clone(), Module::FlipFlop(false));
        } else if name.starts_with("&") {
            name = s[0][1..].to_string();
            module_table.insert(name.clone(), Module::Conjunction(HashMap::new()));
        } else {
            module_table.insert(name.clone(), Module::Broadcast());
        }

        for o in outputs {
            output_table.entry(name.clone())
            .or_insert_with(|| vec![])
            .push(o.clone());
        }
     });

     output_table.iter().for_each(|(from,v)| {
        for to in v {
            if let Some(Module::Conjunction(inputs)) = module_table.get_mut(to) {
                inputs.insert(from.clone(), Pulse::Low);
            }
        }
     });

     (output_table, module_table)
}

fn press_button((output_table, module_table):  &mut (OutputTable, ModuleTable)) -> (u32,u32) {
    let (mut pulse_low, mut pulse_high) = (1,0);
    let to = "broadcaster".to_string();
    let mut pulses = output_table[&to].iter().map(|o| (&to, o, Pulse::Low)).collect::<Vec<(&String, &String, Pulse)>>();
    while pulses.len() > 0 {
        let (from, to, pulse) = pulses.remove(0);

        //println!("{} --{:?}--> {}", from, pulse, to);
        match pulse {
            Pulse::Low => pulse_low += 1,
            Pulse::High => pulse_high += 1
        }

        if let Some(module) = module_table.get_mut(to) {
            match (pulse, module) {
                (Pulse::High, Module::FlipFlop(_)) => {
                },
                (Pulse::Low, Module::FlipFlop(b)) => {
                    match *b {
                        false => output_table[to].iter().for_each(|output| pulses.push((to, output, Pulse::High))),
                        true => output_table[to].iter().for_each(|output| pulses.push((to, output, Pulse::Low)))
                    }
                    *b = !*b;
                },
                (pulse, Module::Conjunction(inputs)) => {
                    inputs.insert(from.clone(), pulse);
                    if inputs.iter().all(|(_,p)| *p == Pulse::High) {
                        output_table[to].iter().for_each(|output| pulses.push((to, output, Pulse::Low)));
                    } else {
                        output_table[to].iter().for_each(|output| pulses.push((to, output, Pulse::High)));
                    }
                },
                _ => {}
            }
        }
    }
    
    (pulse_low, pulse_high)
}

fn main() {
    let input = "day20/assets/input";    
    let mut configuration = parse(input);
    let sum = (0..1000).into_iter().fold((0,0), |acc,_| {
        let b = press_button(&mut configuration);
        (acc.0+b.0, acc.1+b.1)
    });
    println!("sum is {}",sum.0 * sum.1);

    // graphviz visualization
    for (name, module) in configuration.1.iter() {
        let s = match module {
            Module::FlipFlop(_) => name.clone() + &"[shape=circle]".to_string().clone(),
            Module::Conjunction(_) => name.clone() + &"[shape=rect]".to_string().clone(),
            Module::Broadcast() => name.clone(),
        };
        println!("{}",s);
    }
    for (from, to) in configuration.0.iter() {
        println!("{} -> {}",from, to.join(","));
    }

    
    for i in 0..1000000 {
        let Module::Conjunction(ins) = configuration.1.get("bb").unwrap() else { panic!("ayah") };
        if ins.iter().any(|(k,v)| *v == Pulse::High) {
            println!("{}: {:?}",i, ins);
        }
        press_button(&mut configuration);
    }
}