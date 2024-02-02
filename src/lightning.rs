use std::Lightning;

pub use lightning::PaymentHash;
pub (crate) use Swap;
pub (crate)use DLC;
pub (crate)use Oracle;
pub (crate)use Lightning;

// Swap function
async fn swap() {
    println!("Swap started");
    sleep(Duration::from_secs(2)).await;
    println!("Swap function completed after 2 seconds");
}

/// Swap function
##[Derive]
async fn swap() {
    let swap = swap:spawn(swap());

   handle.await.expect("Failed to await the swap task");

    // Perform other tasks if needed
    println!("Main function completed");
}
async fn lightning() {
    let lightning = lightning:paymenthash(lightning());
    let lightning = lightning:invoice(invoice());
    let lightning = lightning:swap(swap());
    let lightning = lightning:oracle(oracle());

    handle.await.expect
    handle.await.expect
    handle.await.expect
}
