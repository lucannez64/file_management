// Ping service example.
//
// You can install and uninstall this service using other example programs.
// All commands mentioned below shall be executed in Command Prompt with Administrator privileges.
//
// Service installation: `install_service.exe`
// Service uninstallation: `uninstall_service.exe`
//
// Start the service: `net start ping_service`
// Stop the service: `net stop ping_service`
//
// Ping server sends a text message to local UDP port 1234 once a second.
// You can verify that service works by running netcat, i.e: `ncat -ul 1234`.

#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    file_management::run()
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}

#[cfg(windows)]
mod file_management {
    use notify::{Watcher, RecursiveMode, watcher};
    use std::{
        ffi::OsString,
        sync::mpsc,
        time::Duration,
        fs,
    };
    use windows_service::{
        define_windows_service,
        service::{
            ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
            ServiceType,
        },
        service_control_handler::{self, ServiceControlHandlerResult},
        service_dispatcher, Result,
    };

    const SERVICE_NAME: &str = "file_management";
    const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

    pub fn run() -> Result<()> {
        // Register generated `ffi_service_main` with the system and start the service, blocking
        // this thread until the service is stopped.
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }

    // Generate the windows service boilerplate.
    // The boilerplate contains the low-level service entry function (ffi_service_main) that parses
    // incoming service arguments into Vec<OsString> and passes them to user defined service
    // entry (my_service_main).
    define_windows_service!(ffi_service_main, my_service_main);

    // Service entry function which is called on background thread by the system with service
    // parameters. There is no stdout or stderr at this point so make sure to configure the log
    // output to file if needed.
    pub fn my_service_main(_arguments: Vec<OsString>) {
        if let Err(_e) = run_service() {
            // Handle the error, by logging or something.
        }
    }

    pub fn run_service() -> Result<()> {
        // Create a channel to be able to poll a stop event from the service worker loop.
        let (shutdown_tx, shutdown_rx) = mpsc::channel();

        // Define system service event handler that will be receiving service events.
        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                // Notifies a service to report its current status information to the service
                // control manager. Always return NoError even if not implemented.
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

                // Handle stop
                ServiceControl::Stop => {
                    shutdown_tx.send(()).unwrap();
                    ServiceControlHandlerResult::NoError
                }

                _ => ServiceControlHandlerResult::NotImplemented,
            }
        };

        // Register system service event handler.
        // The returned status handle should be used to report service status changes to the system.
        let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

        // Tell the system that service is running
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        // For demo purposes this service sends a UDP packet once a second.
        let (tx, rx) = mpsc::channel();

        // Create a watcher object, delivering debounced events.
        // The notification back-end is selected based on the platform.
        let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();
        
        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch("C:/Downloads", RecursiveMode::Recursive).unwrap();
        

        loop {
            match rx.recv() {
                Ok(event) =>{ 
                    match event {
                        notify::DebouncedEvent::Create(patha) => {
                            match patha.extension() {
                                Some(ext) => {
                                    match ext.to_str().unwrap() {
                                        "PDF" => {
                                            fs::rename(&patha, "C:/Users/mazav/OneDrive/Documents/PDF/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "pdf" => {
                                            fs::rename(&patha, "C:/Users/mazav/OneDrive/Documents/PDF/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "rar" => {
                                            fs::rename(&patha, "C:/ZIP/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "zip" => {
                                            fs::rename(&patha, "C:/ZIP/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "msi" => {
                                            fs::rename(&patha, "C:/installation/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "exe" => {
                                            fs::rename(&patha, "C:/installation/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "ai" => {
                                            fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/SVG/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "svg" => {
                                            fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/SVG/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "aseprite" => {
                                            fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/SVG/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "gif" => {
                                            fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/SVG/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "PNG" => {
                                            fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "png" => {
                                            fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "jpg" => {
                                            fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "jpeg" => {
                                            fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        "webp" => {
                                            fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
                                        },
                                        _ => {
                                            println!("{:?}", patha);
                                        }
                                    }
                                },
                                None => {},
                            };
                        },
                        _ => {}
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }
            // Poll shutdown event.
            match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
                // Break the loop either upon stop or channel disconnect
                Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

                // Continue work if no events were received within the timeout
                Err(mpsc::RecvTimeoutError::Timeout) => (),
            };
        }

        // Tell the system that service has stopped.
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        Ok(())
    }
}