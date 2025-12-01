struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}!", self.data);
    }
}

#[allow(unused_variables)]
fn main() {
    let a = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let b = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created.");

    drop(c);

    println!("CustomSmartPointer dropped before the end of main.");
}
