pub mod bmb{
    use std::ffi::OsString;
    use std::net::UdpSocket;
    use std::str;
    use chrono::Utc;
    use nix::unistd::getpid;
    use std::io::Write;
    use std::os::unix::raw::pid_t;
    use rand::{distributions::Alphanumeric, Rng};
    use std::{process::exit, thread, time};

    fn gen_payload(min_len: i32, max_len: i32) -> String {
        let u_min: usize = min_len as usize;
        let u_max: usize = max_len as usize;
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rand::thread_rng().gen_range(u_min..u_max))
            .map(char::from)
            .collect()
    }


    fn udp_push(hostname: &str, pid: pid_t, seq: i32, target_host: &str, target_port: i32, payload: &str, recv: i32) {
        const K_MAX_BUFSIZE: usize = 2048;

        let mut send_string = Vec::new();
        std::write!(&mut send_string, "{}->{}->{}->{}->{}:{}---->{}", Utc::now(), hostname,
                    pid, seq, target_host, target_port, payload).unwrap();
        let socket = UdpSocket::bind("[::]:0").expect("cannot bind to port");
        let mut target_address = Vec::new();
        std::write!(&mut target_address, "{}:{}", target_host, target_port).expect("Cannot write target address!");
        let addr_string = str::from_utf8(&target_address).expect("Can't convert the string");
        socket.send_to(&send_string, addr_string).expect("error on send!!");

        if recv == 1 {
            let mut buf = [0; K_MAX_BUFSIZE];
            let (amt, _src) = socket.recv_from(&mut buf).expect("cannot receive data");
            let echo = str::from_utf8(&buf[..amt]).unwrap();
            println!("{}:->{}", Utc::now(), echo);
        }
    }


    fn looper(hostname: &str, pid: pid_t, target_host: &str, target_port: i32, recv: i32) {
        let mut iter_count = rand::thread_rng();
        const K_RANGE: i32 = 125;

        for i in 1..iter_count.gen_range(1..K_RANGE) {
            let payload = gen_payload(5, 1829).clone();
            udp_push(&hostname, pid, i, target_host, target_port, &payload, recv);
        }
    }

    pub fn threader(thread_count: i32, hostname: &str, pid: pid_t, target_host: &str, target_port: i32, recv: i32) {
        for thd_id in 1..thread_count + 1 {
            let static_hostname = format!("{}", hostname.clone()); //get ownership for the thread
            let static_target_host = format!("{}", target_host.clone());
            let handle = thread::spawn(move || {
                looper(&static_hostname.to_string(), pid, &static_target_host.to_string(), target_port + (thd_id - 1), recv);
            });
            handle.join().unwrap();
            exit(0);
        }
    }
}