use std::time::Duration;

#[cfg(feature = "use_rustls")]
mod ring;
#[cfg(feature = "use_rustls")]
pub use self::ring::{EcdsaP256SHA256KeyPair,
    sha256_hasher, sha256,
    gen_acme_cert, CertBuilder};
#[cfg(feature = "use_openssl")]
mod openssl;
#[cfg(feature = "use_openssl")]
pub use self::openssl::{EcdsaP256SHA256KeyPair,
    sha256_hasher, sha256,
    gen_acme_cert, CertBuilder};

use std::time::{SystemTime, UNIX_EPOCH};
use x509_parser::parse_x509_certificate;

pub fn get_cert_duration_left(x509_cert : &[u8]) -> Result<Duration,()> {
    let valid_until = match parse_x509_certificate(x509_cert) {
        Ok((_, cert)) => {
            cert.validity()
            .not_after
            .timestamp() as u64
        },
        Err(_err) => {
            return Err(());
        }
    };

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
    //Ok(Duration::from_secs(valid_until).saturating_sub(since_the_epoch))
    let valid_secs = (valid_until - since_the_epoch.as_secs()).max(0);
    let wait_secs = Duration::from_secs(valid_secs as u64);
    Ok(wait_secs)
}