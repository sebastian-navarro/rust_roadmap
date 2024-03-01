trait Sellable {
    fn price(&self) -> u16;
    fn description(&self) -> String;
}

struct Sword {
    pub name: String,
    pub damage: u16,
    pub swing_time_ms: u16
}

impl Sellable for Sword {
    fn price(&self) -> u16 {
        (self.damage * 1000_u16) / self.swing_time_ms * 10_u16
    }
    fn description(&self) -> String {
        format!("{}, damage: {}, swing time: {}ms", self.name, self.damage, self.swing_time_ms)
    }
}

struct Shield {
    pub name: String,
    pub armor: u16,
    pub block: u16
}

impl Sellable for Shield {
    fn price(&self) -> u16 {
        self.armor + self.block
    }
    fn description(&self) -> String {
        format!("{}, armor: {}, block: {}ms", self.name, self.armor, self.block)
    }
}

// Compiling time polymorphism /static dispatch / static polymorphism 
fn vendor_text_static<T: Sellable>(item: &T) -> String {
    format!("I offer you: {}, price: [{}g]", item.description(), item.price())
}



// 2nd part Trait object / work when need use a vector / dynamic dispatch / work in runtime / more effective 
fn vendor_text_dynamic(item: &dyn Sellable) -> String {
    format!("I offer you: {}, price: [{}g]", item.description(), item.price())
}


fn main() {
    let sword = Sword {
        name: "Sword of Cowardice".into(),
        damage: 10,
        swing_time_ms: 1500,
    };

    let shield = Shield {
        name: "Golden Barrier".into(),
        armor: 50,
        block: 35,
    };

    println!("{}", vendor_text_static(&sword));
    println!("{}", vendor_text_static(&shield));

    /*
    let sellables = vec![&sword, &shield];
    */
    let sellables: Vec<&dyn Sellable> = vec![&sword, &shield];

    for s in sellables{
        println!("{}", vendor_text_dynamic(s));
        /*
        this fail , because in compiler time doesnt know the Sized
        println!("{}", vendor_text_static(s));
        solution: Use box
        */
    }

    let owned_sellables: Vec<Box<dyn Sellable>> = vec![
        Box::new(
            Sword { name: "Blade on the heap".into(), damage:10, swing_time_ms:800}), 
        Box::new( 
            Shield { name: "Shield on dynamic memory".into(), armor: 20, block: 20})];

    for s in owned_sellables {
        println!("{}", vendor_text_dynamic(s.as_ref()));
    }
}
