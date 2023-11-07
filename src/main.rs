extern crate rev_lines;
use rev_lines::RevLines;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufWriter, Write},
};

macro_rules! zip {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), +) => (
        $x.iter().zip(
            zip!($($y), +))
    )
}

struct Oscillator {
    kcu21: f64,
    kcu22: f64,
    km3: f64,
    km4: f64,
    km5: f64,
    km6: f64,
    km7: f64,
    km8: f64,
    km9: f64,
    km18: f64,
    km19: f64,
    km20: f64,
    km21: f64,
    km22: f64,
    km23: f64,
    km24: f64,
    k6p: f64,
    k8p: f64,
    k9: f64,
}

type State = [f64; 15];

fn main() {
    let _path1 = "/home/kartonrealista/actual_code/praca_magisterska_model_26zmienny/ptau1000.csv";
    let _path2 =
        "/home/kartonrealista/actual_code/model_japonczykow/stezenia8.csv";
    let _path1win = r"C:\Users\admin\Desktop\MTHOMAS\x\model26zmienny\ptau.csv";
    let path2win =
        r"C:\Users\admin\Desktop\MTHOMAS\x\model_japonczykow\stezenia8.csv";

    let mut km = Oscillator {
        kcu21: 10.0_f64.powf(2.0),
        kcu22: 4.5,
        km3: 1.6,
        km4: 40.0,
        km5: 1.5 * 10.0_f64.powf(-3.0),
        km6: 0.3,
        km7: 5.0 * 10.0_f64.powf(2.0),
        km8: 4.0 * 10.0_f64.powf(4.0),
        km9: 40.0,
        km18: 7.0 * 10.0_f64.powf(4.0),
        km19: 8.0 * 10.0_f64.powf(7.0),
        km20: 1.2 * 10.0_f64.powf(7.0),
        km21: 4.0 * 10.0_f64.powf(4.0),
        km22: 1.8 * 10.0_f64.powf(7.0),
        km23: 4.0 * 10.0_f64.powf(9.0),
        km24: 0.2,
        k6p: 6.93 * 10.0_f64.powf(-3.0),
        k8p: 5.0 * 10.0_f64.powf(4.0),
        k9: 1.02 * 10.0_f64.powf(9.0),
    };

    //stale do rk4
    let mut k1s = [0.0; 15];
    let mut k2concs = [0.0; 15];
    let mut k2s = [0.0; 15];
    let mut k3concs = [0.0; 15];
    let mut k3s = [0.0; 15];
    let mut k4concs = [0.0; 15];
    let mut k4s = [0.0; 15];

    let mut t;
    let mut h;
    //stezenia
    let mut conc: State;
    let mut d_conc = [0.0; 15];

    let stezenia_read = File::open(path2win).unwrap();
    let mut rev_lines = RevLines::new(stezenia_read);
    let mut file_was_empty = false;

    match rev_lines.next() {
        Some(line) => {
            let unwrapped_line = line.unwrap().clone();
            let mut time_and_concs = unwrapped_line.trim_end().splitn(16, ",");
            let pt =
                time_and_concs.next().expect("time did not parse correctly");
            t = pt.trim_end().parse::<f64>().unwrap() * 60.0;

            conc = time_and_concs
                .map(|num| num.trim_end().parse::<f64>().unwrap())
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap();
        }
        None => {
            file_was_empty = true;
            let mut c_cu_poczatkowe = String::new();
            println!("c_cu [uM]:");
            io::stdin()
                .read_line(&mut c_cu_poczatkowe)
                .expect("stdin failed - stezenie poczatkowe");
            t = 0.0;
            conc = [10.0_f64.powf(-8.0); 15];
            conc[0] = c_cu_poczatkowe.trim_end().parse::<f64>().unwrap()
                * 10.0_f64.powf(-6.0);
            println!(
                "...c_cu = {}",
                c_cu_poczatkowe.trim_end().parse::<f64>().unwrap()
            );
            conc[1] = 0.417;
            conc[3] = 0.025;
        }
    }

    let stezenia_plik = OpenOptions::new()
        .append(true)
        .open(path2win)
        .expect("Unable to open file");
    let mut stezenia_plik = BufWriter::new(stezenia_plik);

    match file_was_empty {
        true => {
            stezenia_plik
                .write_all(
                    "t,c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14\n"
                        .as_bytes(),
                )
                .expect("tragedia stezenia");
            stezenia_plik
                .write_all(
                    format!(
                        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                        t / 60.0,
                        conc[0],
                        conc[1],
                        conc[2],
                        conc[3],
                        conc[4],
                        conc[5],
                        conc[6],
                        conc[7],
                        conc[8],
                        conc[9],
                        conc[10],
                        conc[11],
                        conc[12],
                        conc[13],
                        conc[14],
                    )
                    .as_bytes(),
                )
                .expect("tragedia stezenia");
        }
        false => {
            stezenia_plik
                .write_all("\n".as_bytes())
                .expect("tragedia stezenia");
        }
    }

    let zapisy_na_sekunde = 10.0;
    while t < 2000.0 {
        if t < 5.0 * 10.0_f64.powf(-1.0) {
            h = 6.0 * 10.0_f64.powf(-9.0)
        } else {
            h = 7.0 * 10.0_f64.powf(-9.0)
        }
        t += h;

        rk4(
            &mut conc,
            &mut d_conc,
            &h,
            &mut km,
            &mut k1s,
            &mut k2concs,
            &mut k2s,
            &mut k3concs,
            &mut k3s,
            &mut k4concs,
            &mut k4s,
        );
        if conc[0] > 0.01 || conc[1].is_nan() {
            println!("{}, {}", t / 60.0, conc[0]);
            break;
        }
        if (zapisy_na_sekunde * (t + h)).floor()
            >= (zapisy_na_sekunde * t).ceil()
        {
            stezenia_plik
                .write_all(
                    format!(
                        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                        t / 60.0,
                        conc[0],
                        conc[1],
                        conc[2],
                        conc[3],
                        conc[4],
                        conc[5],
                        conc[6],
                        conc[7],
                        conc[8],
                        conc[9],
                        conc[10],
                        conc[11],
                        conc[12],
                        conc[13],
                        conc[14],
                    )
                    .as_bytes(),
                )
                .expect("tragedia stezenia");
        }
    }
}

