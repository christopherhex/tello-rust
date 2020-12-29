use std::{net::UdpSocket, str};

// Tello SDK: https://dl-cdn.ryzerobotics.com/downloads/Tello/Tello%20SDK%202.0%20User%20Guide.pdf

pub enum TelloCommand {
    SetSDK,
    Takeoff,
    Land,
    StreamOn,
    StreamOff,
    Emergency,
    Up { x: u32 },
    Down { x: u32 },
    Left { x: u32 },
    Right { x: u32 },
    Forward { x: u32 },
    Back { x: u32 },
    RotateCW { x: u32 },
    RotateCCW { x: u32 },
    Flip { x: char },
    Go { x: u32, y: u32, z: u32, speed: u32},
    Stop,
    ReadSpeed,
    ReadBatt,
    ReadFlightTime,
    ReadWifi,
    ReadSdk,
    ReadSerial
}

pub struct TelloDrone {
    command_socket: UdpSocket,
    stat_socket: UdpSocket
}

impl TelloDrone {

    pub fn init() -> TelloDrone {
        let cmd = UdpSocket::bind("0.0.0.0:8889").expect("Unable to bind to CMD address");
        let stat = UdpSocket::bind("0.0.0.0:8890").expect("Unable to bind to STAT address");

        let inst = TelloDrone {
            command_socket: cmd,
            stat_socket: stat
        };

        inst.send_command(TelloCommand::SetSDK);

        return inst;
    }

    fn get_tello_command_data(command: &TelloCommand) -> Vec<u8> {
        match command {
            TelloCommand::SetSDK => "command".as_bytes().to_vec(),
            TelloCommand::Takeoff => "takeoff".as_bytes().to_vec(),
            TelloCommand::Land => "land".as_bytes().to_vec(),
            TelloCommand::StreamOn => "streamon".as_bytes().to_vec(),
            TelloCommand::StreamOff => "streamoff".as_bytes().to_vec(),
            TelloCommand::Emergency => "emergency".as_bytes().to_vec(),
            TelloCommand::Up { x } => {
                format!("up {val}", val=x.to_string()).as_bytes().to_vec()
            },            
            TelloCommand::Down { x } => {
                format!("down {val}", val=x.to_string()).as_bytes().to_vec()
            },
            TelloCommand::Left { x } => {
                format!("left {val}", val=x.to_string()).as_bytes().to_vec()
            },
            TelloCommand::Right { x } => {
                format!("right {val}", val=x.to_string()).as_bytes().to_vec()
            },
            TelloCommand::Forward { x } => {
                format!("forward {val}", val=x.to_string()).as_bytes().to_vec()
            },
            TelloCommand::Back { x } => {
                format!("back {val}", val=x.to_string()).as_bytes().to_vec()
            },
            TelloCommand::RotateCW { x } => {
                format!("cw {val}", val=x.to_string()).as_bytes().to_vec()
            },
            TelloCommand::RotateCCW { x } => {
                format!("ccw {val}", val=x.to_string()).as_bytes().to_vec()
            },
            TelloCommand::Flip { x } => {
                format!("flip {x}", x=x).as_bytes().to_vec()
            },
            TelloCommand::Go { x, y, z, speed} => {
                format!("go {x} {y} {z} {speed}", x=x.to_string(), y=y.to_string(), z=z.to_string(), speed = speed.to_string()).as_bytes().to_vec()
            },
            TelloCommand::Stop => "stop".as_bytes().to_vec(),
            TelloCommand::ReadSpeed => "speed?".as_bytes().to_vec(),
            TelloCommand::ReadBatt => "battery?".as_bytes().to_vec(),
            TelloCommand::ReadFlightTime => "time?".as_bytes().to_vec(),
            TelloCommand::ReadWifi => "wifi?".as_bytes().to_vec(),
            TelloCommand::ReadSdk => "sdk?".as_bytes().to_vec(),
            TelloCommand::ReadSerial => "sn?".as_bytes().to_vec(),
        }
    }

    pub fn send_command(&self, command: TelloCommand) -> String {
        let command = TelloDrone::get_tello_command_data(&command);
        self.command_socket.send_to(command.as_slice(), "192.168.10.1:8889").expect("Cannot send command");

        self.get_response()
    }

    fn get_response(&self){

        let mut buf = [0; 10];
        match self.command_socket.recv(&mut buf) {
            Ok(received) => println!("received {} bytes {:?}", received, str::from_utf8(&buf[..received])),
            Err(e) => println!("recv function failed: {:?}", e),
        }
    }

    pub fn get_stats(&self) {

        let mut buf = [0;150];

        match self.stat_socket.recv(&mut buf) {
            Ok(received) => println!("received {} bytes {:?}", received,  str::from_utf8(&buf[..received])),
            Err(e) => println!("recv function failed: {:?}", e),
        }
    }
}