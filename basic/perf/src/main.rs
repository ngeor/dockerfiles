use std::env;
use std::io;
use std::io::Write;
use std::process::{exit, Command, Stdio};
use std::time::SystemTime;

mod docker;

#[derive(Debug)]
struct Args {
    count: i32,
    quiet: bool,
    qbasic: bool,
}

fn copy_env(key: &str) -> String {
    format!("{}={}", key, env::var(key).unwrap_or_default())
}

fn copy_basic_mode() -> String {
    copy_env("BLR_BASIC_MODE")
}

impl Args {
    fn parse(&mut self) {
        let mut iterator = env::args().skip(1);
        loop {
            let next = iterator.next();
            match next {
                Some(value) => {
                    if value == "--count" {
                        let next2 = iterator.next();
                        self.count = match next2 {
                            Some(x) => x.parse().unwrap(),
                            None => panic!("--count requires an argument"),
                        };
                    } else if value == "--quiet" {
                        self.quiet = true;
                    } else {
                        panic!("Unexpected parameter {}", value);
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
}

fn now() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn run_standalone(args: &Args) {
    let program = if args.qbasic {
        "./basic/src/HELLOQB.BAS"
    } else {
        "./basic/src/HELLO.BAS"
    };
    let output = Command::new("./basic-launcher-rust/target/release/basic-launcher-rust.exe")
        .args(&[program])
        .stdout(if args.quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        eprintln!("Failed");
        exit(output.status.code().unwrap_or(0));
    }
}

fn progress(n: i32, args: &Args) {
    if args.quiet {
        print!(".");
        io::stdout().flush().unwrap();
    } else {
        println!("{}", n);
    }
}

fn dos_experiment(args: &Args) -> f64 {
    let start = now();
    println!("Running DOS experiment");
    for n in 1..args.count + 1 {
        progress(n, args);
        run_standalone(args);
    }
    let stop = now();
    println!("{}", stop - start);
    let average = ((stop - start) as f64) / (args.count as f64);
    println!("average {}", average);
    average
}

/// Gets the current directory, converting it to a path that Docker understands as a volume.
fn current_dir_as_msys_path() -> String {
    let original = format!("{}", env::current_dir().unwrap().display());
    original.replace("C:\\", "/c/").replace("\\", "/")
}

fn build_image(args: &Args) {
    println!("Building Docker image");
    docker::build("basic", "Dockerfile.standalone", args.quiet);
}

fn run_docker_outside(args: &Args) {
    let bin_volume_spec = docker::Volume {
        host: format!("{}/bin", current_dir_as_msys_path()),
        guest: "/basic/bin".to_string(),
    };
    let src_volume_spec = docker::Volume {
        host: format!("{}/basic/src", current_dir_as_msys_path()),
        guest: "/basic/src".to_string(),
    };
    let program = if args.qbasic {
        "HELLOQB.BAS"
    } else {
        "HELLO.BAS"
    };
    let basic_mode = copy_basic_mode();
    docker::run(
        "basic",
        docker::RunOptions {
            args: vec![program.to_string()],
            volumes: vec![bin_volume_spec, src_volume_spec],
            env: vec![basic_mode],
            entry_point: None,
            ports: vec![],
            detach: false,
            container_name: None,
        },
        args.quiet,
    );
}

fn docker_outside_experiment(args: &Args) -> f64 {
    build_image(args);
    let start = now();
    println!("Running Docker (outside) experiment");
    for n in 1..args.count + 1 {
        progress(n, args);
        run_docker_outside(args);
    }
    let stop = now();
    println!("{}", stop - start);
    let average = ((stop - start) as f64) / (args.count as f64);
    println!("average {}", average);
    average
}

fn docker_inside_experiment(args: &Args) -> f64 {
    build_image(args);
    let start = now();
    println!("Running Docker (inside) experiment");
    let bin_volume_spec: docker::Volume = docker::Volume {
        host: format!("{}/bin", current_dir_as_msys_path()),
        guest: "/basic/bin".to_string(),
    };
    let src_volume_spec: docker::Volume = docker::Volume {
        host: format!("{}/basic/src", current_dir_as_msys_path()),
        guest: "/basic/src".to_string(),
    };
    let perf_volume_spec: docker::Volume = docker::Volume {
        host: format!("{}/perf", current_dir_as_msys_path()),
        guest: "/usr/local/perf/bin:ro".to_string(),
    };
    let fmt_count = args.count.to_string();
    let basic_mode = copy_basic_mode();

    let mut run_args: Vec<String> =
        vec!["/usr/local/perf/bin/perf-inside.sh".to_string(), fmt_count];
    if args.quiet {
        run_args.push("--quiet".to_string());
    }
    docker::run(
        "basic",
        docker::RunOptions {
            args: run_args,
            volumes: vec![bin_volume_spec, src_volume_spec, perf_volume_spec],
            env: vec![basic_mode],
            entry_point: Some("bash".to_string()),
            ports: vec![],
            detach: false,
            container_name: None,
        },
        false,
    );
    let stop = now();
    println!("{}", stop - start);
    let average = ((stop - start) as f64) / (args.count as f64);
    println!("average {}", average);
    average
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ApacheVariant {
    Launcher,
    Interpreter,
}

impl ApacheVariant {
    pub fn to_docker_image_name(&self) -> &str {
        match self {
            Self::Launcher => "basic-httpd",
            Self::Interpreter => "basic-interpreter-httpd",
        }
    }
}

fn build_httpd_image(args: &Args, flavour: ApacheVariant) {
    println!("Building Docker HTTPD image");
    docker::build(
        flavour.to_docker_image_name(),
        if flavour == ApacheVariant::Launcher {
            "Dockerfile.httpd"
        } else {
            "Dockerfile.interpreter.httpd"
        },
        args.quiet,
    );
}

fn start_httpd(args: &Args, flavour: ApacheVariant) {
    println!("Starting HTTPD");
    let bin_volume_spec: docker::Volume = docker::Volume {
        host: format!("{}/bin", current_dir_as_msys_path()),
        guest: "/basic/bin".to_string(),
    };
    let src_host_dir = if flavour == ApacheVariant::Interpreter {
        format!("{}/basic/rest-qb-direct", current_dir_as_msys_path())
    } else {
        if args.qbasic {
            format!("{}/basic/rest-qb", current_dir_as_msys_path())
        } else {
            format!("{}/basic/rest:/basic/src", current_dir_as_msys_path())
        }
    };
    let src_volume_spec: docker::Volume = docker::Volume {
        host: src_host_dir,
        guest: "/basic/src".to_string(),
    };
    let basic_mode = copy_basic_mode();

    let name = flavour.to_docker_image_name();
    docker::run(
        name,
        docker::RunOptions {
            args: vec![],
            volumes: vec![bin_volume_spec, src_volume_spec],
            env: vec![basic_mode],
            entry_point: None,
            ports: vec![docker::Port {
                host: 8080,
                guest: 80,
            }],
            detach: true,
            container_name: Some(name.to_string()),
        },
        args.quiet,
    );
}

fn stop_httpd(args: &Args, flavour: ApacheVariant) {
    println!("Stopping HTTPD");
    docker::stop(flavour.to_docker_image_name(), args.quiet);
}

fn run_curl(i: i32, args: &Args) {
    let payload = format!("hello {}", i);
    let mut run_args = vec![
        "-f",
        "--data",
        &payload,
        "-H",
        "Content-Type: text/plain",
        "http://localhost:8080/api/todo",
    ];

    if args.quiet {
        run_args.insert(0, "--silent");
    }
    let output = Command::new("curl")
        .args(run_args)
        .stdout(if args.quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        eprintln!("Failed");
        exit(output.status.code().unwrap_or(1));
    }
}

fn apache_experiment(args: &Args, flavour: ApacheVariant) -> f64 {
    build_httpd_image(args, flavour);
    start_httpd(args, flavour);
    let start = now();
    println!("Running Apache {:?} experiment", flavour);
    for n in 1..args.count + 1 {
        progress(n, args);
        run_curl(n, args);
    }
    let stop = now();
    println!("{}", stop - start);
    let average = ((stop - start) as f64) / (args.count as f64);
    println!("average {}", average);
    stop_httpd(args, flavour);
    average
}

fn main() {
    let mut args = Args {
        count: 100,
        quiet: false,
        qbasic: env::var("BLR_BASIC_MODE").unwrap_or_default() == "qbasic",
    };
    args.parse();
    let dos_average = dos_experiment(&args);
    let docker_outside_average = docker_outside_experiment(&args);
    let docker_inside_average = docker_inside_experiment(&args);
    let apache_average = apache_experiment(&args, ApacheVariant::Launcher);
    let apache_interpreter_average = apache_experiment(&args, ApacheVariant::Interpreter);
    println!("Summary:");
    println!("| DOS                  | {} |", dos_average);
    println!("| Docker (outside)     | {} |", docker_outside_average);
    println!("| Docker (inside)      | {} |", docker_inside_average);
    println!("| Apache (launcher)    | {} |", apache_average);
    println!("| Apache (interpreter) | {} |", apache_interpreter_average);
}

// TODO make wrapper clients for launching curl, docker, etc
