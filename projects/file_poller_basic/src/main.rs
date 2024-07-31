// Breakdown tasks into atomic functions
// integrate basic functions into larger modules


struct ConfiguredWatchDirs {
    watch_dirs: [WatchDir],
}

struct WatchDir {
}

fn fetch_watch_directory_details() -> ConfiguredWatchDirs {
    // reads the details of the watch directory from the configurations
    // returns an struct watch_dir which contains the information of the directory to read
    let watch_dirs = [WatchDir {}];
    ConfiguredWatchDirs { watch_dirs:watch_dirs }
}



fn main() {
    let proj_scope = "File poller: poll the files as 
        they arrive in a specific folder and move it to another folder, while also add it to a queue.
        Tasks:
            1. Watch for files in a directory configured.
            2. Move the files when the file is ready.
            3. Move the files to another folder and add a message to the queue.
            4. Handle multiple threads and parallely do this.
            5. run a queue in a different service, on a port.
            6. write the message to the port/socket from the polling application.
            7. read the message from the port/socket and add the info from the message to a queue.
            8. Write the info from the queue to a file to persist the order of processing.
        Concepts:
            1. Setting up the configuration of the project.
            2. How to watch a directory for new files.
            3. Keeping track of the files and their states.
            4. Move the files from one directory to another when state is reached.
            5. Parallely processing this ( Threadpool or ProcessPool ).
            6. Logging of the process.
            7. Error Handling of the scenarios.
            8. Write and read from socket.
            9. Writing/Reading into queue.
            10. Writing back of the messages back to file for persistance.
        ";
    println!("The scope pf the project is : \n {proj_scope}");
}


