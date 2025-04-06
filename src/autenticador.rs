use koibumi_base32 as base32;
use std::io::{self, Write};
use std::time::SystemTime;
use totp_lite::{totp_custom, Sha1, DEFAULT_STEP};

pub fn valida_codigo_autenticador() {
    println!("Press ctrl-c to cancel.");
    loop {
        let mut input = String::new();

        print!("Enter your TOTP secret: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let length = input.trim().len();
        if length != 16 && length != 26 && length != 32 {
            println!("Invalid TOTP secret, must be 16, 26 or 32 characters.");
            continue;
        }

        let seconds: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        println!(
            "Your TOTP code: {}",
            totp_custom::<Sha1>(
                // Calculate a new code every 30 seconds.
                DEFAULT_STEP,
                // Calculate a 6 digit code.
                6,
                // Convert the secret into bytes using base32::decode().
                &base32::decode(&input.trim().to_lowercase()).unwrap(),
                // Seconds since the Unix Epoch.
                seconds,
            )
        );
    }
}