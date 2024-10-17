use pqkd::builder;
use url::Url;

#[test]
fn build_pqkd_with_url() {
    let pqkd = builder::BuilderPqkd::with_url("http://127.0.0.1".parse().unwrap()).build();
        
    assert_eq!(*pqkd.url(), "http://127.0.0.1".parse::<Url>().unwrap());
    assert_eq!(pqkd.port_kme(), 8082);
    assert_eq!(pqkd.port_qrng(), 8085);
}

#[test]
fn build_pqkd_with_ip4_port_kme() {
    let pqkd = builder::BuilderPqkd::with_url("http://127.0.0.1".parse().unwrap())
        .with_port_kme(18082)
        .build();

    assert_eq!(*pqkd.url(), "http://127.0.0.1".parse::<Url>().unwrap());
    assert_eq!(pqkd.port_kme(), 18082);
    assert_eq!(pqkd.port_qrng(), 8085);
}

#[test]
fn build_pqkd_with_ip4_port_qrng() {
    let pqkd = builder::BuilderPqkd::with_url("http://127.0.0.1".parse().unwrap())
        .with_port_qrng(18085)
        .build();

    assert_eq!(*pqkd.url(), "http://127.0.0.1".parse::<Url>().unwrap());
    assert_eq!(pqkd.port_kme(), 8082);
    assert_eq!(pqkd.port_qrng(), 18085);
}

#[test]
fn build_pqkd_with_ip4_port_kme_port_qrng() {
    let pqkd = builder::BuilderPqkd::with_url("http://127.0.0.1".parse().unwrap())
        .with_port_kme(18082)
        .with_port_qrng(18085)
        .build();

    assert_eq!(*pqkd.url(), "http://127.0.0.1".parse::<Url>().unwrap());
    assert_eq!(pqkd.port_kme(), 18082);
    assert_eq!(pqkd.port_qrng(), 18085);
}