#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
                println!("Your order is being prepered.");
            }
            other_options => {
                println!("Your order with status {other_options:?} was Shipped.");
            }
        }
    }
}

fn main() {
    OnlineOrderStatus::Ordered.check();
    OnlineOrderStatus::Packed.check();
    OnlineOrderStatus::Shipped.check();
    OnlineOrderStatus::Delivered.check();
}
