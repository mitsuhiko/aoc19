fn eval(code: &str, input: i64) -> Vec<i64> {
    let mut mem: Vec<i64> = code
        .trim()
        .split(',')
        .map(|op| op.parse().unwrap())
        .collect();

    fn arg(mem: &[i64], ip: usize, off: usize) -> i64 {
        let arg_modes = mem[ip] / 100;
        let val = mem[ip + off];
        match arg_modes / 10i64.pow((off - 1) as u32) % 10 {
            0 => mem[val as usize],
            1 => val,
            _ => panic!("wat"),
        }
    }

    fn put(mem: &mut [i64], ip: usize, off: usize, val: i64) {
        let out = mem[ip + off] as usize;
        mem[out] = val;
    }

    let mut out = vec![];
    let mut ip = 0;

    loop {
        match mem[ip] % 100 {
            1 => {
                let a = arg(&mem, ip, 1);
                let b = arg(&mem, ip, 2);
                put(&mut mem, ip, 3, a + b);
                ip += 4;
            }
            2 => {
                let a = arg(&mem, ip, 1);
                let b = arg(&mem, ip, 2);
                put(&mut mem, ip, 3, a * b);
                ip += 4;
            }
            3 => {
                put(&mut mem, ip, 1, input);
                ip += 2;
            }
            4 => {
                out.push(arg(&mem, ip, 1));
                ip += 2;
            }
            5 => {
                if arg(&mem, ip, 1) != 0 {
                    ip = arg(&mem, ip, 2) as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                if arg(&mem, ip, 1) == 0 {
                    ip = arg(&mem, ip, 2) as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                if arg(&mem, ip, 1) < arg(&mem, ip, 2) {
                    put(&mut mem, ip, 3, 1);
                } else {
                    put(&mut mem, ip, 3, 0);
                }
                ip += 4;
            }
            8 => {
                if arg(&mem, ip, 1) == arg(&mem, ip, 2) {
                    put(&mut mem, ip, 3, 1);
                } else {
                    put(&mut mem, ip, 3, 0);
                }
                ip += 4;
            }
            99 => {
                break;
            }
            _ => {
                panic!("this should not happen");
            }
        }
    }

    out
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {:?}", eval(input, 1));
    println!("part 2: {:?}", eval(input, 5));
}

#[test]
fn test_eval() {
    assert_eq!(
        eval(include_str!("../input.txt"), 1).last(),
        Some(&13_346_482)
    );
    assert_eq!(eval("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0), vec![0]);
    assert_eq!(
        eval("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 42),
        vec![1]
    );
    assert_eq!(eval("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0), vec![0]);
    assert_eq!(eval("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 42), vec![1]);
    assert_eq!(eval("3,9,8,9,10,9,4,9,99,-1,8", 8), vec![1]);
    assert_eq!(eval("3,9,8,9,10,9,4,9,99,-1,8", 42), vec![0]);
    assert_eq!(eval("3,9,7,9,10,9,4,9,99,-1,8", 7), vec![1]);
    assert_eq!(eval("3,9,7,9,10,9,4,9,99,-1,8", 8), vec![0]);
    assert_eq!(eval("3,3,1108,-1,8,3,4,3,99", 8), vec![1]);
    assert_eq!(eval("3,3,1108,-1,8,3,4,3,99", 7), vec![0]);
    assert_eq!(eval("3,3,1107,-1,8,3,4,3,99", 7), vec![1]);
    assert_eq!(eval("3,3,1107,-1,8,3,4,3,99", 8), vec![0]);
    assert_eq!(
        eval(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            7
        ),
        vec![999]
    );
    assert_eq!(
        eval(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            8
        ),
        vec![1000]
    );
    assert_eq!(
        eval(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            9
        ),
        vec![1001]
    );
}