fn rk4(
    c: &mut State,
    dc: &mut State,
    h: &f64,
    km: &mut Oscillator,
    k1s: &mut State,
    k2concs: &mut State,
    k2s: &mut State,
    k3concs: &mut State,
    k3s: &mut State,
    k4concs: &mut State,
    k4s: &mut State,
) {
    let differentials = |c: &State, dc: &mut State| {
        let c_oh = 0.045;
        let rcu21 = km.kcu21 * c[0] * c[1];
        let rcu22 = km.kcu22 * c[2];
        let rm3 = km.km3 * c[2] * c[3];
        let rm4 = km.km4 * c[4] * c[4];
        let rm5 = km.km5 * c[1] * c[3];
        let rm6 = km.km6 * c[1] * c[5];
        let rm7 = km.km7 * c[6] * c[6];
        let rm8 = km.km8 * c[6] * c[7] * c_oh;
        let rm9 = km.km9 * c[7] * c[7] * c_oh;
        let rm18 = km.km18 * c[8] * c[7];
        let rm19 = km.km19 * c[9] * c[9] * c_oh;
        let rm20 = km.km20 * c[9] * c[10] * c_oh;
        let rm21 = km.km21 * c[8] * c[4];
        let rm22 = km.km22 * c[10] * c[11] * c_oh;
        let rm23 = km.km23 * c[11] * c[11] * c_oh;
        let rm24 = km.km24 * c[1] * c[14];
        let r6p = km.k6p * c[2];
        let r8p = km.k8p * c[1] * c[12];
        let r9 = km.k9 * c[1] * c[13];

        dc[0] = -rcu21 + rcu22 + rm20 + r8p;
        dc[1] = -rcu21 + rcu22 - rm5 - rm6 - rm24 - r8p - r9;
        dc[2] = rcu21 - rcu22 - rm3;
        dc[3] = -2.0 * rm3 - rm5 + 2.0 * rm20 + 2.0 * rm22;
        dc[4] = rm3 - 2.0 * rm4 - rm21 + rm9 + r6p;
        dc[5] = rm5 - rm6 + rm7 + rm8;
        dc[6] = rm6 - 2.0 * rm7 - rm8;
        dc[7] = rm7 - rm8 - 2.0 * rm9 - rm18 + rm19;
        dc[8] = rm9 - rm18 + rm19 + rm20 - rm21;
        dc[9] = 2.0 * rm18 - 2.0 * rm19 - rm20;
        dc[10] = rm3 - rm20 - rm22;
        dc[11] = rm21 - rm22 - rm23;
        dc[12] = r6p - r8p;
        dc[13] = r8p - r9;
        dc[14] = rm22 + rm23 - rm24;
    };

    let mut kxer = |kxconcs: &State, kxs: &mut State| {
        differentials(kxconcs, dc);
        (0usize..15).for_each(|i| kxs[i] = h * dc[i]);
    };
    let kxconculator = |kxs: &State, multiplier, kxconcs: &mut State| {
        (0usize..15)
            .zip(kxs)
            .for_each(|(i, k)| kxconcs[i] = c[i] + k * multiplier)
    };

    kxer(c, k1s);
    kxconculator(k1s, 0.5, k2concs);
    kxer(k2concs, k2s);
    kxconculator(k2s, 0.5, k3concs);
    kxer(k3concs, k3s);
    kxconculator(k3s, 1.0, k4concs);
    kxer(k4concs, k4s);

    zip!(k1s, k2s, k3s, k4s).enumerate().for_each(
        |(id, (k1, (k2, (k3, k4))))| {
            c[id] += (k1 + 2.0 * k2 + 2.0 * k3 + *k4) / 6.0;
        },
    );
}
