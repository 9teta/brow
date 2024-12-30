
use std::ffi::OsStr;

use tokio::process::Child;


pub async fn cmd_run_web_driver<T: AsRef<OsStr>>(path: T, port: u16, name: &str) -> CloseOnDropChild {
    let port_arg = format!("--port={}", port);
    let child = tokio::process::Command::new(path)
    .arg(port_arg)
    .spawn()
    .expect("Failed spawn web driver process");
    log::info!("Running web driver process on port {} with name {}", port, name);
    let close_on_drop_child = CloseOnDropChild { child, name: name.to_owned()};
    close_on_drop_child 
}


pub struct CloseOnDropChild {
    child: Child,
    name: String
}

impl Drop for CloseOnDropChild {
    fn drop(&mut self) {
        if let Some(id) = self.child.id() {
            log::info!("Closing child process with ID: {} and name {}", id, self.name);
            // Attempt to kill the child process if it's still running
            let _ = self.child.kill();
        }
    }
}