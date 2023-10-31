use ode_solvers::Dop853;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
type State = ode_solvers::SVector<f64, 15>;
type Time = f64;

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
impl ode_solvers::System<State> for Oscillator {
    // Equations of motion of the system
    fn system(&self, _t: Time, c: &State, dc: &mut State) {
        let c_oh = 0.025;
        let rcu21 = self.kcu21 * c[0] * c[1];
        let rcu22 = self.kcu22 * c[2];
        let rm3 = self.km3 * c[2] * c[3];
        let rm4 = self.km4 * c[4] * c[4];
        let rm5 = self.km5 * c[1] * c[3];
        let rm6 = self.km6 * c[1] * c[5];
        let rm7 = self.km7 * c[6] * c[6];
        let rm8 = self.km8 * c[6] * c[7] * c_oh;
        let rm9 = self.km9 * c[7] * c[7] * c_oh;
        let rm18 = self.km18 * c[8] * c[7];
        let rm19 = self.km19 * c[9] * c[9] * c_oh;
        let rm20 = self.km20 * c[9] * c[10] * c_oh;
        let rm21 = self.km21 * c[8] * c[4];
        let rm22 = self.km22 * c[10] * c[11] * c_oh;
        let rm23 = self.km23 * c[11] * c[11] * c_oh;
        let rm24 = self.km24 * c[1] * c[14];
        let r6p = self.k6p * c[2];
        let r8p = self.k8p * c[1] * c[12];
        let r9 = self.k9 * c[1] * c[13];

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
    }
}
fn main() {
    //stałe
    let kcu21 = 10.0_f64.powf(2.0);
    let kcu22 = 8.3;
    let km3 = 1.6;
    let km4 = 40.0;
    let km5 = 1.5 * 10.0_f64.powf(-3.0);
    let km6 = 0.3;
    let km7 = 5.0 * 10.0_f64.powf(2.0);
    let km8 = 4.0 * 10.0_f64.powf(4.0);
    let km9 = 40.0;
    let km18 = 7.0 * 10.0_f64.powf(4.0);
    let km19 = 8.0 * 10.0_f64.powf(7.0);
    let km20 = 1.2 * 10.0_f64.powf(7.0);
    let km21 = 4.0 * 10.0_f64.powf(4.0);
    let km22 = 1.8 * 10.0_f64.powf(7.0);
    let km23 = 4.0 * 10.0_f64.powf(9.0);
    let km24 = 0.2;
    let k6p = 6.93 * 10.0_f64.powf(-3.0);
    let k8p = 5.0 * 10.0_f64.powf(4.0);
    let k9 = 1.02 * 10.0_f64.powf(9.0);

    let system = Oscillator {
        kcu21,
        kcu22,
        km3,
        km4,
        km5,
        km6,
        km7,
        km8,
        km9,
        km18,
        km19,
        km20,
        km21,
        km22,
        km23,
        km24,
        k6p,
        k8p,
        k9,
    };

    let mut c_cu_poczatkowe = String::new();
    println!("c_cu [uM]:");
    io::stdin()
        .read_line(&mut c_cu_poczatkowe)
        .expect("stdin failed - stezenie poczatkowe");

    let mut conc = [10.0_f64.powf(-19.0); 15];
    conc[0] = c_cu_poczatkowe.trim_end().parse::<f64>().unwrap() * 10.0_f64.powf(-6.0);
    println!(
        "...c_cu = {}",
        c_cu_poczatkowe.trim_end().parse::<f64>().unwrap()
    );
    conc[1] = 0.2;
    conc[3] = 0.025;
    let c0 = State::from(conc);

    let mut stepper = Dop853::from_param(
        system,
        0.0,
        500.0,
        1.0,
        c0,
        1.0e-14,
        1.0e-14,
        0.9,
        0.0,
        0.333,
        6.0,
        9.0* 10.0_f64.powf(-9.0),
        10.0_f64.powf(-9.0),
        10_u32.pow(7),
        1000,
        ode_solvers::dop_shared::OutputType::Dense,
    );
    let res = stepper.integrate();

    let path2win = r"C:\Users\admin\Desktop\MTHOMAS\x\model_japonczykow\stezenia.csv";

    let stezenia_plik = OpenOptions::new()
        .append(true)
        .open(path2win)
        .expect("Unable to open file");

    let mut stezenia_plik = BufWriter::new(stezenia_plik);
    stezenia_plik
        .write_all("t,c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14,\n".as_bytes())
        .expect("tragedia stezenia");

    // Handle result
    match res {
        Ok(_stats) => {
            stepper
                .x_out()
                .iter()
                .zip(stepper.y_out())
                .for_each(|(t, concs)| {
                    stezenia_plik.write_all(
                        format!(
                            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                            t,
                            concs[0],
                            concs[1],
                            concs[2],
                            concs[3],
                            concs[4],
                            concs[5],
                            concs[6],
                            concs[7],
                            concs[8],
                            concs[9],
                            concs[10],
                            concs[11],
                            concs[12],
                            concs[13],
                            concs[14]
                        )
                        .as_bytes(),
                    ).expect("aaaaaaaaaaa");
                });

            // Do something with the output...
            // let path = Path::new("./outputs/kepler_orbit_dopri5.dat");
            // save(stepper.x_out(), stepper.y_out(), path);
            // println!("Results saved in: {:?}", path);
        }
        Err(_) => println!("An error    occured."),
    }
}
