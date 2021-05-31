extern crate bcl_blc_lib;
use bcl_blc_lib::obsolete_lib::*;

fn main() {
    let data = "S (S (K S) (S (K (S (K S))) (S (K (S (K K))) (S (K (S (K S))) (S (K S) K))))) (K (S (K K)))";
    let bcl = sk_to_bcl(data);
    match bcl {
        Ok(bcl) => println!("Ok:\n{} ({} bytes)", bcl, (bcl.len() + 7) / 8),
        Err(msg) => println!("Err:\n{}", msg)
    }
}
