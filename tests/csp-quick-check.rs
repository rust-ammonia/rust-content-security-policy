extern crate content_security_policy;

#[cfg(feature = "quickcheck")]
#[macro_use]
extern crate quickcheck;

#[cfg(feature = "quickcheck")]
mod csp_quick_check {
    use content_security_policy::*;
    quickcheck!{
        fn roundtrip_csplist(xs: CspList) -> bool {
            let alpha = xs.to_string();
            let beta = CspList::parse(
                &alpha,
                PolicySource::Header,
                PolicyDisposition::Enforce).to_string();
            assert_eq!(alpha, beta);
            alpha == beta
        }
    }
}
