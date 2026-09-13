use std::process::{Child, Command};

/// Clean up the orphaned processes left behind by the servers.
///
/// On unix the whole process group of every child is killed, plus any stray
/// backend binary that escaped its group. On other systems the children are killed directly.
/// Every child is reaped afterwards.
pub fn terminate_services(children: &mut [&mut Child], backend_binary: &str) {
    #[cfg(unix)]
    {
        // Kill the entire process groups (including any spawned children) using negative PID
        for child in children.iter() {
            let _ = Command::new("kill")
                .arg("-9")
                .arg("--")
                .arg(format!("-{}", child.id()))
                .status();
        }

        // cargo run's child (the actual backend binary) can escape the cargo
        // process group, so the group kill above may not reach it and it stays
        // bound to the port. Kill it directly by binary path as a fallback.
        let _ = Command::new("pkill")
            .arg("-9")
            .arg("-f")
            .arg(backend_binary)
            .status();
    }

    #[cfg(not(unix))]
    {
        let _ = backend_binary;

        for child in children.iter_mut() {
            let _ = child.kill();
        }
    }

    for child in children.iter_mut() {
        let _ = child.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminate_services_runs_without_crashing() {
        let mut children: Vec<&mut Child> = vec![];
        terminate_services(&mut children, "target/debug/backend");
    }
}
